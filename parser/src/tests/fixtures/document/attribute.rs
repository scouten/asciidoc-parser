use std::{cmp::PartialEq, fmt};

use crate::{
    HasSpan,
    document::Attribute,
    tests::fixtures::{TSpan, document::TInterpretedValue},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TAttribute {
    pub name: TSpan,
    pub value_source: Option<TSpan>,
    pub value: TInterpretedValue,
    pub source: TSpan,
}

impl fmt::Debug for TAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &self.name)
            .field("value_source", &self.value_source)
            .field("value", &self.value)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<Attribute<'src>> for TAttribute {
    fn eq(&self, other: &Attribute<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TAttribute> for Attribute<'_> {
    fn eq(&self, other: &TAttribute) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TAttribute> for &Attribute<'_> {
    fn eq(&self, other: &TAttribute) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TAttribute, observed: &Attribute) -> bool {
    if fixture.source != observed.span() {
        return false;
    }

    if &fixture.name != observed.name() {
        return false;
    }

    if let Some(ref fixture_value_source) = fixture.value_source
        && let Some(observed_raw_value) = observed.raw_value()
        && fixture_value_source != &observed_raw_value
    {
        return false;
    }

    if fixture.value_source.is_some() != observed.raw_value().is_some() {
        return false;
    }

    if &fixture.value != observed.value() {
        return false;
    }

    true
}
