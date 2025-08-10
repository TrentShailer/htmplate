use std::path::PathBuf;

use argh::FromArgs;

use crate::{actions::template_html, cli::CommandError};

/// Template an htmplate file.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "template")]
pub struct TemplateSubcommand {
    /// the path to the HTML file containing htmplates
    #[argh(positional)]
    source: PathBuf,

    /// the file to output the templated HTML to
    #[argh(positional)]
    target: PathBuf,
}

impl TemplateSubcommand {
    pub fn template(&self) -> Result<(), CommandError> {
        template_html(&self.source, &self.target)
            .map_err(|source| CommandError::Template { source })
    }
}
