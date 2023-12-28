use std::cmp::PartialEq;

use crate::Span;

// Approximate mock of Span type that we can use
// to declare expected values for easier test writing.
//
// Primary difference is that the data members are public
// so we can declare them inline.
#[derive(Debug, Eq, PartialEq)]
pub(crate) struct TSpan {
    pub data: &'static str,
    pub line: usize,
    pub col: usize,
    pub offset: usize,
}

impl<'a> PartialEq<Span<'a>> for TSpan {
    fn eq(&self, other: &Span<'a>) -> bool {
        tspan_eq(self, other)
    }
}

impl<'a> PartialEq<TSpan> for Span<'a> {
    fn eq(&self, other: &TSpan) -> bool {
        tspan_eq(other, self)
    }
}

fn tspan_eq(tspan: &TSpan, span: &Span) -> bool {
    &tspan.data == span.data()
        && tspan.line == span.line()
        && tspan.col == span.col()
        && tspan.offset == span.byte_offset()
}
