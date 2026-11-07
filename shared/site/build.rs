//! Build provenance for the footer: this repository's short commit and
//! the X_eTaL-libraries commit it pins (XETAL_LIBRARIES_COMMIT). The
//! X_eTaL-ML commit and the X_eTaL commit it pins come from microscope
//! (built from the pinned clone, work/xetal-ml), as do the host and
//! the time of the build.

use std::path::Path;
use std::process::Command;

fn run(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .ok()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}

/// The first seven characters of the commit a pin file holds.
fn pinned(file: &Path) -> String {
    println!("cargo:rerun-if-changed={}", file.display());
    match std::fs::read_to_string(file).unwrap_or_default().trim() {
        "" => "unknown".into(),
        sha => sha.chars().take(7).collect(),
    }
}

fn main() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    println!("cargo:rustc-env=SITE_SHA={}", run("git", &["-C", &root.display().to_string(), "rev-parse", "--short", "HEAD"]));
    println!("cargo:rustc-env=LIBRARIES_SHA={}", pinned(&root.join("XETAL_LIBRARIES_COMMIT")));
    println!("cargo:rerun-if-changed={}", root.join(".git/HEAD").display());
}
