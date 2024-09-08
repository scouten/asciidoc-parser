#![allow(dead_code)] // TEMPORARY while building

use super::Span;

impl<'a> Span<'a> {
    /// Return a new span, discarding the first `n` characters in the input
    /// span.
    pub(crate) fn discard(self, n: usize) -> Self {
        self.into_parse_result(n).rem
    }

    /// Return a new span, discarding all characters in the input span.
    pub(crate) fn discard_all(self) -> Self {
        self.discard(self.len())
    }
}
