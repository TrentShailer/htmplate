use core::str::FromStr;

use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateError, ToHtml},
    icon::Icon,
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
            AlertStyle::Error => Icon::AlertCircle.svg(),
            AlertStyle::Warning => Icon::Warning.svg(),
            AlertStyle::Success => Icon::CheckmarkCircle.svg(),
            AlertStyle::Info => Icon::HelpCircle.svg(),
            AlertStyle::Basic => Icon::InformationCircle.svg(),
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
