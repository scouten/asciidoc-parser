use std::cmp::PartialEq;

use crate::document::AttributeValue;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TAttributeValue {
    Value(&'static str),
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
                v == &av.as_ref()
            } else {
                false
            }
        }

        TAttributeValue::Set => attribute_value == &AttributeValue::Set,
        TAttributeValue::Unset => attribute_value == &AttributeValue::Unset,
    }
}
