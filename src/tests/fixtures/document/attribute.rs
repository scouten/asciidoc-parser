use std::{cmp::PartialEq, fmt};

use crate::{
    document::Attribute,
    tests::fixtures::{document::TRawAttributeValue, TSpan},
    HasSpan,
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

fn fixture_eq_observed(tattribute: &TAttribute, attribute: &Attribute) -> bool {
    if &tattribute.source != attribute.span() {
        return false;
    }

    if &tattribute.name != attribute.name() {
        return false;
    }

    if &tattribute.value != attribute.raw_value() {
        return false;
    }

    true
}
