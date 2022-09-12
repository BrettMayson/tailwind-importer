use std::{io::Write, path::Path};

use walkdir::WalkDir;

/// Call this function in the `build.rs` of any component libraries
pub fn build_library() {
    let out_dir = std::env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("all.rs");
    let mut file = std::fs::File::create(&dest_path).unwrap();
    for entry in WalkDir::new("./src").into_iter().filter_map(|e| e.ok()) {
        if entry.metadata().unwrap().is_file() {
            let source = std::fs::read_to_string(entry.path()).unwrap();
            file.write_all(source.as_bytes()).unwrap();
        }
    }
}

/// Call this function in the `build.rs` of the main Yew application
pub fn build_frontend() {
    let target = std::env::var_os("CARGO_TARGET_DIR").unwrap();
    std::process::Command::new("tailwindcss")
        .arg("--content")
        .arg(format!(
            "{}/**/all.rs,./src/**/*.{{html,rs}},./index.html",
            target.to_str().unwrap(),
        ))
        .arg("-o")
        .arg("./tailwind.css")
        .arg("--minify")
        .output()
        .expect("failed to execute process");

    println!("cargo:rerun-if-changed=./src");
}
