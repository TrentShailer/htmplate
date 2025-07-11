use core::error::Error;

use lol_html::html_content::{ContentType, Element};

use crate::{
    htmplates::{Attribute, Htmplate, HtmplateError},
    icon::get_icon_svg,
};

/// Title htmplate
pub struct Title;
impl Htmplate for Title {
    fn tag(&self) -> &'static str {
        "htmplate\\:title"
    }

    fn replace(&self, el: &mut Element) -> Result<(), Box<dyn Error + Send + Sync>> {
        let icon = if let Some(icon) = el.get_attribute("icon") {
            get_icon_svg(&icon).ok_or_else(|| HtmplateError::invalid_icon(icon))?
        } else {
            ""
        };

        let text = el.get_attribute("text").unwrap_or_default();

        let content = format!(
            r#"<hgroup style="display: flex; flex-direction: row; align-items: center; gap: 1rem">
                {icon}<h1>{text}</h1>
            </hgroup>"#
        );

        el.start_tag().remove();
        el.before(&content, ContentType::Html);

        Ok(())
    }

    fn attributes(&self) -> Vec<Attribute> {
        vec![
            Attribute {
                name: "icon",
                value_description: "an identifier for a filled ionicon https://ionic.io/ionicons",
                required: false,
            },
            Attribute {
                name: "text",
                value_description: "the title text",
                required: false,
            },
        ]
    }

    fn description(&self) -> &'static str {
        "a document title with an optional icon"
    }
}
