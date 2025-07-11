use core::error::Error;

use lol_html::html_content::{ContentType, Element};

use crate::htmplates::Htmplate;

/// Metadata htmplate
pub struct Metadata;
impl Htmplate for Metadata {
    const TAG: &str = "htmplate\\:metadata";

    fn replace(el: &mut Element) -> Result<(), Box<dyn Error + Send + Sync>> {
        const CONTENT: &str = r#"
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta name="author" content="Trent Shailer">"#;

        el.start_tag().remove();
        el.before(CONTENT, ContentType::Html);

        Ok(())
    }
}
