use std::path::PathBuf;

use htmplate_derive::HtmplateElement;

use crate::htmplates::ToHtml;

use crate as htmplate;

#[derive(HtmplateElement)]
/// common document metadata
pub struct Metadata {
    /// this should be the path to the root of the website where the library is located
    root: PathBuf,
}
impl ToHtml for Metadata {
    fn to_html(self) -> Result<String, super::HtmplateErrorKind> {
        let Self { root } = self;

        let favicon_path = root.join("lib").join("favicon.ico");
        let css_path = root.join("lib").join("style.min.css");

        Ok(format!(
            include_str!("template.html"),
            favicon = favicon_path.to_string_lossy().replace("\\", "/"),
            css = css_path.to_string_lossy().replace("\\", "/")
        ))
    }
}
