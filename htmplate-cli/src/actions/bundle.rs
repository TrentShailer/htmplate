use std::{
    io::{self},
    path::Path,
    process::Stdio,
};

use crate::actions::file_exists_and_is_accessable;

pub fn bundle_script(source: &Path) -> Result<(), BundleScriptError> {
    // Ensure deno exists.
    if std::process::Command::new("deno")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_err()
    {
        return Err(BundleScriptError::NoBundler);
    }

    if !file_exists_and_is_accessable(source)
        .map_err(|source| BundleScriptError::ReadSourceMetadata { source })?
    {
        return Err(BundleScriptError::CannotAccessSource);
    }

    let mut child = std::process::Command::new("deno")
        .arg("bundle")
        .arg("--platform")
        .arg("browser")
        .arg("--minify")
        .arg("--sourcemap")
        .arg("--outdir")
        .arg(source.parent().unwrap())
        .arg(source)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| BundleScriptError::SpawnBundlerProcess { source })?;

    let status = child
        .wait()
        .map_err(|source| BundleScriptError::SpawnBundlerProcess { source })?;

    if !status.success() {
        let stderr = io::read_to_string(child.stderr.take().unwrap())
            .map_err(|source| BundleScriptError::ReadBundlerOutput { source })?;

        return Err(BundleScriptError::BundlerProcessError {
            stderr,
            status: status.code().unwrap_or(i32::MIN),
        });
    }

    Ok(())
}

/// Error variants for bundling a script.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum BundleScriptError {
    #[non_exhaustive]
    NoBundler,

    #[non_exhaustive]
    ReadSourceMetadata { source: io::Error },

    #[non_exhaustive]
    CannotAccessSource,

    #[non_exhaustive]
    SpawnBundlerProcess { source: io::Error },

    #[non_exhaustive]
    BundlerProcessError { stderr: String, status: i32 },

    #[non_exhaustive]
    ReadBundlerOutput { source: io::Error },
}
impl core::fmt::Display for BundleScriptError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::NoBundler { .. } => write!(f, "the system does not have the bundler `deno`"),
            Self::SpawnBundlerProcess { .. } => write!(f, "could not spawn bundler process"),
            Self::BundlerProcessError { stderr, status, .. } => {
                write!(f, "the bundler process exited with code {status}: {stderr}")
            }
            Self::ReadBundlerOutput { .. } => write!(f, "could not read the bundler output"),
            Self::ReadSourceMetadata { .. } => write!(f, "could not read source file metadata"),
            Self::CannotAccessSource { .. } => {
                write!(f, "source file does not exist or is inaccessable")
            }
        }
    }
}
impl core::error::Error for BundleScriptError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::SpawnBundlerProcess { source, .. } => Some(source),
            Self::ReadBundlerOutput { source, .. } => Some(source),
            Self::ReadSourceMetadata { source, .. } => Some(source),
            _ => None,
        }
    }
}
