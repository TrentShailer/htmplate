//! htmplates
//!

mod alert;
mod footer;
mod metadata;
mod title;

pub use alert::Alert;
pub use footer::Footer;
use lol_html::html_content::ContentType;
pub use metadata::Metadata;
pub use title::Title;

use crate::HtmplateElement;

/// Trait for turning an htmplate into it's HTML.
pub trait ToHtml {
    /// Turn the htmplate into HTML.
    fn to_html(self) -> Result<String, HtmplateError>;
}

/// Create a standard replacer for an htmplate.
pub fn replacer<T: HtmplateElement + ToHtml>(
    el: &mut lol_html::html_content::Element,
) -> Result<(), Box<dyn core::error::Error + Send + Sync + 'static>> {
    let htmplate = T::from_element(el)?;
    let html = htmplate.to_html()?;

    el.start_tag().remove();
    el.before(&html, ContentType::Html);

    Ok(())
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
impl core::error::Error for HtmplateError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
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
