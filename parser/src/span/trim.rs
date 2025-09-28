use super::Span;

impl Span<'_> {
    /// Return a [`Span`] that is the same as the source, but with any trailing
    /// whitespace removed.
    pub(crate) fn trim_trailing_whitespace(&self) -> Self {
        let new_len = self.data.trim_end().len();
        self.slice(0..new_len)
    }

    /// Return a [`Span`] that is the same as the source, but a single trailing
    /// line-ending (if found) removed.
    pub(crate) fn trim_trailing_line_end(&self) -> Self {
        if self.ends_with("\r\n") {
            self.slice(0..self.len() - 2)
        } else if self.ends_with('\n') {
            self.slice(0..self.len() - 1)
        } else {
            *self
        }
    }
}
