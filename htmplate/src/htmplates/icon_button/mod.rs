use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateErrorKind, ToHtml},
    icon::Icon,
};

use crate as htmplate;

#[derive(HtmplateElement)]
/// a button with an icon
pub struct IconButton {
    /// this should be the button text
    pub text: Option<String>,
    /// this should be an identifier for a filled ionicon https://ionic.io/ionicons
    pub icon: Option<Icon>,
    /// the link this button redirects to
    pub href: Option<String>,
    /// should the link button open in a new tab, defaults to false
    pub new_tab: Option<bool>,
}

impl ToHtml for IconButton {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self {
            text,
            icon,
            href,
            new_tab,
        } = self;

        let circle_class = if text.is_none() { "circle" } else { "" };

        let icon = icon.map(|icon| icon.svg()).unwrap_or_default();
        let text = text.unwrap_or_default();

        let content = if let Some(href) = href {
            let new_tab_attributes = if let Some(new_tab) = new_tab
                && new_tab
            {
                r#"target="_blank" rel="noopener noreferrer""#
            } else {
                ""
            };
            format!(
                include_str!("link.template.html"),
                icon = icon,
                text = text,
                href = href,
                new_tab_attributes = new_tab_attributes,
                circle_class = circle_class,
            )
        } else {
            format!(
                include_str!("button.template.html"),
                icon = icon,
                text = text,
                circle_class = circle_class,
            )
        };

        Ok(content)
    }
}
