//! The [`Parser`] struct and its related structs allow a caller to configure
//! how AsciiDoc parsing occurs and then to initiate the parsing process.
#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod attribute_value;
pub(crate) use attribute_value::AttributeValue;
pub use attribute_value::{AllowableValue, ModificationContext};

mod parser;
pub use parser::Parser;

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    #[test]
    fn what_happens() {
        assert_eq!(2 + 2, 4);

        if false {
            assert_eq!(2 + 2, 5);
        }
    }
}
