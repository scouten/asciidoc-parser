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
        tattribute_value_eq(self, other)
    }
}

impl<'src> PartialEq<TRawAttributeValue> for RawAttributeValue<'src> {
    fn eq(&self, other: &TRawAttributeValue) -> bool {
        tattribute_value_eq(other, self)
    }
}

impl<'src> PartialEq<TRawAttributeValue> for &RawAttributeValue<'src> {
    fn eq(&self, other: &TRawAttributeValue) -> bool {
        tattribute_value_eq(other, self)
    }
}

fn tattribute_value_eq(
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
