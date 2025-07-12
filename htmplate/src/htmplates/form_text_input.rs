use htmplate_derive::HtmplateElement;

use crate::{self as htmplate, htmplates::ToHtml};

#[derive(HtmplateElement)]
/// a text input for a form
pub struct FormTextInput {
    /// the id of the input
    pub id: String,
    /// is the input required
    pub required: bool,
    /// the input label contents
    pub label: String,
    /// the minimum number of characters
    pub min_length: Option<usize>,
    /// the maximum number of characters
    pub max_length: Option<usize>,
}

impl ToHtml for FormTextInput {
    fn to_html(self) -> Result<String, super::HtmplateError> {
        let Self {
            id,
            required,
            label,
            min_length,
            max_length,
        } = self;

        let aria_label = format!("{label} input");
        let required_marker = if required {
            "<strong><span>*</span></strong>"
        } else {
            ""
        };

        let required_attribute = if required { "required" } else { "" };
        let min_length_attribute = min_length
            .map(|length| format!("minlength=\"{length}\""))
            .unwrap_or_default();
        let max_length_attribute = max_length
            .map(|length| format!("maxlength=\"{length}\""))
            .unwrap_or_default();

        Ok(format!(
            r#"
            <section aria-label="{aria_label}">
                <label for="{id}">
                    {required_marker}
                    <span>{label}:</span>
                </label>
                <input
                    id="{id}"
                    type="text"
                    name="{id}"
                    {required_attribute}
                    {min_length_attribute}
                    {max_length_attribute}
                    placeholder="{label}"
                    />
                <small class="hidden" aria-hidden="true" id="{id}.error">!</small>
            </section>"#
        ))
    }
}
