//! # `htmplate` CLI
//!

use core::{error::Error, time::Duration};
use std::{
    fs,
    io::{self, Read, Write, stdout},
    path::{Path, PathBuf},
    process::Stdio,
    sync::mpsc::channel,
    time::Instant,
};

use clap::{Parser, Subcommand};
use htmplate::{
    FromElementError, all_htmplate_details, htmplates::HtmplateError, replace_htmplates,
};
use lol_html::errors::RewritingError;
use notify::{RecursiveMode, Watcher, recommended_watcher};
use ts_cli_helper::{print_fail, print_success, print_warning};
use ts_rust_helper::{
    error::ReportResult,
    style::{BOLD, CLEAR_TERMINAL, CYAN, ERASE_LINE_UP, RED, RESET},
};

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

    /// List the htmplates
    List {
        /// The htmplate to search for
        search: Option<String>,
    },
}

fn main() -> ReportResult<'static, ()> {
    let cli = Cli::parse();

    match cli.command {
        Command::List { search } => {
            let mut htmplates = all_htmplate_details();

            if let Some(search) = search {
                let search = search.to_lowercase();
                htmplates.retain(|htmplate| htmplate.tag.contains(&search));
            }

            let mut stdout = stdout().lock();

            for htmplate in htmplates {
                stdout.write_all(
                    format!(
                        "{BOLD}{CYAN}{}{RESET}  {}\n",
                        htmplate.tag.replace("\\", ""),
                        htmplate.description
                    )
                    .as_bytes(),
                )?;
                for attribute in htmplate.attributes {
                    stdout.write_all(b"  ")?;

                    if attribute.required {
                        stdout.write_all(format!("{BOLD}{RED}*{RESET}").as_bytes())?;
                    } else {
                        stdout.write_all(b" ")?;
                    }

                    stdout.write_all(
                        format!(
                            "{BOLD}[{}]:{RESET} {}\n",
                            attribute.name, attribute.description
                        )
                        .as_bytes(),
                    )?;
                }
            }
            stdout.flush()?;
        }
        Command::Watch { source, output } => {
            let metadata = source.metadata().map_err(CliError::read_source)?;
            if !metadata.is_file() {
                return Err(CliError::SourceIsNotAFile.into());
            }

            let (tx, rx) = channel();

            let mut watcher = recommended_watcher(tx).map_err(CliError::watch_error)?;
            watcher
                .watch(&source, RecursiveMode::NonRecursive)
                .map_err(CliError::watch_error)?;

            if output.is_none() {
                let mut stdout = stdout().lock();
                let _ = write!(stdout, "{CLEAR_TERMINAL}");
                let _ = stdout.flush();
            }
            println!(
                "Watching `{}` press `Ctrl + C` to exit",
                source.to_string_lossy()
            );

            let mut y_position = 1;
            let mut event_count = 1;
            let mut last_event = Instant::now();
            for result in rx {
                if last_event.elapsed() < Duration::from_millis(100) {
                    continue;
                }

                result.map_err(CliError::watch_error)?;

                if output.is_none() {
                    let mut stdout = stdout().lock();
                    let _ = write!(stdout, "{CLEAR_TERMINAL}");
                    let _ = stdout.flush();
                } else {
                    let mut stdout = stdout().lock();
                    for _ in 0..y_position {
                        let _ = write!(stdout, "{ERASE_LINE_UP}");
                    }
                    let _ = stdout.flush();
                }

                y_position = 0;
                let html = fs::read_to_string(&source).map_err(CliError::read_source)?;
                let lines_written = match template_html(&html, output.as_deref()) {
                    Ok(lines) => lines,
                    Err(mut error) => {
                        let CliError::RewriteError {
                            source: rewriting_error,
                        } = &mut error
                        else {
                            return Err(error.into());
                        };

                        let RewritingError::ContentHandlerError(any_error) = rewriting_error else {
                            return Err(error.into());
                        };

                        if let Some(error) = any_error.downcast_ref::<HtmplateError>() {
                            print_fail(&error.to_string(), 0);
                            1
                        } else if let Some(error) = any_error.downcast_mut::<FromElementError>() {
                            error.element_location = error
                                .element_location
                                .as_file_position(html.as_bytes(), source.clone());
                            let display = error.to_string();
                            print_fail(&display, 0);
                            display.lines().count() + 1
                        } else {
                            return Err(error.into());
                        }
                    }
                };
                y_position += lines_written;

                println!(
                    "Watching `{}` press `Ctrl + C` to exit ({})",
                    source.to_string_lossy(),
                    event_count
                );
                y_position += 1;
                last_event = Instant::now();
                event_count += 1;
            }
        }
        Command::Template { source, output } => {
            let metadata = source.metadata().map_err(CliError::read_source)?;
            if !metadata.is_file() {
                return Err(CliError::SourceIsNotAFile.into());
            }
            let html = fs::read_to_string(&source).map_err(CliError::read_source)?;

            if let Err(mut root) = template_html(&html, output.as_deref()) {
                let CliError::RewriteError {
                    source: rewriting_error,
                } = &mut root
                else {
                    return Err(root.into());
                };

                let RewritingError::ContentHandlerError(any_error) = rewriting_error else {
                    return Err(root.into());
                };

                if let Some(error) = any_error.downcast_mut::<FromElementError>() {
                    error.element_location = error
                        .element_location
                        .as_file_position(html.as_bytes(), source.clone());
                }

                return Err(root.into());
            }
        }
    }

    Ok(())
}

