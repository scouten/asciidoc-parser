use std::{cmp::PartialEq, fmt};

use crate::{
    HasSpan,
    tests::fixtures::{Span, attributes::TElementAttribute},
};

#[derive(Eq, PartialEq)]
pub(crate) struct Attrlist {
    pub attributes: &'static [TElementAttribute],
    pub source: Span,
}

impl fmt::Debug for Attrlist {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Attrlist")
            .field("attributes", &self.attributes)
            .field("source", &self.source)
            .finish()
    }
}

impl<'src> PartialEq<crate::attributes::Attrlist<'src>> for Attrlist {
    fn eq(&self, other: &crate::attributes::Attrlist<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Attrlist> for crate::attributes::Attrlist<'_> {
    fn eq(&self, other: &Attrlist) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<Attrlist> for &crate::attributes::Attrlist<'_> {
    fn eq(&self, other: &Attrlist) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Attrlist, observed: &crate::attributes::Attrlist) -> bool {
    if fixture.source != observed.span() {
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
