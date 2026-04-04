use std::env;
use std::ffi::OsString;
use std::fs;
use std::io;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
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

fn main() {
    if let Err(err) = run() {
        panic!("phase-1 native scaffold build failed: {err}");
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

    let cc = env::var_os("CC").unwrap_or_else(|| OsString::from("cc"));
    let ar = env::var_os("AR").unwrap_or_else(|| OsString::from("ar"));
    let profile = env::var("PROFILE")?;

    let native_dir = manifest_dir.join("target/native").join(&profile);
    let obj_dir = native_dir.join("obj");
    fs::create_dir_all(&obj_dir)?;

    let config_header = manifest_dir.join(&manifest.config_header);
    let private_include_dir = manifest_dir.join(&manifest.private_include_dir);
    let public_include_dir = manifest_dir.join(&manifest.public_include_dir);
    let version_script = manifest_dir.join(&manifest.version_script);

    println!("cargo:rerun-if-changed={}", config_header.display());
    println!(
        "cargo:rerun-if-changed={}",
        private_include_dir.display()
    );
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
        if !module.enabled || module.provider != "c_fallback" {
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
        return Err("build/modules.toml did not enable any c_fallback modules".into());
    }

    let static_archive = native_dir.join("libxml2.a");
    let mut ar_cmd = Command::new(&ar);
    ar_cmd.arg("crs");
    ar_cmd.arg(&static_archive);
    for object in &objects {
        ar_cmd.arg(object);
    }
    run_command(&mut ar_cmd, "archive libxml2.a")?;

    let version = &manifest.shared_object_version;
    let shared_name = format!("libxml2.so.{version}");
    let soname = "libxml2.so.2";
    let shared_path = native_dir.join(&shared_name);

    let mut link_cmd = Command::new(&cc);
    link_cmd.arg("-shared");
    link_cmd.arg("-Wl,--no-undefined");
    link_cmd.arg(format!("-Wl,-soname,{soname}"));
    link_cmd.arg(format!(
        "-Wl,--version-script={}",
        version_script.display()
    ));
    link_cmd.arg("-o");
    link_cmd.arg(&shared_path);
    for object in &objects {
        link_cmd.arg(object);
    }
    for link_lib in &manifest.link_libs {
        link_cmd.arg(format!("-l{link_lib}"));
    }
    run_command(&mut link_cmd, &format!("link {}", shared_path.display()))?;

    replace_symlink(&native_dir.join("libxml2.so.2"), Path::new(&shared_name))?;
    replace_symlink(&native_dir.join("libxml2.so"), Path::new("libxml2.so.2"))?;

    let triplet = detect_multiarch(&cc)?;
    let metadata = format!(
        "PROFILE={profile}\nLIBXML2_NATIVE_DIR={native}\nLIBXML2_NATIVE_SHARED={shared}\nLIBXML2_NATIVE_STATIC={static_archive}\nLIBXML2_SONAME={soname}\nLIBXML2_VERSION={version}\nLIBXML2_TRIPLET={triplet}\n",
        native = native_dir.display(),
        shared = shared_path.display(),
        static_archive = static_archive.display(),
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

fn replace_symlink(path: &Path, target: &Path) -> io::Result<()> {
    match fs::remove_file(path) {
        Ok(()) => {}
        Err(err) if err.kind() == io::ErrorKind::NotFound => {}
        Err(err) => return Err(err),
    }
    symlink(target, path)
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
