//! # `htmplate`
//! Library to make reusable components in HTML via `<htmplate:... />` elements.

pub mod htmplates;
mod icon;

use lol_html::{Settings, element, errors::RewritingError, rewrite_str};

use crate::htmplates::{Footer, Htmplate, Metadata, Title};

/// Replace the htmplates in some source HTML.
pub fn replace_htmplates(html: &str) -> Result<String, RewritingError> {
    rewrite_str(
        html,
        Settings {
            element_content_handlers: vec![
                element!(Metadata::TAG, Metadata::replace),
                element!(Title::TAG, Title::replace),
                element!(Footer::TAG, Footer::replace),
            ],

            ..Settings::new()
        },
    )
}
