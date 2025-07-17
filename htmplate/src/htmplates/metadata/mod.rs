use std::path::Path;

use htmplate_derive::HtmplateElement;
use lol_html::html_content::ContentType;

use crate::HtmplateElement;

use crate as htmplate;

#[derive(HtmplateElement)]
/// common document metadata
pub struct Metadata;
impl Metadata {
    /// Convert the metadata htmplate to HTML.
    pub fn to_html(
        self,
        path_from_output_to_assets: &Path,
    ) -> Result<String, super::HtmplateErrorKind> {
        let favicon_path = path_from_output_to_assets.join("favicon.ico");
        let css_path = path_from_output_to_assets.join("style.min.css");

        Ok(format!(
            include_str!("template.html"),
            favicon = favicon_path.to_string_lossy().replace("\\", "/"),
            css = css_path.to_string_lossy().replace("\\", "/")
        ))
    }

    /// Create a custom replacer for Metadata.
    pub fn replacer(
        el: &mut lol_html::html_content::Element,
        html: &str,
        path: &Path,
        path_from_output_to_assets: &Path,
    ) -> Result<(), Box<dyn core::error::Error + Send + Sync + 'static>> {
        let htmplate = Self::from_element(el, html, path)?;
        let html = htmplate.to_html(path_from_output_to_assets)?;

        el.start_tag().remove();
        el.before(&html, ContentType::Html);

        Ok(())
    }
}
