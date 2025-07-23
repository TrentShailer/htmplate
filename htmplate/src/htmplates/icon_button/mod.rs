use htmplate_derive::HtmplateElement;

use crate::{
    htmplates::{HtmplateErrorKind, ToHtml},
    icon::Icon,
};

use crate as htmplate;

#[derive(HtmplateElement)]
/// a button with an icon
pub struct IconButton {
    /// this should be the ID of the button
    pub id: Option<String>,
    /// this should be the button text
    pub text: Option<String>,
    /// this should be an identifier for a filled ionicon https://ionic.io/ionicons
    pub icon: Option<Icon>,
    /// the link this button redirects to
    pub href: Option<String>,
    /// should the link button open in a new tab, defaults to false
    pub new_tab: Option<bool>,
    /// is the button ghost styled
    pub ghost: Option<bool>,
}

impl ToHtml for IconButton {
    fn to_html(self) -> Result<String, HtmplateErrorKind> {
        let Self {
            id,
            text,
            icon,
            href,
            new_tab,
            ghost,
        } = self;

        let circle_class = if text.is_none() { "circle" } else { "" };
        let ghost_class = if ghost.is_some_and(|v| v) {
            "ghost"
        } else {
            ""
        };

        let icon = icon.map(|icon| icon.svg()).unwrap_or_default();
        let text = text.unwrap_or_default();
        let id_attribute = id.map(|id| format!(r#"id="{id}""#)).unwrap_or_default();

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
                id_attribute = id_attribute,
                circle_class = circle_class,
                ghost_class = ghost_class
            )
        } else {
            format!(
                include_str!("button.template.html"),
                icon = icon,
                text = text,
                id_attribute = id_attribute,
                circle_class = circle_class,
                ghost_class = ghost_class
            )
        };

        Ok(content)
    }
}
