//! # `htmplate`
//! Library to make reusable components in HTML via `<htmplate:... />` elements.

pub mod assets;
mod htmplate_element;
pub mod htmplates;
mod icon;

use std::{path::Path, sync::LazyLock};

use lol_html::{Settings, element, errors::RewritingError, rewrite_str};

pub use htmplate_element::{Attribute, FromElementError, HtmplateElement, Location};
pub use lol_html;
use regex::Regex;

use crate::htmplates::{
    Alert, Footer, FormAlert, FormCheckInput, FormSubmit, FormTextInput, Hr, HtmplateError, Icon,
    Metadata, Title, replacer,
};

/// The details for an htmplate
pub struct HtmplateDetails {
    /// The htmplate's tag.
    pub tag: &'static str,
    /// The htmplate's description
    pub description: &'static str,
    /// The htmplate's attributes
    pub attributes: Vec<Attribute>,
}
impl HtmplateDetails {
    pub(crate) fn new<T: HtmplateElement>() -> Self {
        Self {
            tag: T::tag(),
            description: T::description(),
            attributes: T::attributes(),
        }
    }
}

/// Returns the details for all htmplates.
pub fn all_htmplate_details() -> Vec<HtmplateDetails> {
    vec![
        HtmplateDetails::new::<Title>(),
        HtmplateDetails::new::<Metadata>(),
        HtmplateDetails::new::<Footer>(),
        HtmplateDetails::new::<Alert>(),
        HtmplateDetails::new::<FormAlert>(),
        HtmplateDetails::new::<FormTextInput>(),
        HtmplateDetails::new::<FormCheckInput>(),
        HtmplateDetails::new::<FormSubmit>(),
        HtmplateDetails::new::<Icon>(),
        HtmplateDetails::new::<Hr>(),
    ]
}

static NEWLINE_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\r\n|\n|\r)\s*").unwrap());
static GAP_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r">\s<").unwrap());

/// Replace the htmplates in some source HTML.
pub fn replace_htmplates(
    html: &str,
    html_path: &Path,
    path_from_output_to_assets: &Path,
) -> Result<String, ReplaceHtmplateError> {
    let tags: Vec<_> = all_htmplate_details()
        .into_iter()
        .map(|detail| detail.tag)
        .collect();

    let not_found_handler = {
        let not_selectors: String = tags.iter().map(|tag| format!(":not({tag})")).collect();

        element!(format!("*{not_selectors}"), |el| {
            if el.tag_name().starts_with("htmplate") {
                Err(Box::new(ReplaceHtmplateError::HtmplateDoesNotExist {
                    tag: el.tag_name(),
                    location: Location::from_byte_index(
                        el.source_location().bytes().start,
                        html.as_bytes(),
                        html_path,
                    ),
                }))
            } else {
                Ok(())
            }
        })
    };

    let html = rewrite_str(
        html,
        Settings {
            #[rustfmt::skip]
            element_content_handlers: vec![
                element!(Metadata::tag(), |el| Metadata::replacer( el, html, html_path, path_from_output_to_assets)),
                element!(Title::tag(), |el| replacer::<Title>(el, html, html_path)),
                element!(Icon::tag(), |el| replacer::<Icon>(el, html, html_path)),
                element!(Footer::tag(), |el| replacer::<Footer>(el, html, html_path)),
                element!(Alert::tag(), |el| replacer::<Alert>(el, html, html_path)),
                element!(Hr::tag(), |el| replacer::<Hr>(el, html, html_path)),
                element!(FormAlert::tag(), |el| replacer::<FormAlert>(el, html, html_path)),
                element!(FormTextInput::tag(), |el| replacer::<FormTextInput>(el, html, html_path)),
                element!(FormCheckInput::tag(), |el| replacer::<FormCheckInput>(el, html, html_path)),
                element!(FormSubmit::tag(), |el| replacer::<FormSubmit>(el, html, html_path)),
                not_found_handler,
            ],
            ..Settings::new()
        },
    )?;

    let html = NEWLINE_REGEX.replace_all(&html, " ");
    let html = GAP_REGEX.replace_all(&html, ">\n<");

    Ok(format!(
        "<!-- htmplate v{} -->\n{html}",
        env!("CARGO_PKG_VERSION")
    ))
}

/// Error variants for replacing the htmplates.
#[derive(Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum ReplaceHtmplateError {
    #[non_exhaustive]
    InvalidHtmplate { source: FromElementError },

    #[non_exhaustive]
    HtmplateError { source: HtmplateError },

    #[non_exhaustive]
    HtmplateDoesNotExist { tag: String, location: Location },

    #[non_exhaustive]
    RewriteError { source: RewritingError },
}
impl core::fmt::Display for ReplaceHtmplateError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::HtmplateDoesNotExist { tag, location, .. } => {
                write!(f, "htmplate `{tag}` at `{location}` does not exist")
            }
            Self::RewriteError { .. } => write!(f, "rewriting returned an error"),
            Self::InvalidHtmplate { .. } => write!(f, "syntax error"),
            Self::HtmplateError { .. } => write!(f, "error while replacing an htmplate"),
        }
    }
}
impl core::error::Error for ReplaceHtmplateError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        match &self {
            Self::InvalidHtmplate { source, .. } => Some(source),
            Self::RewriteError { source, .. } => Some(source),
            Self::HtmplateError { source } => Some(source),
            _ => None,
        }
    }
}
impl From<RewritingError> for ReplaceHtmplateError {
    fn from(source: RewritingError) -> Self {
        match source {
            RewritingError::ContentHandlerError(error) => {
                if error.is::<Self>() {
                    *error.downcast::<Self>().unwrap()
                } else if error.is::<FromElementError>() {
                    Self::InvalidHtmplate {
                        source: *error.downcast::<FromElementError>().unwrap(),
                    }
                } else {
                    Self::RewriteError {
                        source: RewritingError::ContentHandlerError(error),
                    }
                }
            }
            _ => Self::RewriteError { source },
        }
    }
}
