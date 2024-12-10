use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

type DynError = Box<dyn std::error::Error>;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(-1);
    }
}

fn try_main() -> Result<(), DynError> {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("release") => release()?,
        _ => print_help(),
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "Tasks:

release         runs 'cargo release' after preparing the source directory
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

fn release() -> Result<(), DynError> {
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .args(["release", "--workspace", "minor", "-x", "--no-publish"])
        .status()?;

    if !status.success() {
        Err("cargo release failed")?;
    }

    // For packaged build: rename libcubeb Cargo.toml files to Cargo.toml.in.
    visit_dirs(Path::new("cubeb-sys/libcubeb"), &|entry| {
        let path = entry.path();
        if path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with("Cargo.toml")
        {
            let new_path = path.with_file_name("Cargo.toml.in");
            fs::rename(&path, &new_path).unwrap();
        }
    })
    .unwrap();

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .args(["publish", "--package", "cubeb-sys", "--allow-dirty"])
        .status()?;
    if !status.success() {
        Err("cargo publish failed")?;
    }

    // Rename libcubeb Cargo.toml.in files back to Cargo.toml.
    visit_dirs(Path::new("cubeb-sys/libcubeb"), &|entry| {
        let path = entry.path();
        if path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with("Cargo.toml.in")
        {
            let new_path = path.with_file_name("Cargo.toml");
            fs::rename(&path, &new_path).unwrap();
        }
    })
    .unwrap();

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .args(["publish", "--package", "cubeb-core"])
        .status()?;
    if !status.success() {
        Err("cargo publish failed")?;
    }

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .args(["publish", "--package", "cubeb-backend"])
        .status()?;
    if !status.success() {
        Err("cargo publish failed")?;
    }

    let status = Command::new(&cargo)
        .current_dir(project_root())
        .args(["publish", "--package", "cubeb"])
        .status()?;
    if !status.success() {
        Err("cargo publish failed")?;
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
