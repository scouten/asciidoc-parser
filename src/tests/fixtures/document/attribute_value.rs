use std::cmp::PartialEq;

use crate::document::AttributeValue;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TAttributeValue {
    Value(&'static str),
    Set,
    Unset,
}

impl<'src> PartialEq<AttributeValue<'src>> for TAttributeValue {
    fn eq(&self, other: &AttributeValue<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TAttributeValue> for AttributeValue<'_> {
    fn eq(&self, other: &TAttributeValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TAttributeValue> for &AttributeValue<'_> {
    fn eq(&self, other: &TAttributeValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TAttributeValue, attribute_value: &AttributeValue) -> bool {
    match fixture {
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
