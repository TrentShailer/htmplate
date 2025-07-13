use std::{
    fs,
    io::{self, Read, Write},
    path::{Path, PathBuf},
    process::Stdio,
};

use htmplate::{ReplaceHtmplateError, assets, replace_htmplates};
use pathdiff::diff_paths;

pub fn template_file(
    source: &Path,
    output: &Path,
    asset_directory: Option<&Path>,
) -> Result<(), TemplateError> {
    let source_file = File::Source(source.to_path_buf());
    let output_file = File::Output(output.to_path_buf());

    // Validate source
    {
        if !fs::exists(source)
            .map_err(|source| TemplateError::read_metadata(source, &source_file))?
        {
            return Err(TemplateError::read_metadata(
                io::Error::from(io::ErrorKind::NotFound),
                &source_file,
            ));
        }

        let metadata = source
            .metadata()
            .map_err(|source| TemplateError::read_metadata(source, &source_file))?;

        if !metadata.is_file() {
            return Err(TemplateError::is_not_a_file(&source_file));
        }
    }

    // Validate output
    if fs::exists(output).map_err(|source| TemplateError::read_metadata(source, &output_file))? {
        let metadata = output
            .metadata()
            .map_err(|source| TemplateError::read_metadata(source, &output_file))?;

        if !metadata.is_file() {
            return Err(TemplateError::is_not_a_file(&output_file));
        }
    }

    // Get asset directory
    let output_directory = output.parent().unwrap();
    let asset_directory = asset_directory
        .map(Path::to_path_buf)
        .unwrap_or_else(|| output_directory.join("lib"));
    let asset_file = File::Assets(asset_directory.clone());
    let path_from_output_to_assets = diff_paths(&asset_directory, output_directory).unwrap();

    // Get source contents
    let html = fs::read_to_string(source)
        .map_err(|source| TemplateError::read_file(source, &source_file))?;

    // Template the HTML
    let templated = replace_htmplates(&html, source, &path_from_output_to_assets)
        .map_err(|source| TemplateError::TemplateHtml { source })?;

    // Write the templated HTML to file
    write_html(&templated, output)
        .map_err(|source| TemplateError::write_file(source, &output_file))?;

    // Write the assets out
    assets::write_assets(&asset_directory)
        .map_err(|source| TemplateError::write_file(source, &asset_file))?;

    Ok(())
}

fn write_html(html: &str, output: &Path) -> io::Result<()> {
    // Ensure deno exists.
    if std::process::Command::new("deno")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_err()
    {
        fs::write(output, html)?;
        return Ok(());
    }

    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output)?;

    let mut child = std::process::Command::new("deno")
        .arg("fmt")
        .arg("--ext")
        .arg("html")
        .arg("-")
        .stdout(file)
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stdin = child
        .stdin
        .take()
        .expect("Failed to take stdin on the printer process");
    stdin.write_all(html.as_bytes())?;
    drop(stdin);

    let status = child.wait()?;

    if !status.success() {
        let mut stderr = child
            .stderr
            .take()
            .expect("Failed to take stderr on the printer process");
        let mut stderr_buffer = String::new();
        stderr.read_to_string(&mut stderr_buffer)?;

        fs::write(output, html)?;
        return Ok(());
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub enum File {
    Source(PathBuf),
    Output(PathBuf),
    Assets(PathBuf),
}
impl core::fmt::Display for File {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::Source(path_buf) => write!(f, "source file `{}`", path_buf.to_string_lossy()),
            Self::Output(path_buf) => write!(f, "output file `{}`", path_buf.to_string_lossy()),
            Self::Assets(path_buf) => write!(f, "assets `{}`", path_buf.to_string_lossy()),
        }
    }
}

/// Error variants for templating some HTML.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum TemplateError {
    #[non_exhaustive]
    IsNotAFile { file: File },

    #[non_exhaustive]
    ReadMetadata { source: io::Error, file: File },

    #[non_exhaustive]
    ReadFile { source: io::Error, file: File },

    #[non_exhaustive]
    TemplateHtml { source: ReplaceHtmplateError },

    #[non_exhaustive]
    WriteFile { source: io::Error, file: File },
}
impl core::fmt::Display for TemplateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::IsNotAFile { file, .. } => write!(f, "{file} is not a file"),
            Self::ReadMetadata { file, .. } => write!(f, "could not read the metadata for {file}",),
            Self::ReadFile { file, .. } => write!(f, "could not read {file}"),
            Self::TemplateHtml { .. } => write!(f, "could not template source file"),
            Self::WriteFile { file, .. } => write!(f, "could not write {file}"),
        }
    }
}
impl core::error::Error for TemplateError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::ReadMetadata { source, .. } => Some(source),
            Self::ReadFile { source, .. } => Some(source),
            Self::TemplateHtml { source, .. } => Some(source),
            Self::WriteFile { source, .. } => Some(source),
            _ => None,
        }
    }
}
impl TemplateError {
    #[allow(missing_docs)]
    pub fn is_not_a_file(source: &File) -> Self {
        Self::IsNotAFile {
            file: source.clone(),
        }
    }

    #[allow(missing_docs)]
    pub fn read_metadata(source: io::Error, file: &File) -> Self {
        Self::ReadMetadata {
            source,
            file: file.clone(),
        }
    }

    #[allow(missing_docs)]
    pub fn read_file(source: io::Error, file: &File) -> Self {
        Self::ReadFile {
            source,
            file: file.clone(),
        }
    }

    #[allow(missing_docs)]
    pub fn write_file(source: io::Error, file: &File) -> Self {
        Self::WriteFile {
            source,
            file: file.clone(),
        }
    }
}
