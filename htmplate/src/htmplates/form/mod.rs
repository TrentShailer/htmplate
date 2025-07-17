mod alert;
mod submit;
mod text_input;

use core::str::FromStr;

pub use alert::FormAlert;
pub use submit::FormSubmit;
pub use text_input::FormTextInput;

#[derive(Debug, Clone)]
pub struct FormId(pub String);
impl FromStr for FormId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("/") {
            return Err(());
        }

        Ok(Self(s.to_string()))
    }
}
