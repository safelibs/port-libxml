use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Output};

#[derive(Debug, Default)]
struct BuildManifest {
    version: u32,
    config_header: String,
    private_include_dir: String,
    public_include_dir: String,
    version_script: String,
    shared_object_version: String,
    link_libs: Vec<String>,
    private_headers: Vec<String>,
    module: Vec<ModuleEntry>,
}

#[derive(Debug)]
struct ModuleEntry {
    name: String,
    source: String,
    provider: String,
    enabled: bool,
}

impl Default for ModuleEntry {
    fn default() -> Self {
        Self {
            name: String::new(),
            source: String::new(),
            provider: String::new(),
            enabled: true,
        }
    }
}

fn main() {
    if let Err(err) = run() {
        panic!("safe libxml2 build failed: {err}");
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let manifest_path = manifest_dir.join("build/modules.toml");
    let manifest = parse_build_manifest(&fs::read_to_string(&manifest_path)?)?;
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
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("shims").display()
    );

    let cc = env::var_os("CC").unwrap_or_else(|| OsString::from("cc"));
    let ar = env::var_os("AR").unwrap_or_else(|| OsString::from("ar"));
    let profile = env::var("PROFILE")?;

    let native_dir = manifest_dir.join("target/native").join(&profile);
    let obj_dir = native_dir.join("obj");
    fs::create_dir_all(&obj_dir)?;

    let config_header = manifest_dir.join(&manifest.config_header);
    let private_include_dir = manifest_dir.join(&manifest.private_include_dir);
    let public_include_dir = manifest_dir.join(&manifest.public_include_dir);
    println!("cargo:rerun-if-changed={}", config_header.display());
    println!("cargo:rerun-if-changed={}", private_include_dir.display());
    println!("cargo:rerun-if-changed={}", public_include_dir.display());

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

        if module.provider == "rust" {
            continue;
        }

        let object = obj_dir.join(format!("{}.o", module.name));
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
        objects.push(object);
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
    for link_lib in &manifest.link_libs {
        println!("cargo:rustc-link-lib={link_lib}");
    }

    let triplet = detect_multiarch(&cc)?;
    let target_dir = manifest_dir.join("target").join(&profile);
    let metadata = format!(
        "PROFILE={profile}\nLIBXML2_NATIVE_DIR={native}\nLIBXML2_NATIVE_STATIC={static_archive}\nLIBXML2_SUPPORT_ARCHIVE={support_archive}\nLIBXML2_SONAME=libxml2.so.2\nLIBXML2_VERSION={version}\nLIBXML2_TRIPLET={triplet}\n",
        native = target_dir.display(),
        static_archive = target_dir.join("libxml2.a").display(),
        support_archive = support_archive.display(),
        version = manifest.shared_object_version,
    );
    fs::write(manifest_dir.join("target/build-artifacts.env"), metadata)?;

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

fn parse_build_manifest(input: &str) -> Result<BuildManifest, Box<dyn std::error::Error>> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut manifest = BuildManifest::default();
    let mut current_module: Option<ModuleEntry> = None;
    let mut index = 0;

    while index < lines.len() {
        let line = lines[index].trim();
        index += 1;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if line == "[[module]]" {
            if let Some(module) = current_module.take() {
                validate_module(&module)?;
                manifest.module.push(module);
            }
            current_module = Some(ModuleEntry::default());
            continue;
        }

        let (key, initial_value) = line
            .split_once('=')
            .ok_or_else(|| format!("unsupported manifest line: {line}"))?;
        let mut value = initial_value.trim().to_string();
        if value.starts_with('[') && !value.contains(']') {
            while index < lines.len() {
                let continuation = lines[index].trim();
                index += 1;
                value.push(' ');
                value.push_str(continuation);
                if continuation.contains(']') {
                    break;
                }
            }
        }

        if let Some(module) = current_module.as_mut() {
            assign_module_field(module, key.trim(), &value)?;
        } else {
            assign_manifest_field(&mut manifest, key.trim(), &value)?;
        }
    }

    if let Some(module) = current_module.take() {
        validate_module(&module)?;
        manifest.module.push(module);
    }

    validate_manifest(&manifest)?;
    Ok(manifest)
}

fn assign_manifest_field(
    manifest: &mut BuildManifest,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        "version" => manifest.version = value.parse()?,
        "config_header" => manifest.config_header = parse_string(value)?,
        "private_include_dir" => manifest.private_include_dir = parse_string(value)?,
        "public_include_dir" => manifest.public_include_dir = parse_string(value)?,
        "version_script" => manifest.version_script = parse_string(value)?,
        "shared_object_version" => manifest.shared_object_version = parse_string(value)?,
        "link_libs" => manifest.link_libs = parse_string_array(value)?,
        "private_headers" => manifest.private_headers = parse_string_array(value)?,
        _ => return Err(format!("unsupported manifest field {:?}", key).into()),
    }
    Ok(())
}

fn assign_module_field(
    module: &mut ModuleEntry,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match key {
        "name" => module.name = parse_string(value)?,
        "source" => module.source = parse_string(value)?,
        "provider" => module.provider = parse_string(value)?,
        "enabled" => {
            module.enabled = match value.trim() {
                "true" => true,
                "false" => false,
                _ => return Err(format!("unsupported boolean value {:?}", value).into()),
            };
        }
        _ => return Err(format!("unsupported module field {:?}", key).into()),
    }
    Ok(())
}

fn parse_string(value: &str) -> Result<String, Box<dyn std::error::Error>> {
    let trimmed = value.trim();
    if trimmed.len() < 2 || !trimmed.starts_with('"') || !trimmed.ends_with('"') {
        return Err(format!("expected quoted string, found {:?}", trimmed).into());
    }
    Ok(trimmed[1..trimmed.len() - 1].to_string())
}

fn parse_string_array(value: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let trimmed = value.trim();
    if !trimmed.starts_with('[') || !trimmed.ends_with(']') {
        return Err(format!("expected TOML string array, found {:?}", trimmed).into());
    }
    let inner = trimmed[1..trimmed.len() - 1].trim();
    if inner.is_empty() {
        return Ok(Vec::new());
    }
    inner
        .split(',')
        .filter_map(|item| {
            let trimmed = item.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(parse_string(trimmed))
            }
        })
        .collect::<Result<Vec<_>, _>>()
}

fn validate_manifest(manifest: &BuildManifest) -> Result<(), Box<dyn std::error::Error>> {
    if manifest.version == 0 {
        return Err("manifest version is required".into());
    }
    for (field_name, value) in [
        ("config_header", &manifest.config_header),
        ("private_include_dir", &manifest.private_include_dir),
        ("public_include_dir", &manifest.public_include_dir),
        ("version_script", &manifest.version_script),
        ("shared_object_version", &manifest.shared_object_version),
    ] {
        if value.is_empty() {
            return Err(format!("manifest field {:?} is required", field_name).into());
        }
    }
    Ok(())
}

fn validate_module(module: &ModuleEntry) -> Result<(), Box<dyn std::error::Error>> {
    for (field_name, value) in [
        ("name", &module.name),
        ("source", &module.source),
        ("provider", &module.provider),
    ] {
        if value.is_empty() {
            return Err(format!("module field {:?} is required", field_name).into());
        }
    }
    Ok(())
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
