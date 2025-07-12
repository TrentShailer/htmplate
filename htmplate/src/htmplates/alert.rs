use core::str::FromStr;

use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateError, ToHtml},
    icon::get_icon_svg,
};

use crate as htmplate;

#[derive(Clone, Debug)]
pub enum AlertStyle {
    Error,
    Warning,
    Success,
    Info,
    Basic,
}
impl FromStr for AlertStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "error" => Ok(Self::Error),
            "warning" => Ok(Self::Warning),
            "success" => Ok(Self::Success),
            "info" => Ok(Self::Info),
            "basic" => Ok(Self::Basic),
            _ => Err(()),
        }
    }
}

#[derive(HtmplateElement)]
/// an admonition style alert.
pub struct Alert {
    /// one of [error, warning, success, info, basic]
    pub status: AlertStyle,
    /// the alert text
    pub text: Option<String>,
}

impl ToHtml for Alert {
    fn to_html(self) -> Result<String, HtmplateError> {
        let icon = match &self.status {
            AlertStyle::Error => get_icon_svg("alert-circle").unwrap(),
            AlertStyle::Warning => get_icon_svg("warning").unwrap(),
            AlertStyle::Success => get_icon_svg("checkmark-circle").unwrap(),
            AlertStyle::Info => get_icon_svg("help-circle").unwrap(),
            AlertStyle::Basic => get_icon_svg("information-circle").unwrap(),
        };

        let class = match &self.status {
            AlertStyle::Error => "error",
            AlertStyle::Warning => "warning",
            AlertStyle::Success => "success",
            AlertStyle::Info => "info",
            AlertStyle::Basic => "",
        };

        let text = self.text.unwrap_or_default();

        Ok(format!(
            r#"
            <div class="alert {class}">
                {icon}{text}
            </div>"#
        ))
    }
}
