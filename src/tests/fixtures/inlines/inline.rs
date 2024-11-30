use crate::{
    inlines::Inline,
    tests::fixtures::{inlines::TInlineMacro, span::TSpan},
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TInline {
    Uninterpreted(TSpan),
    Sequence(Vec<Self>, TSpan),
    Macro(TInlineMacro),
}

impl<'src> PartialEq<Inline<'src>> for TInline {
    fn eq(&self, other: &Inline<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TInline> for Inline<'_> {
    fn eq(&self, other: &TInline) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TInline, observed: &Inline) -> bool {
    match fixture {
        TInline::Uninterpreted(ref fixture_span) => match observed {
            Inline::Uninterpreted(ref observed_span) => fixture_span == observed_span,
            _ => false,
        },

        TInline::Sequence(ref fixture_inlines, ref fixture_span) => match observed {
            Inline::Sequence(ref observed_inlines, ref observed_span) => {
                if fixture_inlines.len() != observed_inlines.len() {
                    return false;
                }

                for (fixture, observed) in fixture_inlines.iter().zip(observed_inlines.iter()) {
                    if fixture != observed {
                        return false;
                    }
                }

                fixture_span == observed_span
            }
            _ => false,
        },

        TInline::Macro(ref fixture_macro) => match observed {
            Inline::Macro(ref observed_macro) => fixture_macro == observed_macro,
            _ => false,
        },
    }
}
