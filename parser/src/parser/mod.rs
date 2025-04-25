#![allow(unused)] // DO NOT MERGE
//! The [`Parser`] struct and its related structs allow a caller to configure
//! how AsciiDoc parsing occurs and then to initiate the parsing process.

mod attribute_value;
pub use attribute_value::AllowableValue;
pub(crate) use attribute_value::{AttributeValue, ModificationContext};

mod parser;
pub use parser::Parser;
