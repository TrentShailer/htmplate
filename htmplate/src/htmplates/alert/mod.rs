use core::str::FromStr;

use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateErrorKind, ToHtml},
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
    /// this should be one of [error, warning, success, info, basic]
    pub status: AlertStyle,
    /// this should be the alert text
    pub text: Option<String>,
}

impl ToHtml for Alert {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self { status, text } = self;

        let icon = match &status {
            AlertStyle::Error => Icon::AlertCircle.svg(),
            AlertStyle::Warning => Icon::Warning.svg(),
            AlertStyle::Success => Icon::CheckmarkCircle.svg(),
            AlertStyle::Info => Icon::HelpCircle.svg(),
            AlertStyle::Basic => Icon::InformationCircle.svg(),
        };

        let class = match &status {
            AlertStyle::Error => "error",
            AlertStyle::Warning => "warning",
            AlertStyle::Success => "success",
            AlertStyle::Info => "info",
            AlertStyle::Basic => "",
        };

        let html = format!(
            include_str!("template.html"),
            class = class,
            icon = icon,
            text = text.unwrap_or_default()
        );

        Ok(html)
    }
}
