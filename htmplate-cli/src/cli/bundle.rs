use std::path::PathBuf;

use argh::FromArgs;

use crate::{actions::bundle_script, cli::CommandError};

/// Bundle the TypeScript.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "bundle")]
pub struct BundleSubcommand {
    /// the path to the TypeScript file to bundle
    #[argh(positional)]
    source: PathBuf,
}

impl BundleSubcommand {
    pub fn bundle(&self) -> Result<(), CommandError> {
        bundle_script(&self.source).map_err(|source| CommandError::Bundle { source })
    }
}
