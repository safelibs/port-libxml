use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct BuildManifest {
    version: u32,
    config_header: String,
    private_include_dir: String,
    public_include_dir: String,
    version_script: String,
    shared_object_version: String,
    link_libs: Vec<String>,
    private_headers: Vec<String>,
    #[serde(default)]
    module: Vec<ModuleEntry>,
}

#[derive(Debug, Deserialize)]
struct ModuleEntry {
    name: String,
    source: String,
    provider: String,
    #[serde(default = "default_enabled")]
    enabled: bool,
}

fn default_enabled() -> bool {
    true
}

const PRIVATE_SUPPORT_PREFIX: &str = "__safe_private_";
const PRIVATE_SUPPORT_MODULES: &[&str] = &[
    "xpath",
    "xpointer",
    "pattern",
    "xmlregexp",
    "valid",
    "c14n",
    "catalog",
    "xinclude",
    "xmlmodule",
    "xlink",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SymbolKind {
    Function,
    Object,
}

fn main() {
    if let Err(err) = run() {
        panic!("safe libxml2 build failed: {err}");
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let manifest_path = manifest_dir.join("build/modules.toml");
    let manifest: BuildManifest = toml::from_str(&fs::read_to_string(&manifest_path)?)?;
    if manifest.version != 1 {
        return Err(format!("unsupported build manifest version {}", manifest.version).into());
    }

    println!("cargo:rerun-if-changed={}", manifest_path.display());
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join(&manifest.version_script).display()
    );
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=AR");
    println!("cargo:rerun-if-env-changed=CFLAGS");
    println!("cargo:rerun-if-env-changed=NM");
    println!("cargo:rerun-if-env-changed=OBJCOPY");
    println!("cargo:rerun-if-changed={}", manifest_dir.join("shims").display());

    let cc = env::var_os("CC").unwrap_or_else(|| OsString::from("cc"));
    let ar = env::var_os("AR").unwrap_or_else(|| OsString::from("ar"));
    let nm = env::var_os("NM").unwrap_or_else(|| OsString::from("nm"));
    let objcopy = env::var_os("OBJCOPY").unwrap_or_else(|| OsString::from("objcopy"));
    let profile = env::var("PROFILE")?;
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    let native_dir = manifest_dir.join("target/native").join(&profile);
    let obj_dir = native_dir.join("obj");
    fs::create_dir_all(&obj_dir)?;

    let config_header = manifest_dir.join(&manifest.config_header);
    let private_include_dir = manifest_dir.join(&manifest.private_include_dir);
    let public_include_dir = manifest_dir.join(&manifest.public_include_dir);
    let version_script = manifest_dir.join(&manifest.version_script);

    println!("cargo:rerun-if-changed={}", config_header.display());
    println!("cargo:rerun-if-changed={}", private_include_dir.display());
    println!("cargo:rerun-if-changed={}", public_include_dir.display());

    let export_symbols = parse_version_script_symbols(&version_script)?;
    for module_name in PRIVATE_SUPPORT_MODULES {
        fs::write(out_dir.join(format!("aliases_{module_name}.rs")), "")?;
    }

    for header in &manifest.private_headers {
        println!(
            "cargo:rerun-if-changed={}",
            private_include_dir.join(header).display()
        );
    }

    let default_cflags = vec![
        OsString::from("-g"),
        OsString::from("-O2"),
        OsString::from("-fPIC"),
        OsString::from("-fno-strict-aliasing"),
        OsString::from("-Wno-unused-parameter"),
    ];
    let env_cflags = env::var("CFLAGS")
        .ok()
        .map(|value| {
            value
                .split_whitespace()
                .map(OsString::from)
                .collect::<Vec<_>>()
        })
        .unwrap_or_else(|| default_cflags.clone());

    let mut objects = Vec::new();
    for module in &manifest.module {
        let source = manifest_dir.join(&module.source);
        println!("cargo:rerun-if-changed={}", source.display());
        if !module.enabled {
            continue;
        }

        let private_support = is_private_support_module(&module.name);
        if module.provider == "rust" && !private_support {
            continue;
        }

        let object = if private_support {
            obj_dir.join(format!("{}.private.raw.o", module.name))
        } else {
            obj_dir.join(format!("{}.o", module.name))
        };
        let mut command = Command::new(&cc);
        command.args(&env_cflags);
        command.arg("-DHAVE_CONFIG_H");
        command.arg(format!("-I{}", private_include_dir.display()));
        command.arg(format!("-I{}", public_include_dir.display()));
        command.arg(format!("-I{}", config_header.parent().unwrap().display()));
        command.arg("-c");
        command.arg(&source);
        command.arg("-o");
        command.arg(&object);
        run_command(&mut command, &format!("compile {}", source.display()))?;
        if private_support {
            let defined = parse_defined_symbols(&nm, &object)?;
            let map_path = obj_dir.join(format!("{}.private.syms", module.name));
            let private_object = obj_dir.join(format!("{}.private.o", module.name));
            fs::write(&map_path, build_private_symbol_map(&defined))?;

            let mut objcopy_cmd = Command::new(&objcopy);
            objcopy_cmd.arg("--redefine-syms");
            objcopy_cmd.arg(&map_path);
            objcopy_cmd.arg(&object);
            objcopy_cmd.arg(&private_object);
            run_command(
                &mut objcopy_cmd,
                &format!("rename private support symbols for {}", module.name),
            )?;

            write_alias_file(
                &out_dir,
                &module.name,
                &defined,
                &export_symbols,
            )?;
            objects.push(private_object);
        } else {
            objects.push(object);
        }
    }

    if objects.is_empty() {
        return Err("build/modules.toml did not enable any non-rust modules".into());
    }

    let support_archive = native_dir.join("libxml2_c_support.a");
    let _ = fs::remove_file(&support_archive);
    let mut ar_cmd = Command::new(&ar);
    ar_cmd.arg("crs");
    ar_cmd.arg(&support_archive);
    for object in &objects {
        ar_cmd.arg(object);
    }
    run_command(&mut ar_cmd, "archive libxml2_c_support.a")?;

    println!("cargo:rustc-link-search=native={}", native_dir.display());
    println!("cargo:rustc-link-lib=static:+whole-archive=xml2_c_support");
    println!("cargo:rustc-cdylib-link-arg=-Wl,--no-undefined");
    println!("cargo:rustc-cdylib-link-arg=-Wl,--undefined-version");
    println!("cargo:rustc-cdylib-link-arg=-Wl,-soname,libxml2.so.2");
    println!(
        "cargo:rustc-cdylib-link-arg=-Wl,--version-script={}",
        version_script.display()
    );
    for link_lib in &manifest.link_libs {
        println!("cargo:rustc-link-lib={link_lib}");
    }

    let triplet = detect_multiarch(&cc)?;
    let target_dir = manifest_dir.join("target").join(&profile);
    let metadata = format!(
        "PROFILE={profile}\nLIBXML2_NATIVE_DIR={native}\nLIBXML2_NATIVE_SHARED={shared}\nLIBXML2_NATIVE_STATIC={static_archive}\nLIBXML2_SUPPORT_ARCHIVE={support_archive}\nLIBXML2_SONAME=libxml2.so.2\nLIBXML2_VERSION={version}\nLIBXML2_TRIPLET={triplet}\n",
        native = target_dir.display(),
        shared = target_dir.join("libxml2.so").display(),
        static_archive = target_dir.join("libxml2.a").display(),
        support_archive = support_archive.display(),
        version = manifest.shared_object_version,
    );
    fs::write(manifest_dir.join("target/build-artifacts.env"), metadata)?;

    Ok(())
}

fn is_private_support_module(module_name: &str) -> bool {
    PRIVATE_SUPPORT_MODULES.iter().any(|name| *name == module_name)
}

fn parse_version_script_symbols(
    path: &PathBuf,
) -> Result<BTreeSet<String>, Box<dyn std::error::Error>> {
    let mut exports = BTreeSet::new();

    for raw_line in fs::read_to_string(path)?.lines() {
        let line = raw_line.trim();
        if line.is_empty()
            || line == "global:"
            || line.starts_with("LIBXML")
            || line.starts_with('}')
            || line.starts_with('#')
        {
            continue;
        }
        let symbol = line.split(';').next().unwrap().trim();
        if symbol.is_empty() {
            continue;
        }
        exports.insert(symbol.to_string());
    }

    Ok(exports)
}

fn parse_defined_symbols(
    nm: &OsString,
    object: &PathBuf,
) -> Result<BTreeMap<String, SymbolKind>, Box<dyn std::error::Error>> {
    let output = Command::new(nm)
        .arg("-g")
        .arg("--defined-only")
        .arg("--format=posix")
        .arg(object)
        .output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    if !output.status.success() {
        return Err(format!(
            "nm failed for {} with status {}.\nstdout:\n{}\nstderr:\n{}",
            object.display(),
            output.status,
            stdout,
            stderr
        )
        .into());
    }

    let mut symbols = BTreeMap::new();
    for line in stdout.lines() {
        let mut fields = line.split_whitespace();
        let Some(name) = fields.next() else {
            continue;
        };
        let Some(kind) = fields.next() else {
            continue;
        };
        let Some(symbol_kind) = map_symbol_kind(kind) else {
            continue;
        };
        symbols.insert(name.to_string(), symbol_kind);
    }
    Ok(symbols)
}

fn map_symbol_kind(kind: &str) -> Option<SymbolKind> {
    let first = kind.chars().next()?;
    match first {
        'T' | 't' | 'W' | 'w' => Some(SymbolKind::Function),
        'A' | 'B' | 'b' | 'C' | 'D' | 'd' | 'G' | 'g' | 'R' | 'r' | 'S' | 's' | 'V'
        | 'v' => Some(SymbolKind::Object),
        _ => None,
    }
}

fn build_private_symbol_map(defined: &BTreeMap<String, SymbolKind>) -> String {
    let mut map = String::new();
    for name in defined.keys() {
        map.push_str(name);
        map.push(' ');
        map.push_str(PRIVATE_SUPPORT_PREFIX);
        map.push_str(name);
        map.push('\n');
    }
    map
}

fn write_alias_file(
    out_dir: &PathBuf,
    module_name: &str,
    defined: &BTreeMap<String, SymbolKind>,
    exports: &BTreeSet<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = out_dir.join(format!("aliases_{module_name}.rs"));
    let official = defined
        .keys()
        .filter(|symbol| exports.contains(*symbol))
        .cloned()
        .collect::<BTreeSet<_>>();

    if official.is_empty() {
        fs::write(file, "")?;
        return Ok(());
    }

    let mut asm = String::new();
    for symbol in official {
        let Some(kind) = defined.get(&symbol) else {
            return Err(format!(
                "missing private support definition for symbol {symbol} in module {module_name}"
            )
            .into());
        };
        if *kind != SymbolKind::Function {
            continue;
        }
        asm.push_str("    .globl ");
        asm.push_str(&symbol);
        asm.push('\n');
        asm.push_str("    .type ");
        asm.push_str(&symbol);
        asm.push_str(", @function\n");
        asm.push_str(&symbol);
        asm.push_str(":\n");
        asm.push_str("        jmp ");
        asm.push_str(PRIVATE_SUPPORT_PREFIX);
        asm.push_str(&symbol);
        asm.push('\n');
        asm.push_str("    .size ");
        asm.push_str(&symbol);
        asm.push_str(", .-");
        asm.push_str(&symbol);
        asm.push('\n');
    }

    let content = if asm.is_empty() {
        String::new()
    } else {
        format!("core::arch::global_asm!(r#\"\n{asm}\"#);\n")
    };
    fs::write(file, content)?;
    Ok(())
}

fn detect_multiarch(cc: &OsString) -> Result<String, Box<dyn std::error::Error>> {
    let output = Command::new(cc).arg("-print-multiarch").output()?;
    if output.status.success() {
        let triplet = String::from_utf8(output.stdout)?.trim().to_string();
        if !triplet.is_empty() {
            return Ok(triplet);
        }
    }
    Ok(env::var("TARGET").unwrap_or_else(|_| String::from("unknown-target")))
}

fn run_command(command: &mut Command, what: &str) -> Result<(), Box<dyn std::error::Error>> {
    let output = command.output()?;
    ensure_success(output, what)
}

fn ensure_success(output: Output, what: &str) -> Result<(), Box<dyn std::error::Error>> {
    if output.status.success() {
        return Ok(());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    Err(format!(
        "{what} failed with status {}.\nstdout:\n{}\nstderr:\n{}",
        output.status, stdout, stderr
    )
    .into())
}
