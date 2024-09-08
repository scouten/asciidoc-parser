use super::Span;

/// Represents a successful parse result and subsequent remainder of the span.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ParseResult<'src, T> {
    /// The successful parse result.
    pub(crate) t: T,

    /// Remainder of previous input span.
    pub(crate) rem: Span<'src>,
}

impl<'src> ParseResult<'src, Span<'src>> {
    /// Discard any instances of the given character from the beginning of
    /// `self.rem`.
    pub(crate) fn trim_rem_start_matches(&self, c: char) -> Self {
        if let Some(rem) = self.rem.strip_prefix(c) {
            let prefix_len = self.rem.len() - rem.len();
            let rem = self.rem.slice_from(prefix_len..);
            Self { t: self.t, rem }
        } else {
            *self
        }
    }

    /// Discard any instances of the given character from the end of `self.t`.
    pub(crate) fn trim_t_end_matches(&self, c: char) -> Self {
        if let Some(inp) = self.t.strip_suffix(c) {
            let inp = self.t.slice(0..inp.len());
            Self {
                t: inp,
                rem: self.rem,
            }
        } else {
            *self
        }
    }

    /// Discard any trailing spaces from `self.t`.
    pub(crate) fn trim_t_trailing_spaces(&self) -> Self {
        let inp = self.t.trim_end_matches(' ');
        let inp = self.t.slice(0..inp.len());
        Self {
            t: inp,
            rem: self.rem,
        }
    }
}
