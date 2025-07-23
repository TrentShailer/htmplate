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
    /// this should be if the alert starts collapsed
    pub hidden: Option<bool>,
}

impl ToHtml for Alert {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self {
            status,
            text,
            hidden,
        } = self;

        let hidden = if hidden.is_some_and(|v| v) {
            "collapse"
        } else {
            ""
        };

        let icon = match &status {
            AlertStyle::Error => Icon::AlertCircle.svg(),
            AlertStyle::Warning => Icon::Warning.svg(),
            AlertStyle::Success => Icon::CheckmarkCircle.svg(),
            AlertStyle::Info => Icon::HelpCircle.svg(),
            AlertStyle::Basic => Icon::InformationCircle.svg(),
        };

        let style = match &status {
            AlertStyle::Error => "error",
            AlertStyle::Warning => "warning",
            AlertStyle::Success => "success",
            AlertStyle::Info => "info",
            AlertStyle::Basic => "",
        };

        let html = format!(
            include_str!("template.html"),
            style = style,
            icon = icon,
            text = text.unwrap_or_default(),
            hidden = hidden
        );

        Ok(html)
    }
}
