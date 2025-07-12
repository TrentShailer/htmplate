use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateError, ToHtml},
    icon::get_icon_svg,
};

use crate as htmplate;

#[derive(HtmplateElement)]
/// a document title with an optional icon
pub struct Title {
    /// the title text
    pub text: Option<String>,
    /// an identifier for a filled ionicon https://ionic.io/ionicons
    pub icon: Option<String>,
}

impl ToHtml for Title {
    fn to_html(self) -> Result<String, HtmplateError> {
        let icon = if let Some(icon) = self.icon {
            get_icon_svg(&icon).ok_or_else(|| HtmplateError::invalid_icon(icon))?
        } else {
            ""
        };

        let text = self.text.unwrap_or_default();

        let content = format!(
            r#"<hgroup style="display: flex; flex-direction: row; align-items: center; gap: 1rem">
                {icon}<h1>{text}</h1>
            </hgroup>"#
        );

        Ok(content)
    }
}
