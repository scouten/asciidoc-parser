use std::cmp::PartialEq;

use crate::{document::AttributeValue, tests::fixtures::TSpan};

#[derive(Debug, Eq, PartialEq)]
#[allow(dead_code)] // TEMPORARY
pub(crate) enum TAttributeValue {
    Value(TSpan),
    Set,
    Unset,
}

impl<'a> PartialEq<AttributeValue<'a>> for TAttributeValue {
    fn eq(&self, other: &AttributeValue<'a>) -> bool {
        tattribute_value_eq(self, other)
    }
}

impl<'a> PartialEq<TAttributeValue> for AttributeValue<'a> {
    fn eq(&self, other: &TAttributeValue) -> bool {
        tattribute_value_eq(other, self)
    }
}

impl<'a> PartialEq<TAttributeValue> for &AttributeValue<'a> {
    fn eq(&self, other: &TAttributeValue) -> bool {
        tattribute_value_eq(other, self)
    }
}

fn tattribute_value_eq(
    tattribute_value: &TAttributeValue,
    attribute_value: &AttributeValue,
) -> bool {
    match tattribute_value {
        TAttributeValue::Value(ref v) => {
            if let AttributeValue::Value(ref av) = attribute_value {
                v == av
            } else {
                false
            }
        }

        TAttributeValue::Set => attribute_value == &AttributeValue::Set,

        TAttributeValue::Unset => attribute_value == &AttributeValue::Unset,
    }
}
