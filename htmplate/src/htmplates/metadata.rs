use htmplate_derive::HtmplateElement;

use crate::htmplates::ToHtml;

use crate as htmplate;

#[derive(HtmplateElement)]
/// common document metadata
pub struct Metadata;
impl ToHtml for Metadata {
    fn to_html(self) -> Result<String, super::HtmplateError> {
        Ok(r#"
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta name="author" content="Trent Shailer">"#
            .to_string())
    }
}
