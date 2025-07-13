use htmplate_derive::HtmplateElement;

use crate::htmplates::{HtmplateError, ToHtml};

use crate as htmplate;

#[derive(HtmplateElement)]
/// an SVG icon from https://ionic.io/ionicons
pub struct Icon {
    /// an identifier for a filled ionicon https://ionic.io/ionicons
    pub icon: crate::icon::Icon,
}
impl ToHtml for Icon {
    fn to_html(self) -> Result<String, HtmplateError> {
        Ok(self.icon.svg().to_string())
    }
}
