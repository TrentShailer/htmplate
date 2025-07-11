//! # `htmplate`
//! Library to make reusable components in HTML via `<htmplate:... />` elements.

pub mod htmplates;
mod icon;

use lol_html::{Settings, element, errors::RewritingError, rewrite_str};

use crate::htmplates::{HtmplateError, get_all_htmplates};

/// Replace the htmplates in some source HTML.
pub fn replace_htmplates(html: &str) -> Result<String, RewritingError> {
    let all_htmplates = get_all_htmplates();

    let mut handlers: Vec<_> = all_htmplates
        .iter()
        .map(|htmplate| element!(htmplate.tag(), |el| htmplate.replace(el)))
        .collect();

    let not_found_handler = {
        let not_selectors: String = all_htmplates
            .iter()
            .map(|htmplate| format!(":not({})", htmplate.tag()))
            .collect();

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

    handlers.push(not_found_handler);

    rewrite_str(
        html,
        Settings {
            element_content_handlers: handlers,

            ..Settings::new()
        },
    )
}
