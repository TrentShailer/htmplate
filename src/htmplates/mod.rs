//! htmplates
//!

use core::error::Error;

use lol_html::html_content::Element;

mod footer;
mod metadata;
mod title;

pub use footer::Footer;
pub use metadata::Metadata;
pub use title::Title;

/// An htmplate
pub trait Htmplate {
    /// The HTML tag for this element.
    fn tag(&self) -> &'static str;

    /// Replace a htmplate with the HTML.
    fn replace(&self, el: &mut Element) -> Result<(), Box<dyn Error + Send + Sync>>;
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
}
impl core::fmt::Display for HtmplateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::InvalidIcon { icon, .. } => write!(f, "the icon `{icon}` does not exist"),
            Self::HtmplateNotFound { tag, .. } => write!(f, "the htmplate `{tag}` does not exist"),
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
