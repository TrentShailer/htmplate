use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateError, ToHtml},
    icon::Icon,
};

use crate as htmplate;

#[derive(HtmplateElement)]
/// the shared footer
pub struct Footer;
impl ToHtml for Footer {
    fn to_html(self) -> Result<String, HtmplateError> {
        let icon = Icon::LogoGithub.svg();

        Ok(format!(
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
        ))
    }
}
