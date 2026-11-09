//! The names of this repository's own libraries (libs/<Name>/), so the
//! pages can credit their functions to this repository: the shell
//! credits every library it reaches through MICROSCOPE_LIBS to
//! X_eTaL-libraries (docs/xetal-ML-asks.md, GM7).

use std::path::Path;

fn main() {
    let libs = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../libs");
    println!("cargo:rerun-if-changed={}", libs.display());
    let mut names: Vec<String> = std::fs::read_dir(&libs)
        .map(|d| d.filter_map(Result::ok).map(|e| e.path()).filter(|p| p.join("src").is_dir()).filter_map(|p| p.file_name().and_then(|n| n.to_str()).map(str::to_string)).collect())
        .unwrap_or_default();
    names.sort();
    let code = format!("pub const OWN_LIBRARIES: &[&str] = &{names:?};\n");
    std::fs::write(Path::new(&std::env::var("OUT_DIR").unwrap()).join("own.rs"), code).unwrap();
}
