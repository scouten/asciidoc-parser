use std::{cmp::PartialEq, fmt};

use crate::document::RefType;

#[derive(Eq, PartialEq)]
pub(crate) struct RefEntry {
    pub(crate) id: &'static str,
    pub(crate) reftext: Option<&'static str>,
    pub(crate) ref_type: RefType,
}

impl fmt::Debug for RefEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("crate::document::RefEntry")
            .field("id", &self.id)
            .field("reftext", &self.reftext)
            .field("ref_type", &self.ref_type)
            .finish()
    }
}

impl PartialEq<crate::document::RefEntry> for RefEntry {
    fn eq(&self, other: &crate::document::RefEntry) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<RefEntry> for crate::document::RefEntry {
    fn eq(&self, other: &RefEntry) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<RefEntry> for &crate::document::RefEntry {
    fn eq(&self, other: &RefEntry) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &RefEntry, observed: &crate::document::RefEntry) -> bool {
    fixture.id == observed.id
        && fixture.reftext == observed.reftext.as_deref()
        && fixture.ref_type == observed.ref_type
}
