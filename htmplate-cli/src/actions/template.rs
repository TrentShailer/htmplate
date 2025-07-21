use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::Stdio,
};

use htmplate::{ReplaceHtmplateError, replace_htmplates};

use crate::actions::file_exists_and_is_accessable;

pub fn template_html(source: &Path, target: &Path) -> Result<(), TemplateError> {
    if !file_exists_and_is_accessable(source)
        .map_err(|source| TemplateError::ReadSourceMetadata { source })?
    {
        return Err(TemplateError::CannotAccessSource);
    }

    let source_html =
        fs::read_to_string(source).map_err(|source| TemplateError::ReadSource { source })?;
    let html = replace_htmplates(&source_html, source)
        .map_err(|source| TemplateError::Template { source })?;

    // Ensure deno exists.
    if std::process::Command::new("deno")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_err()
    {
        fs::write(target, html).map_err(|source| TemplateError::WriteOutput { source })?;
        return Err(TemplateError::NoFormatter);
    }

    let mut child = std::process::Command::new("deno")
        .arg("fmt")
        .arg("--ext")
        .arg("html")
        .arg("-")
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|source| TemplateError::SpawnFormatter { source })?;

    let mut stdin = child.stdin.take().unwrap();
    stdin
        .write_all(html.as_bytes())
        .map_err(|source| TemplateError::WriteOutput { source })?;
    drop(stdin);

    let status = child
        .wait()
        .map_err(|source| TemplateError::SpawnFormatter { source })?;
    if !status.success() {
        fs::write(target, html).map_err(|source| TemplateError::WriteOutput { source })?;
        let stderr = io::read_to_string(child.stderr.take().unwrap())
            .map_err(|source| TemplateError::ReadFormatterOutput { source })?;
        return Err(TemplateError::FormatterError {
            stderr,
            status: status.code().unwrap_or(i32::MIN),
        });
    }

    let stdout = io::read_to_string(child.stdout.take().unwrap())
        .map_err(|source| TemplateError::ReadFormatterOutput { source })?;
    fs::write(target, stdout).map_err(|source| TemplateError::WriteOutput { source })?;

    Ok(())
}

/// Error variants for templating HTML.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TemplateError {
    #[non_exhaustive]
    ReadSourceMetadata { source: io::Error },

    #[non_exhaustive]
    CannotAccessSource,

    #[non_exhaustive]
    NoFormatter,

    #[non_exhaustive]
    SpawnFormatter { source: io::Error },

    #[non_exhaustive]
    FormatterError { stderr: String, status: i32 },

    #[non_exhaustive]
    WriteOutput { source: io::Error },

    #[non_exhaustive]
    ReadFormatterOutput { source: io::Error },

    #[non_exhaustive]
    ReadSource { source: io::Error },

    #[non_exhaustive]
    Template { source: ReplaceHtmplateError },
}
impl core::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::ReadSourceMetadata { .. } => write!(f, "could not read source file metadata"),
            Self::CannotAccessSource { .. } => {
                write!(f, "source file does not exist or is inaccessable")
            }
            Self::NoFormatter { .. } => write!(f, "the system does not have the formatter `deno`"),
            Self::SpawnFormatter { .. } => write!(f, "could not spawn the formatter process"),
            Self::FormatterError { stderr, status, .. } => write!(
                f,
                "the formatter process exited with code {status}: {stderr}"
            ),
            Self::WriteOutput { .. } => write!(f, "could not write output"),
            Self::ReadFormatterOutput { .. } => write!(f, "could not read formatter output"),
            Self::ReadSource { .. } => write!(f, "could not read source file"),
            Self::Template { .. } => write!(f, "failed templating source file"),
        }
    }
}
impl core::error::Error for TemplateError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::ReadSourceMetadata { source, .. } => Some(source),
            Self::SpawnFormatter { source, .. } => Some(source),
            Self::WriteOutput { source, .. } => Some(source),
            Self::ReadFormatterOutput { source, .. } => Some(source),
            Self::ReadSource { source, .. } => Some(source),
            Self::Template { source, .. } => Some(source),
            _ => None,
        }
    }
}
