use htmplate_derive::HtmplateElement;

use crate::{self as htmplate, htmplates::ToHtml};

#[derive(HtmplateElement)]
/// the submit button for a form
pub struct FormSubmit;

impl ToHtml for FormSubmit {
    fn to_html(self) -> Result<String, super::HtmplateError> {
        Ok(r#"
            <section aria-label="submit the form">
                <button id="submit" type="submit">
                    Submit
                </button>
            </section>"#
            .to_string())
    }
}
