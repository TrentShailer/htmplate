//! # `htmplate`
//! Library to make reusable components in HTML via `<htmplate:... />` elements.

mod htmplate_element;
pub mod htmplates;
mod icon;

use lol_html::{Settings, element, errors::RewritingError, rewrite_str};

pub use htmplate_element::{Attribute, FromElementError, HtmplateElement, Location};
pub use lol_html;

use crate::htmplates::{Alert, Footer, HtmplateError, Metadata, Title, replacer};

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
    ]
}

/// Replace the htmplates in some source HTML.
pub fn replace_htmplates(html: &str) -> Result<String, RewritingError> {
    let tags: Vec<_> = all_htmplate_details()
        .into_iter()
        .map(|detail| detail.tag)
        .collect();

    let not_found_handler = {
        let not_selectors: String = tags.iter().map(|tag| format!(":not({tag})")).collect();

        element!(format!("*{not_selectors}"), |el| {
            if el.tag_name().starts_with("htmplate") {
                Err(Box::new(HtmplateError::HtmplateNotFound {
                    tag: el.tag_name(),
                }))
            } else {
                Ok(())
            }
        })
    };

    rewrite_str(
        html,
        Settings {
            element_content_handlers: vec![
                element!(Title::tag(), replacer::<Title>),
                element!(Metadata::tag(), replacer::<Metadata>),
                element!(Footer::tag(), replacer::<Footer>),
                element!(Alert::tag(), replacer::<Alert>),
                not_found_handler,
            ],
            ..Settings::new()
        },
    )
}
