use std::{cmp::PartialEq, fmt};

use crate::Span;

#[derive(Eq, PartialEq)]
pub(crate) struct TSpan {
    pub data: &'static str,
    pub line: usize,
    pub col: usize,
    pub offset: usize,
}

impl fmt::Debug for TSpan {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("data", &self.data)
            .field("line", &self.line)
            .field("col", &self.col)
            .field("offset", &self.offset)
            .finish()
    }
}

impl<'src> PartialEq<Span<'src>> for TSpan {
    fn eq(&self, other: &Span<'src>) -> bool {
        tspan_eq(self, other)
    }
}

impl<'src> PartialEq<TSpan> for Span<'src> {
    fn eq(&self, other: &TSpan) -> bool {
        tspan_eq(other, self)
    }
}

impl<'src> PartialEq<TSpan> for &Span<'src> {
    fn eq(&self, other: &TSpan) -> bool {
        tspan_eq(other, self)
    }
}

fn tspan_eq(tspan: &TSpan, span: &Span) -> bool {
    tspan.data == span.data()
        && tspan.line == span.line()
        && tspan.col == span.col()
        && tspan.offset == span.byte_offset()
}
