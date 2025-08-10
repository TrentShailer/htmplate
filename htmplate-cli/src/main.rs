//! # `htmplate-cli`
//!

use ts_error::ReportProgramExit;

use crate::cli::Cli;

mod actions;
mod cli;

fn main() -> ReportProgramExit {
    let cli: Cli = argh::from_env();

    cli.command.execute()?;

    Ok(())
}
