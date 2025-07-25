use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateErrorKind, ToHtml, create_or_prepend_html_attribute},
    icon::Icon,
};

use crate as htmplate;

#[derive(HtmplateElement)]
/// a document title with an optional icon
pub struct Title {
    /// this should be the title text
    pub text: Option<String>,
    /// this should be an identifier for a filled ionicon https://ionic.io/ionicons
    pub icon: Option<Icon>,
}

impl ToHtml for Title {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let mut icon = self
            .icon
            .map(|icon| icon.svg())
            .unwrap_or_default()
            .to_string();
        create_or_prepend_html_attribute("class", "mauve", " ", &mut icon);
        let text = self.text.unwrap_or_default();

        let content = format!(include_str!("template.html"), icon = icon, text = text);

        Ok(content)
    }
}
