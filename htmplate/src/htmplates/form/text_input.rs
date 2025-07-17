use htmplate_derive::HtmplateElement;

use crate::{
    self as htmplate,
    htmplates::{HtmplateErrorKind, ToHtml, form::FormId},
};

#[derive(HtmplateElement)]
/// a text input for a form
pub struct FormTextInput {
    /// this should be the id of the input, must start with a `/`
    pub id: FormId,
    /// this should be the id of the form, must start with a `/`
    pub form: FormId,
    /// this should be the input label contents
    pub label: String,
    /// this should be "true" if the input required
    pub required: Option<bool>,
    /// this should be "true" if the text input for a credential-like field
    pub credential: Option<bool>,
}

impl ToHtml for FormTextInput {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self {
            id: FormId(id),
            form: FormId(form),
            required,
            label,
            credential,
        } = self;

        let is_required = required.is_some_and(|required| required);
        let required_marker = if is_required {
            r#"<span aria-hidden="true"><strong>*</strong></span>"#
        } else {
            ""
        };

        let required_attribute = if is_required { "required" } else { "" };

        let mut extra_attributes: Vec<String> = Vec::new();
        if let Some(credential) = credential
            && credential
        {
            extra_attributes.push(r#"minlength="4""#.to_string());
            extra_attributes.push(r#"maxlength="64""#.to_string());
            extra_attributes.push(r#"autocapitalize="off""#.to_string());
            extra_attributes.push(r#"autocomplete="off""#.to_string());
        }
        let extra_attributes = extra_attributes.join("\n");

        let label_id = format!("{form}{id}/label");
        let input_id = format!("{form}{id}/input");
        let error_id = format!("{form}{id}/error");

        Ok(format!(
            include_str!("text_input.html"),
            label_id = label_id,
            input_id = input_id,
            error_id = error_id,
            label = label,
            required_marker = required_marker,
            required_attribute = required_attribute,
            extra_attributes = extra_attributes,
        ))
    }
}
