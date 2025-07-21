//! htmplates
//!

mod alert;
mod footer;
mod form;
mod hr;
mod icon;
mod metadata;
mod title;

use std::path::Path;

use lol_html::html_content::ContentType;

pub use alert::Alert;
pub use footer::Footer;
pub use form::{FormAlert, FormCheckInput, FormSubmit, FormTextInput};
pub use hr::Hr;
pub use icon::Icon;
pub use metadata::Metadata;
pub use title::Title;

use crate::{HtmplateElement, Location};

/// Trait for turning an htmplate into it's HTML.
pub trait ToHtml {
    /// Turn the htmplate into HTML.
    fn to_html(self) -> Result<String, HtmplateErrorKind>;
}

/// Create a standard replacer for an htmplate.
pub fn replacer<T: HtmplateElement + ToHtml>(
    el: &mut lol_html::html_content::Element,
    html: &str,
    file_path: &Path,
) -> Result<(), Box<dyn core::error::Error + Send + Sync + 'static>> {
    let htmplate = T::from_element(el, html, file_path)?;

    match htmplate.to_html() {
        Ok(html) => {
            el.start_tag().remove();
            el.before(&html, ContentType::Html);
        }
        Err(kind) => {
            let location = Location::from_byte_index(
                el.source_location().bytes().start,
                html.as_bytes(),
                file_path,
            );
            return Err(Box::new(HtmplateError {
                tag: el.tag_name(),
                location,
                kind,
            }));
        }
    }

    Ok(())
}

/// Error type for replacing an htmplate.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub struct HtmplateError {
    pub tag: String,
    pub location: Location,
    pub kind: HtmplateErrorKind,
}
impl core::fmt::Display for HtmplateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "could not template a {} at {}", self.tag, self.location)
    }
}
impl core::error::Error for HtmplateError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.kind)
    }
}

/// Error variants for replacing a htmplate.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum HtmplateErrorKind {
    #[non_exhaustive]
    InvalidAttribute { attribute: String, expected: String },
}
impl core::fmt::Display for HtmplateErrorKind {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::InvalidAttribute {
                attribute,
                expected,
                ..
            } => write!(f, "invalid attribute `{attribute}`, {expected}"),
        }
    }
}
impl core::error::Error for HtmplateErrorKind {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}
impl HtmplateErrorKind {
    #[allow(missing_docs)]
    pub fn invalid_attribute<S1: ToString, S2: ToString>(attribute: S1, expected: S2) -> Self {
        Self::InvalidAttribute {
            attribute: attribute.to_string(),
            expected: expected.to_string(),
        }
    }
}
