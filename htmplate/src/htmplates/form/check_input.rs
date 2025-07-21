use htmplate_derive::HtmplateElement;

use crate::{
    self as htmplate,
    htmplates::{HtmplateErrorKind, ToHtml, form::FormId},
};

#[derive(HtmplateElement)]
/// a checkbox input for a form
pub struct FormCheckInput {
    /// this should be the id of the input, must start with a `/`
    pub id: FormId,
    /// this should be the id of the form, must start with a `/`
    pub form: FormId,
    /// this should be the input label contents
    pub label: String,
    /// this should be "true" if the input required
    pub required: Option<bool>,
}

impl ToHtml for FormCheckInput {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self {
            id: FormId(id),
            form: FormId(form),
            required,
            label,
        } = self;

        let is_required = required.is_some_and(|required| required);
        let required_marker = if is_required {
            r#"<span aria-hidden="true"><strong>*</strong></span>"#
        } else {
            ""
        };

        let required_attribute = if is_required { "required" } else { "" };

        let label_id = format!("{form}{id}/label");
        let input_id = format!("{form}{id}/input");

        Ok(format!(
            include_str!("check_input.html"),
            label_id = label_id,
            input_id = input_id,
            label = label,
            required_marker = required_marker,
            required_attribute = required_attribute,
        ))
    }
}
