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
        TInline::Uninterpreted(ref tspan) => match observed {
            Inline::Uninterpreted(ref span) => tspan == span,
            _ => false,
        },

        TInline::Sequence(ref tinlines, ref tspan) => match observed {
            Inline::Sequence(ref inlines, ref span) => {
                if tinlines.len() != inlines.len() {
                    return false;
                }

                for (fixture, observed) in tinlines.iter().zip(inlines.iter()) {
                    if fixture != observed {
                        return false;
                    }
                }

                tspan == span
            }
            _ => false,
        },

        TInline::Macro(ref tinline_macro) => match observed {
            Inline::Macro(ref inline_macro) => tinline_macro == inline_macro,
            _ => false,
        },
    }
}
