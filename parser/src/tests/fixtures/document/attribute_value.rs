use std::cmp::PartialEq;

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum InterpretedValue {
    Value(&'static str),
    Set,
    Unset,
}

impl PartialEq<crate::document::InterpretedValue> for InterpretedValue {
    fn eq(&self, other: &crate::document::InterpretedValue) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<InterpretedValue> for crate::document::InterpretedValue {
    fn eq(&self, other: &InterpretedValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<InterpretedValue> for &crate::document::InterpretedValue {
    fn eq(&self, other: &InterpretedValue) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(
    fixture: &InterpretedValue,
    observed: &crate::document::InterpretedValue,
) -> bool {
    match fixture {
        InterpretedValue::Value(fixture_value) => {
            if let crate::document::InterpretedValue::Value(observed_value) = observed {
                fixture_value == observed_value
            } else {
                false
            }
        }

        InterpretedValue::Set => observed == &crate::document::InterpretedValue::Set,
        InterpretedValue::Unset => observed == &crate::document::InterpretedValue::Unset,
    }
}