fn template_html(html: &str, output: Option<&Path>) -> Result<usize, CliError> {
    let start = Instant::now();
    let templated_html = replace_htmplates(html).map_err(CliError::rewrite_error)?;
    let template_duration = start.elapsed();

    let sink = get_stdio_output(output)?;

    if let Err(source) = write_formatted_html(&templated_html, sink) {
        match source {
            WriteFormattedError::NoDeno => {
                let sink = get_write_output(output)?;
                write_unformatted(&templated_html, sink)
                    .map_err(CliError::write_unformatted_html)?;
            }

            WriteFormattedError::FormatError { stderr } => {
                let sink = get_write_output(output)?;
                write_unformatted(&templated_html, sink)
                    .map_err(CliError::write_unformatted_html)?;

                print_warning("could not format templated HTML:", 0);
                for line in stderr.lines() {
                    println!(" {line}");
                }

                return Ok(stderr.lines().count() + 1);
            }

            _ => return Err(CliError::write_formatted_html(source)),
        }
    }

    print_success(
        &format!("Templated HTML in {}µs", template_duration.as_micros()),
        0,
    );

    Ok(1)
}

fn get_stdio_output(output_path: Option<&Path>) -> Result<Stdio, CliError> {
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

fn get_write_output(output_path: Option<&Path>) -> Result<Box<dyn Write>, CliError> {
    let Some(output_path) = output_path else {
        return Ok(Box::new(stdout()));
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

    Ok(Box::new(file))
}

fn write_formatted_html<W: Into<Stdio>>(html: &str, out: W) -> Result<(), WriteFormattedError> {
    // Ensure deno exists.
    if std::process::Command::new("deno")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .is_err()
    {
        return Err(WriteFormattedError::NoDeno);
    }

    let mut child = std::process::Command::new("deno")
        .arg("fmt")
        .arg("--ext")
        .arg("html")
        .arg("-")
        .stdout(out)
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

        return Err(WriteFormattedError::FormatError {
            stderr: stderr_buffer,
        });
    }

    Ok(())
}

fn write_unformatted<W: Write>(html: &str, mut out: W) -> io::Result<()> {
    out.write_all(html.as_bytes())
}

/// Error variants for writing formatted HTML.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum WriteFormattedError {
    #[non_exhaustive]
    NoDeno,

    #[non_exhaustive]
    FormatError { stderr: String },

    #[non_exhaustive]
    IoError { source: io::Error },
}
impl core::fmt::Display for WriteFormattedError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::FormatError { stderr, .. } => {
                writeln!(f, "writing formatted HTML failed:")?;
                for line in stderr.lines() {
                    writeln!(f, " {line}")?;
                }
                Ok(())
            }
            Self::IoError { .. } => write!(f, "IO error from spawning formatter process"),
            Self::NoDeno => write!(f, "deno is required to format the output"),
        }
    }
}
impl Error for WriteFormattedError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::IoError { source, .. } => Some(source),
            _ => None,
        }
    }
}
impl From<io::Error> for WriteFormattedError {
    fn from(value: io::Error) -> Self {
        Self::IoError { source: value }
    }
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
    OpenOutput { source: io::Error },

    #[non_exhaustive]
    WriteUnformattedHtml { source: io::Error },

    #[non_exhaustive]
    WriteFormattedHtml { source: WriteFormattedError },

    #[non_exhaustive]
    RewriteError { source: RewritingError },

    #[non_exhaustive]
    WatchError { source: notify::Error },
}
impl core::fmt::Display for CliError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::ReadSource { .. } => write!(f, "could not read source file"),
            Self::OpenOutput { .. } => write!(f, "could not open output"),
            Self::RewriteError { .. } => write!(f, "could not parse an htmplate"),
            Self::WatchError { .. } => write!(f, "error while watching source file"),
            Self::SourceIsNotAFile { .. } => write!(f, "source is not a file"),
            Self::OutputIsNotAFile { .. } => write!(f, "output is not a file"),
            Self::WriteFormattedHtml { .. } => write!(f, "could not write formatted HTML"),
            Self::WriteUnformattedHtml { .. } => write!(f, "could not write HTML"),
        }
    }
}
impl Error for CliError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            Self::ReadSource { source, .. } => Some(source),
            Self::OpenOutput { source, .. } => Some(source),
            Self::RewriteError { source, .. } => Some(source),
            Self::WatchError { source, .. } => Some(source),
            Self::WriteFormattedHtml { source, .. } => Some(source),
            Self::WriteUnformattedHtml { source, .. } => Some(source),
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
        Self::OpenOutput { source }
    }

    #[allow(missing_docs)]
    pub fn rewrite_error(source: RewritingError) -> Self {
        Self::RewriteError { source }
    }

    #[allow(missing_docs)]
    pub fn watch_error(source: notify::Error) -> Self {
        Self::WatchError { source }
    }

    #[allow(missing_docs)]
    pub fn write_formatted_html(source: WriteFormattedError) -> Self {
        Self::WriteFormattedHtml { source }
    }

    #[allow(missing_docs)]
    pub fn write_unformatted_html(source: io::Error) -> Self {
        Self::WriteUnformattedHtml { source }
    }
}
