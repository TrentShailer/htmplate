use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::{self, Write, stdout},
    path::{Path, PathBuf},
    sync::mpsc::channel,
};

use argh::FromArgs;
use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use ts_ansi::{format_failure, format_success, style::CLEAR_TERMINAL};
use ts_error::{IntoReport, Report};
use ts_path::{DisplayPath, RelativePath};

use crate::actions::{WriteLibraryError, bundle_script, template_html, write_library};

/// Watch a directory and template any htmplate files on change.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "watch")]
pub struct WatchSubcommand {
    /// the path to watch for file changes
    #[argh(positional)]
    root: PathBuf,
}

impl WatchSubcommand {
    pub fn watch(&self) -> Result<(), WatchError> {
        write_library(&self.root.join("lib"))
            .map_err(|source| WatchError::WriteLibrary { source })?;
        eprintln!("{}", format_success!("write library"));

        let mut status_map = process_all_files(&self.root);
        if let Err(report) = display_tracked_files(&status_map).into_report() {
            eprintln!("{report}",)
        };

        let (tx, rx) = channel();
        let mut watcher =
            recommended_watcher(tx).map_err(|source| WatchError::CreateWatcher { source })?;
        watcher
            .watch(&self.root, RecursiveMode::Recursive)
            .map_err(|source| WatchError::CreateWatcher { source })?;

        for result in &rx {
            let event = result.map_err(|source| WatchError::WatcherError { source })?;

            match event.kind {
                EventKind::Modify(_) | EventKind::Create(_) => {}

                EventKind::Remove(_) => {
                    for path in event.paths {
                        status_map.remove(&path);
                    }

                    if let Err(report) = display_tracked_files(&status_map).into_report() {
                        eprintln!("{report}");
                    }

                    continue;
                }
                _ => continue,
            }

            for path in event.paths {
                handle_file(&path, &mut status_map);
            }
            if let Err(report) = display_tracked_files(&status_map).into_report() {
                eprintln!("{report}");
            }
        }

        Ok(())
    }
}

fn handle_file(path: &Path, status_map: &mut HashMap<PathBuf, FileStatus>) {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());

    if path.ends_with("index.template.html") {
        let target = &path.with_file_name("index.html");
        let result = template_html(&path, target);

        if let Some(status) = status_map.get_mut(&path) {
            status.event_count += 1;
            status.last_status = result.map_err(Box::from);
        } else {
            status_map.insert(
                path.to_path_buf(),
                FileStatus {
                    event_count: 1,
                    last_status: result.map_err(Box::from),
                    kind: FileKind::Html,
                },
            );
        }
    } else if path.ends_with("index.ts") {
        let result = bundle_script(&path);

        if let Some(status) = status_map.get_mut(&path) {
            status.event_count += 1;
            status.last_status = result.map_err(Box::from);
        } else {
            status_map.insert(
                path.to_path_buf(),
                FileStatus {
                    event_count: 1,
                    last_status: result.map_err(Box::from),
                    kind: FileKind::Script,
                },
            );
        }
    }
}

fn process_all_files(root: &Path) -> HashMap<PathBuf, FileStatus> {
    let mut status_map: HashMap<PathBuf, FileStatus> = HashMap::new();

    let mut file_queue = VecDeque::new();
    file_queue.push_back(root.to_path_buf());
    while let Some(path) = file_queue.pop_front() {
        let Ok(metadata) = fs::metadata(&path) else {
            continue;
        };
        if metadata.is_dir() {
            let Ok(dir) = fs::read_dir(&path) else {
                continue;
            };
            for entry in dir {
                let Ok(entry) = entry else {
                    continue;
                };

                file_queue.push_back(entry.path());
            }
        } else if metadata.is_file() {
            handle_file(&path, &mut status_map);
        }
    }

    status_map
}

#[derive(Debug)]
pub struct FileStatus {
    pub event_count: usize,
    pub last_status: Result<(), Box<dyn core::error::Error>>,
    pub kind: FileKind,
}
#[derive(Debug)]
pub enum FileKind {
    Script,
    Html,
}
impl core::fmt::Display for FileStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let event_count = self.event_count;

        match &self.last_status {
            Ok(_) => {
                let operation = match self.kind {
                    FileKind::Script => "bundled",
                    FileKind::Html => "templated",
                };
                write!(f, "was {operation} ({event_count})")?;
            }
            Err(source) => {
                let operation = match self.kind {
                    FileKind::Script => "bundle",
                    FileKind::Html => "template",
                };
                let report = Report::new(source.as_ref());

                write!(f, "failed to {operation} ({event_count}):\n{report}")?;
            }
        }
        Ok(())
    }
}

fn display_tracked_files(map: &HashMap<PathBuf, FileStatus>) -> io::Result<()> {
    let mut stdout = stdout().lock();

    stdout.write_all(CLEAR_TERMINAL.as_bytes())?;

    for (path, status) in map.iter() {
        if status.last_status.is_err() {
            stdout.write_all(
                format!(
                    "{}",
                    format_failure!(
                        "`{}` {}\n",
                        path.relative_to_cwd().opinionated_display(),
                        status
                    )
                )
                .as_bytes(),
            )?;
        } else {
            stdout.write_all(
                format!(
                    "{}",
                    format_success!(
                        "`{}` {}\n",
                        path.relative_to_cwd().opinionated_display(),
                        status
                    )
                )
                .as_bytes(),
            )?;
        }
    }

    stdout.write_all(b"Press `ctrl + C` to exit\n")?;
    stdout.flush()?;

    Ok(())
}

/// Error variants for watching a directory.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum WatchError {
    #[non_exhaustive]
    WriteLibrary { source: WriteLibraryError },

    #[non_exhaustive]
    CreateWatcher { source: notify::Error },

    #[non_exhaustive]
    WatcherError { source: notify::Error },
}
impl core::fmt::Display for WatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::WriteLibrary { .. } => write!(f, "could not write library"),
            Self::CreateWatcher { .. } => write!(f, "could not create file system watcher"),
            Self::WatcherError { .. } => write!(f, "watcher returned an error event"),
        }
    }
}
impl core::error::Error for WatchError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::WriteLibrary { source, .. } => Some(source),
            Self::CreateWatcher { source, .. } => Some(source),
            Self::WatcherError { source, .. } => Some(source),
        }
    }
}
