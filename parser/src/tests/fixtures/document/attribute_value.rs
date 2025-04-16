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

fn fixture_eq_observed(fixture: &TAttributeValue, observed: &AttributeValue) -> bool {
    match fixture {
        TAttributeValue::Value(ref fixture_value) => {
            if let AttributeValue::Value(ref observed_value) = observed {
                fixture_value == &observed_value.as_ref()
            } else {
                false
            }
        }

        TAttributeValue::Set => observed == &AttributeValue::Set,
        TAttributeValue::Unset => observed == &AttributeValue::Unset,
    }
}
