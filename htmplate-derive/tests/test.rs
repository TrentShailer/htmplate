#![allow(missing_docs)]

use core::str::FromStr;

use htmplate_derive::HtmplateElement;

#[derive(HtmplateElement)]
/// a test struct
pub struct TestStruct {
    /// a required string
    pub required_str: String,
    /// a required i8
    pub required_i8: i8,
    /// a required custom enum
    pub required_custom: Custom,
    /// an optional string
    pub optional_str: Option<String>,
    /// an optional i8
    pub optional_i8: Option<i8>,
    /// an optional custom enum
    pub optional_custom: Option<Custom>,
}

pub enum Custom {
    A,
    B,
    C,
}
impl FromStr for Custom {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            _ => Err(()),
        }
    }
}
