use std::{cmp::PartialEq, fmt};

use crate::{tests::fixtures::span::Span, warnings::WarningType};

#[derive(Eq, PartialEq)]
pub(crate) struct Warning {
    pub source: Span,
    pub warning: WarningType,
}

impl fmt::Debug for Warning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Warning")
            .field("source", &self.source)
            .field("warning", &self.warning)
            .finish()
    }
}

impl<'src> PartialEq<crate::warnings::Warning<'src>> for Warning {
    fn eq(&self, other: &crate::warnings::Warning<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Warning> for crate::warnings::Warning<'_> {
    fn eq(&self, other: &Warning) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<Warning> for &crate::warnings::Warning<'_> {
    fn eq(&self, other: &Warning) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Warning, observed: &crate::warnings::Warning) -> bool {
    fixture.source == observed.source && fixture.warning == observed.warning
}
