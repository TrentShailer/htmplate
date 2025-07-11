//! htmplates
//!

use core::error::Error;

use lol_html::html_content::Element;
use ts_rust_helper::style::{BOLD, DIM, RED, RESET};

mod footer;
mod metadata;
mod title;

pub use footer::Footer;
pub use metadata::Metadata;
pub use title::Title;

/// An attribute of an htmplate
#[derive(Debug, Clone)]
pub struct Attribute {
    /// The attribute name.
    pub name: &'static str,
    /// The description of the attribute's value.
    pub value_description: &'static str,
    /// If the attribute is required
    pub required: bool,
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.required {
            write!(f, "{BOLD}{RED}*{RESET}")?;
        }
        write!(
            f,
            "{BOLD}[{}]{RESET}: {}",
            self.name, self.value_description
        )?;

        Ok(())
    }
}

/// An htmplate
pub trait Htmplate {
    /// The HTML tag for this element.
    fn tag(&self) -> &'static str;

    /// Return the attributes for this element.
    fn attributes(&self) -> Vec<Attribute>;

    /// Replace a htmplate with the HTML.
    fn replace(&self, el: &mut Element) -> Result<(), Box<dyn Error + Send + Sync>>;

    /// The description of the htmplate.
    fn description(&self) -> &'static str;
}

/// Returns all the htmplates.
pub fn get_all_htmplates() -> Vec<Box<dyn Htmplate>> {
    vec![Box::new(Metadata), Box::new(Footer), Box::new(Title)]
}

/// Error variants for replacing a htmplate.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum HtmplateError {
    #[non_exhaustive]
    InvalidIcon { icon: String },

    #[non_exhaustive]
    HtmplateNotFound { tag: String },

    #[non_exhaustive]
    MissingAttributes {
        tag: String,
        attributes: Vec<String>,
    },
}
impl fmt::Display for HtmplateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Self::InvalidIcon { icon, .. } => write!(f, "the icon `{icon}` does not exist"),
            Self::HtmplateNotFound { tag, .. } => write!(f, "the htmplate `{tag}` does not exist"),
            Self::MissingAttributes {
                tag, attributes, ..
            } => write!(
                f,
                "a `{tag}` is missing the attributes: [{}]",
                attributes.join(", ")
            ),
        }
    }
}
impl Error for HtmplateError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
impl HtmplateError {
    #[allow(missing_docs)]
    pub fn invalid_icon<S: ToString>(icon: S) -> Self {
        Self::InvalidIcon {
            icon: icon.to_string(),
        }
    }
}
