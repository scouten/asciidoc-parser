use std::{cmp::PartialEq, fmt};

use crate::{
    attributes::Attrlist,
    tests::fixtures::{attributes::TElementAttribute, TSpan},
    HasSpan,
};

#[derive(Eq, PartialEq)]
pub(crate) struct TAttrlist {
    pub attributes: Vec<TElementAttribute>,
    pub source: TSpan,
}

impl fmt::Debug for TAttrlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attrlist")
            .field("attributes", &self.attributes)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<Attrlist<'src>> for TAttrlist {
    fn eq(&self, other: &Attrlist<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TAttrlist> for Attrlist<'_> {
    fn eq(&self, other: &TAttrlist) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TAttrlist> for &Attrlist<'_> {
    fn eq(&self, other: &TAttrlist) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TAttrlist, observed: &Attrlist) -> bool {
    if &fixture.source != observed.span() {
        return false;
    }

    if fixture.attributes.len() != observed.attributes().len() {
        return false;
    }

    for (fixture_attr, observed_attr) in fixture.attributes.iter().zip(observed.attributes()) {
        if fixture_attr != observed_attr {
            return false;
        }
    }

    true
}
