use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateError, ToHtml},
    icon::Icon,
};

use crate as htmplate;

#[derive(HtmplateElement)]
/// a document title with an optional icon
pub struct Title {
    /// the title text
    pub text: Option<String>,
    /// an identifier for a filled ionicon https://ionic.io/ionicons
    pub icon: Option<Icon>,
}

impl ToHtml for Title {
    fn to_html(self) -> Result<String, HtmplateError> {
        let icon = self.icon.map(|icon| icon.svg()).unwrap_or_default();
        let text = self.text.unwrap_or_default();

        let content = format!(
            r#"<hgroup style="display: flex; flex-direction: row; align-items: center; gap: 1rem">
                {icon}<h1>{text}</h1>
            </hgroup>"#
        );

        Ok(content)
    }
}
