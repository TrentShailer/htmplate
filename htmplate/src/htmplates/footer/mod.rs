use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateErrorKind, ToHtml},
    icon::Icon,
};

use crate as htmplate;

#[derive(HtmplateElement)]
/// the shared footer
pub struct Footer;
impl ToHtml for Footer {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let icon = Icon::LogoGithub.svg();

        Ok(format!(include_str!("template.html"), icon = icon))
    }
}
