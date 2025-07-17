use htmplate_derive::HtmplateElement;

use crate::{
    self as htmplate,
    htmplates::{HtmplateErrorKind, ToHtml, form::FormId},
    icon::Icon,
};

#[derive(HtmplateElement)]
/// an alert for a form
pub struct FormAlert {
    /// this should be the ID of the form this is for, must start with a `/`
    pub form: FormId,
}

impl ToHtml for FormAlert {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self { form: FormId(form) } = self;
        let icon = Icon::AlertCircle.svg();

        Ok(format!(
            include_str!("alert.html"),
            form = form,
            icon = icon
        ))
    }
}
