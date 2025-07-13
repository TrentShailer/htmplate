use std::{io, path::Path};

use htmplate::assets::write_assets;

use crate::cli::Command;

impl Command {
    pub fn write_assets(directory: &Path) -> io::Result<()> {
        write_assets(directory)
    }
}
