use std::cmp::PartialEq;

use crate::document::InterpretedValue;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TInterpretedValue {
    Value(&'static str),
    Set,
    Unset,
}

impl PartialEq<InterpretedValue> for TInterpretedValue {
    fn eq(&self, other: &InterpretedValue) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TInterpretedValue> for InterpretedValue {
    fn eq(&self, other: &TInterpretedValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TInterpretedValue> for &InterpretedValue {
    fn eq(&self, other: &TInterpretedValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TInterpretedValue, observed: &InterpretedValue) -> bool {
    match fixture {
        TInterpretedValue::Value(fixture_value) => {
            if let InterpretedValue::Value(observed_value) = observed {
                fixture_value == observed_value
            } else {
                false
            }
        }

        TInterpretedValue::Set => observed == &InterpretedValue::Set,
        TInterpretedValue::Unset => observed == &InterpretedValue::Unset,
    }
}
