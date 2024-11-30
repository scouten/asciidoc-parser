use std::{cmp::PartialEq, fmt};

use crate::{
    tests::fixtures::span::TSpan,
    warnings::{Warning, WarningType},
};

#[derive(Eq, PartialEq)]
pub(crate) struct TWarning {
    pub source: TSpan,
    pub warning: WarningType,
}

impl fmt::Debug for TWarning {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Warning")
            .field("source", &self.source)
            .field("warning", &self.warning)
            .finish()
    }
}

impl<'src> PartialEq<Warning<'src>> for TWarning {
    fn eq(&self, other: &Warning<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TWarning> for Warning<'_> {
    fn eq(&self, other: &TWarning) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TWarning> for &Warning<'_> {
    fn eq(&self, other: &TWarning) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(twarning: &TWarning, warning: &Warning) -> bool {
    twarning.source == warning.source && twarning.warning == warning.warning
}
