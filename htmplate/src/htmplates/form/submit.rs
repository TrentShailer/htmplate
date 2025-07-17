use htmplate_derive::HtmplateElement;

use crate::{
    self as htmplate,
    htmplates::{HtmplateErrorKind, ToHtml, form::FormId},
};

#[derive(HtmplateElement)]
/// the submit button for a form
pub struct FormSubmit {
    /// this should be the ID of the form this submits, must start with a `/`
    pub form: FormId,
}

impl ToHtml for FormSubmit {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self { form: FormId(form) } = self;

        Ok(format!(include_str!("submit.html"), form = form))
    }
}
