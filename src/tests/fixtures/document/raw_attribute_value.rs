use std::cmp::PartialEq;

use crate::{document::RawAttributeValue, tests::fixtures::TSpan};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TRawAttributeValue {
    Value(TSpan),
    Set,
    Unset,
}

impl<'src> PartialEq<RawAttributeValue<'src>> for TRawAttributeValue {
    fn eq(&self, other: &RawAttributeValue<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TRawAttributeValue> for RawAttributeValue<'_> {
    fn eq(&self, other: &TRawAttributeValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TRawAttributeValue> for &RawAttributeValue<'_> {
    fn eq(&self, other: &TRawAttributeValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(
    tattribute_value: &TRawAttributeValue,
    attribute_value: &RawAttributeValue,
) -> bool {
    match tattribute_value {
        TRawAttributeValue::Value(ref v) => {
            if let RawAttributeValue::Value(ref av) = attribute_value {
                v == av
            } else {
                false
            }
        }

        TRawAttributeValue::Set => attribute_value == &RawAttributeValue::Set,
        TRawAttributeValue::Unset => attribute_value == &RawAttributeValue::Unset,
    }
}
