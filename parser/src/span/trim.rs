use super::Span;

impl Span<'_> {
    /// Return a [`Span`] that is the same as the source, but with any trailing
    /// whitespace removed.
    pub(crate) fn trim_trailing_whitespace(&self) -> Self {
        let new_len = self.data.trim_end().len();
        self.slice(0..new_len)
    }
}
