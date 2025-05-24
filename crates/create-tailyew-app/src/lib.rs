// crates/create-tailyew-app/src/lib.rs

use std::{
    ffi::OsStr,
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::Confirm;
use include_dir::{include_dir, Dir, DirEntry};

/// Embed everything under `<crate-root>/src/starter/app`
static APP_DIR: Dir<'_> = include_dir!("src/starter/app");

/// CLI args
#[derive(Debug, Parser)]
#[command(
    name = "create-tailyew-app",
    version = "0.1.0",
    about   = "Scaffold a new Tailyew Yew+Tailwind SPA"
)]
pub struct Config {
    /// Name of your new project (this becomes the folder name)
    #[arg(value_name = "NAME")]
    pub name: String,

    /// Optional base directory (defaults to current dir)
    #[arg(long)]
    pub dest: Option<String>,
}

pub fn scaffold_frontend(config: Config) -> Result<()> {
    // compute output root = {dest base}/{name}
    let base = config.dest.as_deref().unwrap_or(".");
    let out_root = Path::new(base).join(&config.name);

    // make sure it exists
    fs::create_dir_all(&out_root)
        .with_context(|| format!("creating project dir `{}`", out_root.display()))?;

    // 1) extract the embedded `app/` subtree
    copy_dir_recursive(&APP_DIR, &out_root)?;

    // 1a) rename placeholder manifests ("--Cargo.toml") → "Cargo.toml"
    fix_manifest_placeholders(&out_root)?;

    // 2) run `npm install` in `<dest>/<name>/frontend`
    let frontend_dir = out_root.join("frontend");
    Command::new("npm")
        .arg("install")
        .current_dir(&frontend_dir)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("`npm install` failed")?;

    // 3) ensure `cargo-watch` is installed (for hot-reload)
    if Command::new("cargo")
        .args(["watch", "--version"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| !s.success())
        .unwrap_or(true)
    {
        println!("⏳ Installing cargo-watch for hot reloading…");
        Command::new("cargo")
            .args(["install", "cargo-watch"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context("failed to install cargo-watch")?;
    }

    // 4) optionally start the dev server via Makefile
    if Confirm::new()
        .with_prompt("Start the dev server now?")
        .default(true)
        .interact()?
    {
        Command::new("make")
            .arg("run-frontend")
            .current_dir(&out_root)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .context("`make run-frontend` failed")?;
    }

    Ok(())
}

/// Rename every `--Cargo.toml` to `Cargo.toml` under `root`
fn fix_manifest_placeholders(root: &Path) -> Result<()> {
    for entry in fs::read_dir(root).with_context(|| format!("reading dir `{}`", root.display()))? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            fix_manifest_placeholders(&path)?;
        } else if path.file_name() == Some(OsStr::new("--Cargo.toml")) {
            let new_path = path.with_file_name("Cargo.toml");
            fs::rename(&path, &new_path).with_context(|| {
                format!("renaming `{}` → `{}`", path.display(), new_path.display())
            })?;
        }
    }
    Ok(())
}

/// Recursively copy an `include_dir::Dir` into `dest`
fn copy_dir_recursive(dir: &Dir, dest: &Path) -> Result<()> {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(sub) => {
                let sub_path = dest.join(sub.path());
                fs::create_dir_all(&sub_path)
                    .with_context(|| format!("creating directory `{}`", sub_path.display()))?;
                copy_dir_recursive(sub, dest)?;
            }
            DirEntry::File(file) => {
                let out_path = dest.join(file.path());
                if let Some(parent) = out_path.parent() {
                    fs::create_dir_all(parent)
                        .with_context(|| format!("creating directory `{}`", parent.display()))?;
                }
                fs::write(&out_path, file.contents())
                    .with_context(|| format!("writing file `{}`", out_path.display()))?;
            }
        }
    }
    Ok(())
}
