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

impl<'a> PartialEq<Attribute<'a>> for TAttribute {
    fn eq(&self, other: &Attribute<'a>) -> bool {
        tattribute_eq(self, other)
    }
}

impl<'a> PartialEq<TAttribute> for Attribute<'a> {
    fn eq(&self, other: &TAttribute) -> bool {
        tattribute_eq(other, self)
    }
}

impl<'a> PartialEq<TAttribute> for &Attribute<'a> {
    fn eq(&self, other: &TAttribute) -> bool {
        tattribute_eq(other, self)
    }
}

fn tattribute_eq(tattribute: &TAttribute, attribute: &Attribute) -> bool {
    if &tattribute.source != attribute.span() {
        return false;
    }

    if &tattribute.name != attribute.name() {
        return false;
    }

    if &tattribute.value != attribute.value() {
        return false;
    }

    true
}
