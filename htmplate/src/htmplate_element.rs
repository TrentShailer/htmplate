use std::path::PathBuf;

/// A trait marking a struct as an htmplate.
pub trait HtmplateElement: Sized {
    /// Try convert an element to this htmplate.
    fn from_element(el: &lol_html::html_content::Element) -> Result<Self, FromElementError>;

    /// Get the template's attributes
    fn attributes() -> Vec<Attribute>;

    /// Get the htmplate's tag
    fn tag() -> &'static str;

    /// Get the htmplate's description
    fn description() -> &'static str;
}

/// An attribute on an htmplate.
#[derive(Debug, Clone, Copy)]
pub struct Attribute {
    /// The attribute name.
    pub name: &'static str,
    /// A description of the attribute, should flow on from "this should be ..."
    pub description: &'static str,
    /// If the attribute is required.
    pub required: bool,
}

#[derive(Debug, Clone)]
/// A location in a file.
pub enum Location {
    /// A given byte index.
    Byte(usize),
    /// A position in the file at the path.
    FilePosition {
        /// The path to the file.
        path: PathBuf,
        /// The line number.
        line: usize,
        /// The column
        column: usize,
    },
}
impl Location {
    /// Convert a byte position to a file position.
    pub fn as_file_position(&self, raw_file: &[u8], path: PathBuf) -> Self {
        let Self::Byte(index) = self else {
            return self.clone();
        };

        let mut consumed = 0;
        let mut line = 1;
        let mut column = 1;

        let mut iter = raw_file.iter();
        while consumed < *index
            && let Some(byte) = iter.next()
        {
            consumed += 1;
            match byte {
                b'\n' => {
                    line += 1;
                    column = 1;
                }
                _ => column += 1,
            }
        }

        Self::FilePosition { path, line, column }
    }
}
impl core::fmt::Display for Location {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self {
            Self::Byte(index) => write!(f, "byte {index}"),
            Self::FilePosition { path, line, column } => {
                write!(f, "{}:{}:{}", path.to_string_lossy(), line, column)
            }
        }
    }
}

/// Error for converting an element to an HtmplateElement.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct FromElementError {
    pub missing_attributes: Vec<Attribute>,
    pub invalid_attributes: Vec<Attribute>,
    pub element_tag: String,
    pub element_location: Location,
}

impl core::fmt::Display for FromElementError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "invalid `{}` at {}",
            self.element_tag, self.element_location
        )?;

        for attribute in &self.missing_attributes {
            writeln!(
                f,
                "  missing required attribute `{}`, this should be {}",
                attribute.name, attribute.description
            )?;
        }

        for attribute in &self.invalid_attributes {
            writeln!(
                f,
                "  invalid attribute `{}`, this should be {}",
                attribute.name, attribute.description
            )?;
        }

        Ok(())
    }
}
impl core::error::Error for FromElementError {
    fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
        None
    }
}
