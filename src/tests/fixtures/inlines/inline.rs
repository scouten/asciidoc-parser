use crate::{inlines::Inline, tests::fixtures::span::TSpan};

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TInline {
    Uninterpreted(TSpan),
}

impl<'a> PartialEq<Inline<'a>> for TInline {
    fn eq(&self, other: &Inline<'a>) -> bool {
        tinline_eq(self, other)
    }
}

impl<'a> PartialEq<TInline> for Inline<'a> {
    fn eq(&self, other: &TInline) -> bool {
        tinline_eq(other, self)
    }
}

fn tinline_eq(tinline: &TInline, inline: &Inline) -> bool {
    match tinline {
        TInline::Uninterpreted(ref tspan) => match inline {
            Inline::Uninterpreted(ref span) => tspan == span,
            // _ => false,
        },
    }
}
