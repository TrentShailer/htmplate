use std::{io, path::Path};

use htmplate::assets::write_assets;

pub fn write_library(directory: &Path) -> Result<(), WriteLibraryError> {
    write_assets(directory).map_err(|source| WriteLibraryError::WriteAssets { source })
}

/// Error variants for writing the library files.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum WriteLibraryError {
    #[non_exhaustive]
    WriteAssets { source: io::Error },
}
impl core::fmt::Display for WriteLibraryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::WriteAssets { .. } => write!(f, "could not write assets"),
        }
    }
}
impl core::error::Error for WriteLibraryError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::WriteAssets { source, .. } => Some(source),
        }
    }
}
