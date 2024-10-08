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
        tinline_eq(self, other)
    }
}

impl<'src> PartialEq<TInline> for Inline<'src> {
    fn eq(&self, other: &TInline) -> bool {
        tinline_eq(other, self)
    }
}

fn tinline_eq(tinline: &TInline, inline: &Inline) -> bool {
    match tinline {
        TInline::Uninterpreted(ref tspan) => match inline {
            Inline::Uninterpreted(ref span) => tspan == span,
            _ => false,
        },

        TInline::Sequence(ref tinlines, ref tspan) => match inline {
            Inline::Sequence(ref inlines, ref span) => {
                if tinlines.len() != inlines.len() {
                    return false;
                }

                for (tinline, inline) in tinlines.iter().zip(inlines.iter()) {
                    if tinline != inline {
                        return false;
                    }
                }

                tspan == span
            }
            _ => false,
        },

        TInline::Macro(ref tinline_macro) => match inline {
            Inline::Macro(ref inline_macro) => tinline_macro == inline_macro,
            _ => false,
        },
    }
}
