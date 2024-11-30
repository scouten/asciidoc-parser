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
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TSpan> for Span<'_> {
    fn eq(&self, other: &TSpan) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TSpan> for &Span<'_> {
    fn eq(&self, other: &TSpan) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TSpan, span: &Span) -> bool {
    fixture.data == span.data()
        && fixture.line == span.line()
        && fixture.col == span.col()
        && fixture.offset == span.byte_offset()
}
