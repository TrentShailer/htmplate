//! Build script for the library

use std::{path::Path, process::Command};

fn main() {
    println!("cargo::rerun-if-changed=assets/styles/");
    println!("cargo::rerun-if-changed=assets/scripts/");

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let assets_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("assets");

    // Bundle CSS
    {
        let mut child = Command::new("lightningcss")
            .arg("--bundle")
            .arg("--minify")
            .arg("--output-file")
            .arg(Path::new(&out_dir).join("lib.min.css"))
            .arg(assets_root.join("styles").join("lib.css"))
            .spawn()
            .unwrap();

        let status = child.wait().unwrap();
        assert!(status.success());
    }

    // Bundle JS
    {
        let mut child = Command::new("deno")
            .arg("run")
            .arg("-A")
            .arg(assets_root.join("scripts").join("build.ts"))
            .arg("--outdir")
            .arg(Path::new(&out_dir))
            .arg("--entryPoint")
            .arg(assets_root.join("scripts").join("lib.ts"))
            .spawn()
            .unwrap();

        let status = child.wait().unwrap();
        assert!(status.success());
    }
}
