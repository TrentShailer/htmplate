//! Build script for the library

use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn main() {
    println!("cargo::rerun-if-changed=assets/styles/");
    println!("cargo::rerun-if-changed=assets/scripts/");

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let assets_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("assets");

    // Bundle CSS
    {
        let mut child = Command::new("lightningcss")
            .arg("--bundle")
            .arg("--minify")
            .arg("--output-file")
            .arg(out_dir.join("style.min.css"))
            .arg(assets_root.join("styles").join("lib.css"))
            .spawn()
            .unwrap();

        let status = child.wait().unwrap();
        assert!(status.success());
    }

    // Bundle TS
    {
        let scripts_dir = assets_root.join("scripts");
        for entry in fs::read_dir(scripts_dir).unwrap() {
            let Ok(metadata) = entry else {
                continue;
            };
            if metadata
                .path()
                .extension()
                .is_none_or(|extension| extension != "ts")
            {
                continue;
            }

            fs::copy(metadata.path(), out_dir.join(metadata.file_name())).unwrap();
        }
    }
}
