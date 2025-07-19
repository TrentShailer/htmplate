use core::time::Duration;
use std::{path::Path, sync::mpsc::channel, time::Instant};

use htmplate::ReplaceHtmplateError;
use notify::{RecursiveMode, Watcher, recommended_watcher};
use ts_cli_helper::{print_fail, print_success};
use ts_rust_helper::{error::ErrorStackStyle, style::*};

use crate::cli::{
    Command,
    template::{TemplateError, template_file},
};

impl Command {
    pub fn watch(
        source: &Path,
        output: &Path,
        asset_directory: Option<&Path>,
    ) -> Result<(), WatchError> {
        let mut y = 0;
        let mut event_count = 1;

        // Do first run to validate
        let result = template_file(source, output, asset_directory);
        report_result(result, source, event_count, &mut y)
            .map_err(|source| WatchError::TemplateFile { source })?;

        let (tx, rx) = channel();
        let mut watcher =
            recommended_watcher(tx).map_err(|source| WatchError::WatchSource { source })?;
        watcher
            .watch(source, RecursiveMode::NonRecursive)
            .map_err(|source| WatchError::WatchSource { source })?;

        let mut last_event = Instant::now();
        for result in &rx {
            // Format at most once per 100 milliseconds
            if last_event.elapsed() < Duration::from_millis(100) {
                continue;
            }
            // Do not format if there is another event within 100 milliseconds
            if rx.recv_timeout(Duration::from_millis(100)).is_ok() {
                continue;
            }

            result.map_err(|source| WatchError::WatchSource { source })?;

            let result = template_file(source, output, asset_directory);
            report_result(result, source, event_count, &mut y)
                .map_err(|source| WatchError::TemplateFile { source })?;

            last_event = Instant::now();
            event_count += 1;
        }

        Ok(())
    }
}

fn report_result(
    result: Result<(), TemplateError>,
    source_file: &Path,
    event_count: usize,
    y: &mut usize,
) -> Result<(), TemplateError> {
    while *y > 0 {
        print!("{ERASE_LINE_UP}");
        *y -= 1;
    }

    match result {
        Ok(_) => {
            print_success(format_args!(
                "Templated `{}`",
                source_file.to_string_lossy()
            ));
            *y += 1;
        }

        Err(error) => match error {
            TemplateError::TemplateHtml { ref source } => match source {
                ReplaceHtmplateError::InvalidHtmplate { source, .. } => {
                    print_fail(format_args!(
                        "Could not template `{}`",
                        source_file.to_string_lossy()
                    ));

                    let report = source.to_string();
                    print!("{report}");
                    *y += report.lines().count() + 1
                }

                ReplaceHtmplateError::HtmplateError { source, .. } => {
                    print_fail(format_args!(
                        "Could not template `{}`",
                        source_file.to_string_lossy()
                    ));

                    let report = ErrorStackStyle::Stacked { indent: 2 }
                        .display(&source)
                        .unwrap_or_default();

                    print!("{report}");

                    *y += report.lines().count() + 1
                }

                ReplaceHtmplateError::HtmplateDoesNotExist { .. } => {
                    print_fail(format_args!(
                        "Could not template `{}`",
                        source_file.to_string_lossy()
                    ));

                    let report = ErrorStackStyle::Stacked { indent: 2 }
                        .display(&source)
                        .unwrap_or_default();

                    print!("{report}");

                    *y += report.lines().count() + 1
                }
                _ => return Err(error),
            },

            _ => return Err(error),
        },
    }

    println!(
        "Watching `{}` press `Ctrl + C` to exit ({event_count})",
        source_file.to_string_lossy()
    );
    *y += 1;
    Ok(())
}

/// Error variants for watching a file.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum WatchError {
    #[non_exhaustive]
    WatchSource { source: notify::Error },

    #[non_exhaustive]
    TemplateFile { source: TemplateError },
}
impl core::fmt::Display for WatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::WatchSource { .. } => write!(f, "error while watching source file"),
            Self::TemplateFile { .. } => write!(f, "templating failed"),
        }
    }
}
impl core::error::Error for WatchError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::WatchSource { source, .. } => Some(source),
            Self::TemplateFile { source, .. } => Some(source),
        }
    }
}
