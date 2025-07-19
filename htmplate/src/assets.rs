//! Static assets for the components.

use std::{fs, io, path::Path};

const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/style.min.css"));

const BASE64_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/base64.js"));
const BASE64_DECLARATION: &str = include_str!(concat!(env!("OUT_DIR"), "/base64.d.ts"));
const FETCH_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/fetch.js"));
const FETCH_DECLARATION: &str = include_str!(concat!(env!("OUT_DIR"), "/fetch.d.ts"));
const FORM_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/form.js"));
const FORM_DECLARATION: &str = include_str!(concat!(env!("OUT_DIR"), "/form.d.ts"));
const REDIRECT_JS: &str = include_str!(concat!(env!("OUT_DIR"), "/redirect.js"));
const REDIRECT_DECLARATION: &str = include_str!(concat!(env!("OUT_DIR"), "/redirect.d.ts"));

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
    fs::write(directory.join("base64.js"), BASE64_JS)?;
    fs::write(directory.join("base64.d.ts"), BASE64_DECLARATION)?;
    fs::write(directory.join("fetch.js"), FETCH_JS)?;
    fs::write(directory.join("fetch.d.ts"), FETCH_DECLARATION)?;
    fs::write(directory.join("form.js"), FORM_JS)?;
    fs::write(directory.join("form.d.ts"), FORM_DECLARATION)?;
    fs::write(directory.join("redirect.js"), REDIRECT_JS)?;
    fs::write(directory.join("redirect.d.ts"), REDIRECT_DECLARATION)?;

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
