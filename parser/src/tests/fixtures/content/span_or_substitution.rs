#![allow(unused)]

use crate::{
    content::SpanOrSubstitution,
    tests::fixtures::{content::TSubstitution, TSpan},
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TSpanOrSubstitution {
    Span(TSpan),
    Substitution(TSubstitution),
}

impl<'src> PartialEq<SpanOrSubstitution<'src>> for TSpanOrSubstitution {
    fn eq(&self, other: &SpanOrSubstitution<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TSpanOrSubstitution> for SpanOrSubstitution<'_> {
    fn eq(&self, other: &TSpanOrSubstitution) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TSpanOrSubstitution> for &SpanOrSubstitution<'_> {
    fn eq(&self, other: &TSpanOrSubstitution) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TSpanOrSubstitution, observed: &SpanOrSubstitution) -> bool {
    match fixture {
        TSpanOrSubstitution::Span(ref f_span) => match observed {
            &SpanOrSubstitution::Span(ref o_span) => f_span == o_span,
            _ => false,
        },

        TSpanOrSubstitution::Substitution(ref f_sub) => match observed {
            SpanOrSubstitution::Substitution(ref o_sub) => f_sub == o_sub,
            _ => false,
        },
    }
}
