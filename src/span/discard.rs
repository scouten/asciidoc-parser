use super::Span;

impl<'src> Span<'src> {
    /// Return a new span, discarding the first `n` characters in the input
    /// span.
    pub(crate) fn discard(self, n: usize) -> Self {
        self.into_parse_result(n).after
    }

    /// Return a new span, discarding all characters in the input span.
    pub(crate) fn discard_all(self) -> Self {
        self.discard(self.len())
    }
}
