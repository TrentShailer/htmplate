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
    ) -> Result<String, super::HtmplateError> {
        let icon_path = path_from_output_to_assets.join("favicon.ico");
        let css_path = path_from_output_to_assets.join("lib.min.css");

        Ok(format!(
            r#"
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta name="author" content="Trent Shailer">
            <link rel="icon" type="image/x-icon" href="{}">
            <link rel="stylesheet" href="{}">"#,
            icon_path.to_string_lossy(),
            css_path.to_string_lossy()
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
