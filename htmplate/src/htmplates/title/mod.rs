use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateErrorKind, ToHtml},
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
        let icon = self.icon.map(|icon| icon.svg()).unwrap_or_default();
        let text = self.text.unwrap_or_default();

        let content = format!(include_str!("template.html"), icon = icon, text = text);

        Ok(content)
    }
}
