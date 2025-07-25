use std::{io, path::PathBuf};

use clap::{Parser, Subcommand};
use log::Level;

use crate::{
    actions::{
        BundleScriptError, TemplateError, WriteLibraryError, bundle_script, template_html,
        write_library,
    },
    cli::watch::{WatchError, watch},
};

mod list;
mod watch;

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
        /// The path to watch for changes to `index.template.html` file changes.
        root: PathBuf,

        /// Enable verbose debug output
        #[arg(long)]
        verbose: bool,
    },

    /// Template a file
    Template {
        /// The path to the HTML file containing htmplates.
        source: PathBuf,

        /// The file to output the templated HTML to.
        target: PathBuf,

        /// Enable verbose debug output
        #[arg(long)]
        verbose: bool,
    },

    /// Write the assets out
    Assets {
        /// The directory to write the assets to.
        asset_directory: PathBuf,
    },

    /// Bundle the TypeScript
    Bundle {
        /// The path to the TypeScript file to bundle.
        source: PathBuf,

        /// Enable verbose debug output
        #[arg(long)]
        verbose: bool,
    },

    /// List the htmplates
    List {
        /// The htmplate to search for
        search: Option<String>,
    },
}

impl Command {
    pub fn execute(self) -> Result<(), CommandError> {
        match self {
            Self::List { search } => Self::print_templates(search.as_deref())
                .map_err(|source| CommandError::List { source })?,

            Self::Template {
                source,
                target,
                verbose,
            } => {
                if verbose {
                    simple_logger::init_with_level(Level::Debug)
                        .map_err(|source| CommandError::SetupLogger { source })?;
                }

                template_html(&source, &target)
                    .map_err(|source| CommandError::Template { source })?;
            }

            Self::Assets { asset_directory } => {
                write_library(&asset_directory).map_err(|source| CommandError::Assets { source })?
            }

            Self::Bundle { source, verbose } => {
                if verbose {
                    simple_logger::init_with_level(Level::Debug)
                        .map_err(|source| CommandError::SetupLogger { source })?;
                }

                bundle_script(&source).map_err(|source| CommandError::Bundle { source })?;
            }

            Self::Watch { root, verbose } => {
                if verbose {
                    simple_logger::init_with_level(Level::Debug)
                        .map_err(|source| CommandError::SetupLogger { source })?;
                }
                watch(&root).map_err(|source| CommandError::Watch { source })?
            }
        }

        Ok(())
    }
}

/// Error variants for the subcommands.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum CommandError {
    #[non_exhaustive]
    List { source: io::Error },

    #[non_exhaustive]
    Assets { source: WriteLibraryError },

    #[non_exhaustive]
    Template { source: TemplateError },

    #[non_exhaustive]
    Bundle { source: BundleScriptError },

    #[non_exhaustive]
    Watch { source: WatchError },

    #[non_exhaustive]
    SetupLogger { source: log::SetLoggerError },
}
impl core::fmt::Display for CommandError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::List { .. } => write!(f, "listing htmplates failed"),
            Self::Assets { .. } => write!(f, "writing assets failed"),
            Self::Template { .. } => write!(f, "templating HTML failed"),
            Self::Watch { .. } => write!(f, "failed while watching"),
            Self::SetupLogger { .. } => write!(f, "failed to create logger"),
            Self::Bundle { .. } => write!(f, "failed to bundle script"),
        }
    }
}
impl core::error::Error for CommandError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::List { source, .. } => Some(source),
            Self::Assets { source, .. } => Some(source),
            Self::Template { source, .. } => Some(source),
            Self::Watch { source, .. } => Some(source),
            Self::SetupLogger { source, .. } => Some(source),
            Self::Bundle { source, .. } => Some(source),
        }
    }
}
