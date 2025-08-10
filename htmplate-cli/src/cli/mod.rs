use std::io;

use argh::FromArgs;

use crate::{
    actions::{BundleScriptError, TemplateError, WriteLibraryError},
    cli::{
        assets::AssetsSubcommand,
        bundle::BundleSubcommand,
        list::ListSubcommand,
        template::TemplateSubcommand,
        watch::{WatchError, WatchSubcommand},
    },
};

mod assets;
mod bundle;
mod list;
mod template;
mod watch;

/// Template `<htmplate:... />` elements in HTML.
#[derive(Debug, FromArgs)]
pub struct Cli {
    /// what subcommand to run
    #[argh(subcommand)]
    pub command: Command,
}

/// Subcommand
#[derive(Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Watch(WatchSubcommand),
    Template(TemplateSubcommand),
    Assets(AssetsSubcommand),
    Bundle(BundleSubcommand),
    List(ListSubcommand),
}

impl Command {
    pub fn execute(self) -> Result<(), CommandError> {
        match self {
            Self::List(subcommand) => subcommand
                .print_templates()
                .map_err(|source| CommandError::List { source }),
            Self::Template(subcommand) => subcommand.template(),
            Self::Assets(subcommand) => subcommand.write_assets(),
            Self::Bundle(subcommand) => subcommand.bundle(),
            Self::Watch(subcommand) => subcommand
                .watch()
                .map_err(|source| CommandError::Watch { source }),
        }
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
}
impl core::fmt::Display for CommandError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::List { .. } => write!(f, "listing htmplates failed"),
            Self::Assets { .. } => write!(f, "writing assets failed"),
            Self::Template { .. } => write!(f, "templating HTML failed"),
            Self::Watch { .. } => write!(f, "failed while watching"),
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
            Self::Bundle { source, .. } => Some(source),
        }
    }
}
