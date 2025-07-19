use std::{
    collections::HashMap,
    ffi::OsStr,
    io::{self, Write, stdout},
    path::{Path, PathBuf},
    sync::mpsc::channel,
    time::Instant,
};

use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use ts_rust_helper::{
    error::{ErrorStackStyle, Report},
    style::*,
};

use crate::cli::{
    Command,
    template::{TemplateError, template_file},
};

impl Command {
    pub fn watch(watch_target: &Path, asset_directory: Option<&Path>) -> Result<(), WatchError> {
        let (tx, rx) = channel();
        let mut watcher =
            recommended_watcher(tx).map_err(|source| WatchError::WatchSource { source })?;
        watcher
            .watch(watch_target, RecursiveMode::Recursive)
            .map_err(|source| WatchError::WatchSource { source })?;

        let mut status_map: HashMap<PathBuf, FileStatus> = HashMap::new();

        for result in &rx {
            let event = result.map_err(|source| WatchError::WatchSource { source })?;

            match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) => {}

                EventKind::Remove(_) => {
                    for path in event.paths {
                        status_map.remove(&path);
                    }

                    if let Err(e) = display_tracked_files(&status_map) {
                        eprintln!("could not display: {e}");
                    }

                    continue;
                }
                _ => continue,
            }

            for path in event.paths {
                let Ok(path) = path.canonicalize() else {
                    continue;
                };

                let source_name = path
                    .file_name()
                    .unwrap_or_else(|| OsStr::new(""))
                    .to_string_lossy();

                if !source_name.ends_with(".template.html") {
                    continue;
                }

                let output_name = source_name.replace(".template.html", ".html");
                let output = path.with_file_name(output_name);

                let result = template_file(&path, &output, asset_directory);

                if let Some(status) = status_map.get_mut(&path) {
                    status.event_count += 1;
                    status.last_event = Instant::now();
                    status.last_status = result;
                } else {
                    status_map.insert(
                        path,
                        FileStatus {
                            event_count: 1,
                            last_event: Instant::now(),
                            last_status: result,
                        },
                    );
                }

                if let Err(e) = display_tracked_files(&status_map) {
                    eprintln!("could not display: {e}");
                }
            }
        }

        Ok(())
    }
}

pub struct FileStatus {
    pub event_count: usize,
    pub last_event: Instant,
    pub last_status: Result<(), TemplateError>,
}

fn display_tracked_files(map: &HashMap<PathBuf, FileStatus>) -> io::Result<()> {
    let mut stdout = stdout().lock();

    stdout.write_all(CLEAR_TERMINAL.as_bytes())?;

    for (file, status) in map.iter() {
        match &status.last_status {
            Ok(_) => {
                stdout.write_all(
                    format!(
                        "{BOLD}{GREEN}Success{RESET}{BOLD}:{RESET} templated {} ({})\n",
                        display_path(file),
                        status.event_count
                    )
                    .as_bytes(),
                )?;
            }
            Err(error) => {
                stdout.write_all(format!("{BOLD}{RED}Failure{RESET}{BOLD}:{RESET} ").as_bytes())?;

                let report = Report::new(
                    format!("formatting {}", display_path(file)),
                    error,
                    ErrorStackStyle::Stacked { indent: 2 },
                );

                stdout.write_all(report.to_string().as_bytes())?;
                stdout.write_all(b"\n")?;
            }
        }
    }

    stdout.write_all(b"Press `ctrl + C` to exit")?;

    Ok(())
}

pub fn display_path(path: &Path) -> String {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    path.to_string_lossy().replace("\\\\?\\", "")
}

/// Error variants for watching a file.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum WatchError {
    #[non_exhaustive]
    WatchSource { source: notify::Error },
}
impl core::fmt::Display for WatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::WatchSource { .. } => write!(f, "error while watching source"),
        }
    }
}
impl core::error::Error for WatchError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::WatchSource { source, .. } => Some(source),
        }
    }
}
