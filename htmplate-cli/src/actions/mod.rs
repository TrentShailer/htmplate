use std::{fs, io, path::Path};

mod bundle;
mod template;
mod write_library;

fn file_exists_and_is_accessable(path: &Path) -> io::Result<bool> {
    if !fs::exists(path)? {
        return Ok(false);
    }

    if !path.metadata()?.is_file() {
        return Ok(false);
    }

    Ok(true)
}

pub use bundle::{BundleScriptError, bundle_script};
pub use template::{TemplateError, template_html};
pub use write_library::{WriteLibraryError, write_library};
