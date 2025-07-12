use htmplate_derive::HtmplateElement;

use crate::{self as htmplate, htmplates::ToHtml, icon::Icon};

#[derive(HtmplateElement)]
/// an alert for a form
pub struct FormAlert {
    /// the ID of the form this alert is for
    pub form_id: String,
}

impl ToHtml for FormAlert {
    fn to_html(self) -> Result<String, super::HtmplateError> {
        let form_id = self.form_id;
        let icon = Icon::AlertCircle.svg();

        Ok(format!(
            r#"
            <div class="alert error collapse" aria-hidden="true" id="{form_id}.error">
                <div>
                    {icon}
                </div>
                <div>
                    <div id="{form_id}.error.content"></div>
                </div>
            </div>"#
        ))
    }
}
