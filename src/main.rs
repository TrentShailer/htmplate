//! # `htmplate` CLI
//!

use core::time::Duration;
use std::{
    fs,
    io::{self, Write, stdout},
    path::{Path, PathBuf},
    process::Stdio,
    sync::mpsc::channel,
    time::Instant,
};

use clap::{Parser, Subcommand};
use htmplate::replace_htmplates;
use lol_html::errors::RewritingError;
use notify::{Config, Event, RecursiveMode, Watcher};
use notify_debouncer_full::{DebounceEventResult, new_debouncer};
use ts_cli_helper::print_success;
use ts_rust_helper::error::ReportResult;

/// Replace the `<htmplate:... />` elements in an HTML file with their contents.
#[derive(Debug, Parser)]
pub struct Cli {
    /// Subcommand
    #[clap(subcommand)]
    pub command: Command,
}

/// Subcommand
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Watch a file and template it whenever it is modified.
    Watch {
        /// The path to the HTML file containing htmplates.
        source: PathBuf,

        /// The file to output the templated HTML to.
        output: Option<PathBuf>,
    },

    /// Template a file
    Template {
        /// The path to the HTML file containing htmplates.
        source: PathBuf,

        /// The file to output the templated HTML to.
        output: Option<PathBuf>,
    },
}

fn main() -> ReportResult<'static, ()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Watch { source, output } => {
            let metadata = source.metadata().map_err(CliError::read_source)?;
            if !metadata.is_file() {
                return Err(CliError::SourceIsNotAFile.into());
            }

            let (tx, rx) = channel::<DebounceEventResult>();

            let mut debouncer = new_debouncer(Duration::from_millis(150), None, tx)
                .map_err(CliError::watch_error)?;

            debouncer
                .watch(&source, RecursiveMode::NonRecursive)
                .map_err(CliError::watch_error)?;

            eprintln!(
                "\x1bcWatching `{}` press `Ctrl + C` to exit",
                source.to_string_lossy()
            );

            for result in rx {
                if let Err(mut errors) = result {
                    let first = errors.pop().unwrap();
                    return Err(CliError::watch_error(first).into());
                }

                eprintln!("\x1bc");

                template_file(&source, output.as_deref())?;

                eprintln!(
                    "Watching `{}` press `Ctrl + C` to exit",
                    source.to_string_lossy()
                );
            }
        }
        Command::Template { source, output } => {
            let metadata = source.metadata().map_err(CliError::read_source)?;
            if !metadata.is_file() {
                return Err(CliError::SourceIsNotAFile.into());
            }

            template_file(&source, output.as_deref())?;
        }
    }

    Ok(())
}

fn template_file(source: &Path, output: Option<&Path>) -> Result<(), CliError> {
    let html = fs::read_to_string(source).map_err(CliError::read_source)?;

    let start = Instant::now();
    let templated_html = replace_htmplates(&html).map_err(CliError::rewrite_error)?;
    let template_duration = start.elapsed();

    let sink = get_output(output)?;

    write_formatted_html(&templated_html, sink).map_err(CliError::write_output)?;

    eprintln!(
        "\nTemplated `{}` in {}µs",
        source.to_string_lossy(),
        template_duration.as_micros()
    );

    Ok(())
}

fn get_output(output_path: Option<&Path>) -> Result<Stdio, CliError> {
    let Some(output_path) = output_path else {
        return Ok(stdout().into());
    };

    if fs::exists(output_path).map_err(CliError::write_output)? {
        let metadata = output_path.metadata().map_err(CliError::write_output)?;
        if !metadata.is_file() {
            return Err(CliError::OutputIsNotAFile);
        }
    }

    let file = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_path)
        .map_err(CliError::write_output)?;

    Ok(file.into())
}

fn write_formatted_html<W: Into<Stdio>>(html: &str, out: W) -> io::Result<()> {
    // If deno does not exist, write to output in-place.
    let mut child = if std::process::Command::new("deno")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_err()
    {
        std::process::Command::new("echo")
            .stdout(out)
            .stdin(Stdio::piped())
            .spawn()?
    } else {
        std::process::Command::new("deno")
            .arg("fmt")
            .arg("--ext")
            .arg("html")
            .arg("-")
            .stdout(out)
            .stdin(Stdio::piped())
            .spawn()?
    };

    let mut stdin = child
        .stdin
        .take()
        .expect("Failed to take stdin on the printer process");

    stdin.write_all(html.as_bytes())?;
    drop(stdin);

    let status = child.wait()?;

    if !status.success() {
        return Err(io::Error::other(format!("Exit code {status}")));
    }

    Ok(())
}

/// Error variants for the CLI.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CliError {
    #[non_exhaustive]
    SourceIsNotAFile,

    #[non_exhaustive]
    OutputIsNotAFile,

    #[non_exhaustive]
    ReadSource { source: io::Error },

    #[non_exhaustive]
    WriteOutput { source: io::Error },

    #[non_exhaustive]
    RewriteError { source: RewritingError },

    #[non_exhaustive]
    WatchError { source: notify::Error },
}
impl core::fmt::Display for CliError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::ReadSource { .. } => write!(f, "could not read source file"),
            Self::WriteOutput { .. } => write!(f, "could not write output"),
            Self::RewriteError { .. } => write!(f, "could not replace htmplates"),
            Self::WatchError { .. } => write!(f, "error while watching source file"),
            Self::SourceIsNotAFile { .. } => write!(f, "source is not a file"),
            Self::OutputIsNotAFile { .. } => write!(f, "output is not a file"),
        }
    }
}
impl core::error::Error for CliError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::ReadSource { source, .. } => Some(source),
            Self::WriteOutput { source, .. } => Some(source),
            Self::RewriteError { source, .. } => Some(source),
            Self::WatchError { source, .. } => Some(source),
            _ => None,
        }
    }
}
impl CliError {
    #[allow(missing_docs)]
    pub fn read_source(source: io::Error) -> Self {
        Self::ReadSource { source }
    }

    #[allow(missing_docs)]
    pub fn write_output(source: io::Error) -> Self {
        Self::WriteOutput { source }
    }

    #[allow(missing_docs)]
    pub fn rewrite_error(source: RewritingError) -> Self {
        Self::RewriteError { source }
    }

    #[allow(missing_docs)]
    pub fn watch_error(source: notify::Error) -> Self {
        Self::WatchError { source }
    }
}
