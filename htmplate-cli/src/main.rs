//! # `htmplate-cli`
//!

use clap::Parser;
use ts_rust_helper::error::ReportProgramExit;

use crate::cli::Cli;

mod cli;

fn main() -> ReportProgramExit {
    let cli = Cli::parse();

    cli.command.execute()?;

    Ok(())
}
