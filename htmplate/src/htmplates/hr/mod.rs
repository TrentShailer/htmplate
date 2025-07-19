use htmplate_derive::HtmplateElement;

use crate::htmplates::{HtmplateErrorKind, ToHtml};

use crate as htmplate;

#[derive(HtmplateElement)]
/// a horizontal divider with some text content
pub struct Hr {
    /// this should be the text in the middle of the divider
    pub text: String,
}
impl ToHtml for Hr {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self { text } = self;
        let html = format!(include_str!("template.html"), text = text);
        Ok(html)
    }
}
