use crate::{content::Substitution, tests::fixtures::TSpan};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TSubstitution {
    pub original: TSpan,
    pub replacement: &'static str,
}

impl<'src> PartialEq<Substitution<'src>> for TSubstitution {
    fn eq(&self, other: &Substitution<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TSubstitution> for Substitution<'_> {
    fn eq(&self, other: &TSubstitution) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TSubstitution> for &Substitution<'_> {
    fn eq(&self, other: &TSubstitution) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TSubstitution, observed: &Substitution) -> bool {
    fixture.original == observed.original && fixture.replacement == observed.replacement
}
