use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{e}");
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    let version = env::args().nth(2);
    match task.as_deref() {
        Some("release") => release(version.as_deref().unwrap_or("minor"))?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

release [VERSION]    runs 'cargo release [VERSION]' after preparing the source directory
                     [VERSION] can be 'major', 'minor', or 'patch'. If not specified, 'minor' is used.
"
    )
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&fs::DirEntry)) -> std::io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn package_version(manifest: &Path) -> Result<String, DynError> {
    let contents = fs::read_to_string(manifest)?;
    for line in contents.lines() {
        if let Some(rest) = line.strip_prefix("version = \"") {
            if let Some(end) = rest.find('"') {
                return Ok(rest[..end].to_string());
            }
        }
    }
    Err(format!("no version key found in {}", manifest.display()).into())
}

// For packaged build: rename libcubeb Cargo.toml files to Cargo.toml.in (or
// back again).
fn rename_libcubeb_manifests(from: &str, to: &str) -> Result<(), DynError> {
    visit_dirs(&project_root().join("cubeb-sys/libcubeb"), &|entry| {
        let path = entry.path();
        if path.file_name().unwrap().to_str().unwrap().ends_with(from) {
            let new_path = path.with_file_name(to);
            fs::rename(&path, &new_path).unwrap();
        }
    })?;
    Ok(())
}

fn verify_package_contents(cargo: &str) -> Result<(), DynError> {
    let output = Command::new(cargo)
        .current_dir(project_root())
        .args(["package", "--list", "--package", "cubeb-sys", "--allow-dirty"])
        .output()?;
    if !output.status.success() {
        Err("cargo package --list failed")?;
    }
    let list = String::from_utf8(output.stdout)?;
    let required = [
        "libcubeb/src/cubeb-coreaudio-rs/Cargo.toml.in",
        "libcubeb/src/cubeb-pulse-rs/Cargo.toml.in",
    ];
    for path in required {
        if !list.lines().any(|l| l.contains(path)) {
            Err(format!(
                "package is missing {path} — nested submodule not checked out"
            ))?;
        }
    }
    Ok(())
}

fn publish(cargo: &str, package: &str, allow_dirty: bool) -> Result<(), DynError> {
    let mut args = vec!["publish", "--package", package];
    if allow_dirty {
        args.push("--allow-dirty");
    }
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(args)
        .status()?;
    if !status.success() {
        Err(format!("cargo publish {package} failed"))?;
    }
    Ok(())
}

fn release(version: &str) -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let manifest = project_root().join("cubeb-sys/Cargo.toml");
    let version_before = package_version(&manifest)?;

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .args(["release", "--workspace", version, "-x", "--no-publish"])
        .status()?;

    if !status.success() {
        Err("cargo release failed")?;
    }

    // cargo release exits successfully when its confirmation prompt is
    // declined (or unanswerable without a tty), so check it actually bumped
    // the versions before publishing anything.
    if package_version(&manifest)? == version_before {
        Err("cargo release did not bump any versions (release declined?)")?;
    }

    let status = Command::new("git")
        .current_dir(project_root())
        .args(["submodule", "update", "--init", "--recursive"])
        .status()?;
    if !status.success() {
        Err("git submodule update failed")?;
    }

    verify_package_contents(&cargo)?;

    rename_libcubeb_manifests("Cargo.toml", "Cargo.toml.in")?;
    let result = publish(&cargo, "cubeb-sys", true);
    // Rename the manifests back even if the publish failed.
    rename_libcubeb_manifests("Cargo.toml.in", "Cargo.toml")?;
    result?;

    publish(&cargo, "cubeb-core", false)?;
    publish(&cargo, "cubeb-backend", false)?;
    publish(&cargo, "cubeb", false)?;

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
