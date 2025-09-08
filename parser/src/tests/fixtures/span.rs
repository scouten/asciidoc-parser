use std::{cmp::PartialEq, fmt};

#[derive(Eq, PartialEq)]
pub(crate) struct Span {
    pub data: &'static str,
    pub line: usize,
    pub col: usize,
    pub offset: usize,
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Span")
            .field("data", &self.data)
            .field("line", &self.line)
            .field("col", &self.col)
            .field("offset", &self.offset)
            .finish()
    }
}

impl<'src> PartialEq<crate::Span<'src>> for Span {
    fn eq(&self, other: &crate::Span<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<Span> for crate::Span<'_> {
    fn eq(&self, other: &Span) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<Span> for &crate::Span<'_> {
    fn eq(&self, other: &Span) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &Span, observed: &crate::Span) -> bool {
    fixture.data == observed.data()
        && fixture.line == observed.line()
        && fixture.col == observed.col()
        && fixture.offset == observed.byte_offset()
}
