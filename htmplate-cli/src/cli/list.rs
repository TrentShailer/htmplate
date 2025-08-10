use std::io::{self, Write, stdout};

use argh::FromArgs;
use htmplate::{HtmplateDetails, all_htmplate_details};
use ts_ansi::style::*;

/// List the htmplate elements.
#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "list")]
pub struct ListSubcommand {
    /// search string for a specific htmplate element
    #[argh(positional)]
    search: Option<String>,
}

impl ListSubcommand {
    pub fn print_templates(&self) -> io::Result<()> {
        let mut htmplates = all_htmplate_details();

        if let Some(search) = self.search.as_ref() {
            let search = search.to_lowercase();
            htmplates.retain(|htmplate| htmplate.tag.contains(&search));
        }

        let mut stdout = stdout().lock();

        for htmplate in htmplates {
            write_htmplate(&htmplate, &mut stdout)?;
        }

        Ok(())
    }
}

fn write_htmplate<W: Write>(details: &HtmplateDetails, f: &mut W) -> io::Result<()> {
    writeln!(
        f,
        "<{BLUE}{}{RESET} /> {DIM}{}{RESET}",
        details.tag.replace("\\", ""),
        details.description
    )?;

    let attribute_width = details
        .attributes
        .iter()
        .map(|v| v.name.len())
        .max()
        .unwrap_or_default();

    for attribute in &details.attributes {
        // Write padding
        write!(
            f,
            "{}",
            " ".repeat(attribute_width - attribute.name.len() + 2)
        )?;

        if attribute.required {
            write!(f, "{RED}*{RESET}")?;
        } else {
            write!(f, " ")?;
        }

        writeln!(
            f,
            "[{}]{DIM}: {}{RESET}",
            attribute.name, attribute.description
        )?;
    }

    writeln!(f)?;

    Ok(())
}
