use std::{cmp::PartialEq, fmt};

use crate::{
    HasSpan,
    document::Attribute,
    tests::fixtures::{TSpan, document::TRawAttributeValue},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TAttribute {
    pub name: TSpan,
    pub value: TRawAttributeValue,
    pub source: TSpan,
}

impl fmt::Debug for TAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("name", &self.name)
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
    if &fixture.source != observed.span() {
        return false;
    }

    if &fixture.name != observed.name() {
        return false;
    }

    if &fixture.value != observed.raw_value() {
        return false;
    }

    true
}
