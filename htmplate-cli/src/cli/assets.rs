use std::path::PathBuf;

use argh::FromArgs;

use crate::{actions::write_library, cli::CommandError};

/// Write the assets to a directory.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "assets")]
pub struct AssetsSubcommand {
    /// the directory to write the assets to
    #[argh(positional)]
    asset_directory: PathBuf,
}

impl AssetsSubcommand {
    pub fn write_assets(&self) -> Result<(), CommandError> {
        write_library(&self.asset_directory).map_err(|source| CommandError::Assets { source })
    }
}
