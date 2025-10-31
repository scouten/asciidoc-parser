use std::fmt;

use crate::internal::debug::DebugSliceReference;

#[derive(Eq, PartialEq)]
pub(crate) struct SectionNumber {
    pub components: &'static [usize],
}

impl fmt::Debug for SectionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionNumber")
            .field("components", &DebugSliceReference(&self.components))
            .finish()
    }
}

impl PartialEq<crate::blocks::SectionNumber> for SectionNumber {
    fn eq(&self, other: &crate::blocks::SectionNumber) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<SectionNumber> for crate::blocks::SectionNumber {
    fn eq(&self, other: &SectionNumber) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &SectionNumber, observed: &crate::blocks::SectionNumber) -> bool {
    fixture.components == observed.components()
}
