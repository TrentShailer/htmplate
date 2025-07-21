//! Static assets for the components.

use std::{fs, io, path::Path};

const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/style.min.css"));

const BASE64_TS: &str = include_str!(concat!(env!("OUT_DIR"), "/base64.ts"));
const FETCH_TS: &str = include_str!(concat!(env!("OUT_DIR"), "/fetch.ts"));
const FORM_TS: &str = include_str!(concat!(env!("OUT_DIR"), "/form.ts"));
const REDIRECT_TS: &str = include_str!(concat!(env!("OUT_DIR"), "/redirect.ts"));

const FAVICON: &[u8] = include_bytes!("assets/favicon.ico");

const FIRA_CODE: &[u8] = include_bytes!("assets/styles/fonts/FiraCode.ttf");
const FIRA_CODE_LICENSE: &[u8] = include_bytes!("assets/styles/fonts/FiraCode-OFL.txt");

const NUNITO: &[u8] = include_bytes!("assets/styles/fonts/Nunito.ttf");
const NUNITO_LICENSE: &[u8] = include_bytes!("assets/styles/fonts/Nunito-OFL.txt");

const QUICKSAND: &[u8] = include_bytes!("assets/styles/fonts/Quicksand.ttf");
const QUICKSAND_LICENSE: &[u8] = include_bytes!("assets/styles/fonts/Quicksand-OFL.txt");

/// Write the assets to a given directory.
pub fn write_assets(directory: &Path) -> io::Result<()> {
    fs::create_dir_all(directory)?;
    fs::remove_dir_all(directory)?;

    // Create directories
    fs::create_dir_all(directory)?;
    fs::create_dir_all(directory.join("fonts"))?;

    // Write style
    fs::write(directory.join("style.min.css"), CSS)?;

    // Write scripts
    fs::write(directory.join("base64.ts"), BASE64_TS)?;
    fs::write(directory.join("fetch.ts"), FETCH_TS)?;
    fs::write(directory.join("form.ts"), FORM_TS)?;
    fs::write(directory.join("redirect.ts"), REDIRECT_TS)?;

    // Write static
    fs::write(directory.join("favicon.ico"), FAVICON)?;

    // Write fonts
    {
        let directory = directory.join("fonts");
        fs::write(directory.join("FiraCode.ttf"), FIRA_CODE)?;
        fs::write(directory.join("FiraCode-OFL.txt"), FIRA_CODE_LICENSE)?;
        fs::write(directory.join("Nunito.ttf"), NUNITO)?;
        fs::write(directory.join("Nunito-OFL.txt"), NUNITO_LICENSE)?;
        fs::write(directory.join("Quicksand.ttf"), QUICKSAND)?;
        fs::write(directory.join("Quicksand-OFL.txt"), QUICKSAND_LICENSE)?;
    }

    // Write version marker
    fs::write(
        directory.join(format!("assets-v{}", env!("CARGO_PKG_VERSION"))),
        "",
    )?;

    Ok(())
}
