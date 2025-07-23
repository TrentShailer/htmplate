use std::{
    collections::{HashMap, VecDeque},
    fs,
    io::{self, Write, stdout},
    path::{Path, PathBuf},
    sync::mpsc::channel,
};

use notify::{EventKind, RecursiveMode, Watcher, recommended_watcher};
use ts_cli_helper::print_success;
use ts_rust_helper::{
    error::{ErrorStackStyle, IntoErrorReport},
    path::RelativePath,
    style::*,
};

use crate::actions::{WriteLibraryError, bundle_script, template_html, write_library};

pub fn watch(root: &Path) -> Result<(), WatchError> {
    write_library(&root.join("lib")).map_err(|source| WatchError::WriteLibrary { source })?;
    print_success("wrote library");

    let mut status_map = process_all_files(root);
    log::debug!("processed all files");
    if let Err(report) = display_tracked_files(&status_map).into_report("display tracked files") {
        eprintln!("{report}",)
    };

    let (tx, rx) = channel();
    let mut watcher =
        recommended_watcher(tx).map_err(|source| WatchError::CreateWatcher { source })?;
    watcher
        .watch(root, RecursiveMode::Recursive)
        .map_err(|source| WatchError::CreateWatcher { source })?;
    log::debug!("started watching");

    for result in &rx {
        let event = result.map_err(|source| WatchError::WatcherError { source })?;

        match event.kind {
            EventKind::Modify(_) | EventKind::Create(_) => {}

            EventKind::Remove(_) => {
                for path in event.paths {
                    status_map.remove(&path);
                }

                if let Err(report) =
                    display_tracked_files(&status_map).into_report("display tracked files")
                {
                    eprintln!("{report}");
                }

                continue;
            }
            _ => continue,
        }

        for path in event.paths {
            handle_file(&path, &mut status_map);
        }
        if let Err(report) = display_tracked_files(&status_map).into_report("display tracked files")
        {
            eprintln!("{report}");
        }
    }

    Ok(())
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
        let target = &path.with_file_name("index.js");
        let result = bundle_script(&path, target);

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
        log::debug!("processing: {path:?}");
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
                let report = ErrorStackStyle::default()
                    .display(source.as_ref())
                    .unwrap_or_else(|_| source.to_string());

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
                    "{BOLD}{RED}Failure{RESET}{BOLD}:{RESET} `{}` {}\n",
                    path.relative_to_current_dir().opinionated_display(),
                    status
                )
                .as_bytes(),
            )?;
        } else {
            stdout.write_all(
                format!(
                    "{BOLD}{GREEN}Success{RESET}{BOLD}:{RESET} `{}` {}\n",
                    path.relative_to_current_dir().opinionated_display(),
                    status
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
