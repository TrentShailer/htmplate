use htmplate_derive::HtmplateElement;

use crate::{self as htmplate, htmplates::ToHtml, icon::Icon};

#[derive(HtmplateElement)]
/// an alert for a form
pub struct FormAlert;

impl ToHtml for FormAlert {
    fn to_html(self) -> Result<String, super::HtmplateError> {
        let icon = Icon::AlertCircle.svg();

        Ok(format!(
            r#"
            <div class="alert error collapse" aria-hidden="true" id="form.error">
                <div>
                    {icon}
                </div>
                <div>
                    <div id="form.error.content"></div>
                </div>
            </div>"#
        ))
    }
}
