use super::Span;

impl<'src> Span<'src> {
    /// Return a [`Span`] that is the same as the source, but with any trailing
    /// whitespace removed.
    #[allow(dead_code)] // TEMPORARY until used
    pub(crate) fn trim_trailing_whitespace(&self) -> Self {
        let new_len = self.data.trim_end().len();
        self.slice(0..new_len)
    }
}
