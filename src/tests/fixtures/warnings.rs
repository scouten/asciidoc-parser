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
        twarning_eq(self, other)
    }
}

impl<'src> PartialEq<TWarning> for Warning<'src> {
    fn eq(&self, other: &TWarning) -> bool {
        twarning_eq(other, self)
    }
}

impl<'src> PartialEq<TWarning> for &Warning<'src> {
    fn eq(&self, other: &TWarning) -> bool {
        twarning_eq(other, self)
    }
}

fn twarning_eq(twarning: &TWarning, warning: &Warning) -> bool {
    twarning.source == warning.source && twarning.warning == warning.warning
}
