use core::error::Error;

use lol_html::html_content::{ContentType, Element};

use crate::{
    htmplates::{Htmplate, HtmplateError},
    icon::get_icon_svg,
};

/// Footer htmplate
pub struct Footer;
impl Htmplate for Footer {
    fn tag(&self) -> &'static str {
        "htmplate\\:footer"
    }

    fn replace(&self, el: &mut Element) -> Result<(), Box<dyn Error + Send + Sync>> {
        let icon = get_icon_svg("logo-github")
            .ok_or_else(|| HtmplateError::invalid_icon("logo-github"))?;

        let content = format!(
            r#"<footer>
                <a
                    class="button"
                    href="https://github.com/trentshailer"
                    target="_blank"
                    rel="noopener noreferrer"
                    aria-label="Link to Trent Shailer's Git Hub"
                >
                    {icon}Made by Trent Shailer
                </a>
            </footer>"#
        );

        el.start_tag().remove();
        el.before(&content, ContentType::Html);

        Ok(())
    }

    fn attributes(&self) -> Vec<super::Attribute> {
        vec![]
    }

    fn description(&self) -> &'static str {
        "the shared footer"
    }
}
