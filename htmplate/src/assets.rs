//! Static assets for the components.

use std::{fs, io, path::Path};

/// The bundled and minified CSS for this version.
pub const CSS: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.min.css"));
/// The bundled and minified JS for this version.
pub const JS: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.js"));
/// The declaration file for the JS.
pub const DECLARATION: &str = include_str!(concat!(env!("OUT_DIR"), "/lib.d.ts"));

/// The favicon.
pub const FAVICON: &[u8] = include_bytes!("assets/favicon.ico");

/// Fira code font.
pub const FIRA_CODE: &[u8] = include_bytes!("assets/styles/fonts/FiraCode.ttf");
/// Fira code license
pub const FIRA_CODE_LICENSE: &[u8] = include_bytes!("assets/styles/fonts/FiraCode-OFL.txt");

/// Nunito font.
pub const NUNITO: &[u8] = include_bytes!("assets/styles/fonts/Nunito.ttf");
/// Nunito license
pub const NUNITO_LICENSE: &[u8] = include_bytes!("assets/styles/fonts/Nunito-OFL.txt");

/// Quicksand font.
pub const QUICKSAND: &[u8] = include_bytes!("assets/styles/fonts/Quicksand.ttf");
/// Quicksand license
pub const QUICKSAND_LICENSE: &[u8] = include_bytes!("assets/styles/fonts/Quicksand-OFL.txt");

/// Write the assets to a given directory.
pub fn write_assets(directory: &Path) -> io::Result<()> {
    fs::create_dir_all(directory)?;
    fs::remove_dir_all(directory)?;

    // Create directories
    fs::create_dir_all(directory)?;
    fs::create_dir_all(directory.join("fonts"))?;

    // Write style
    fs::write(directory.join("lib.min.css"), CSS)?;

    // Write scripts
    fs::write(directory.join("lib.js"), JS)?;
    fs::write(directory.join("lib.d.ts"), DECLARATION)?;

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
