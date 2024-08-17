use super::{ParseResult, Span};

impl<'a> Span<'a> {
    /// Split the current span into a [`ParseResult<Span>`] at the
    /// given position.
    pub(crate) fn into_parse_result(self, mut at_index: usize) -> ParseResult<'a, Self> {
        at_index = self.data.len().min(at_index);

        ParseResult {
            t: self.slice_to(..at_index),
            rem: self.slice_from(at_index..),
        }
    }

    /// Split this span at the first character that matches `predicate`,
    /// but will not return an empty subspan.
    ///
    /// Return `None` if:
    ///
    /// * `predicate` never returns `true`,
    /// * `predicate` returns `true` for the _first_ character in the span, or
    /// * the span is empty.
    pub(crate) fn split_at_match_non_empty<P>(self, predicate: P) -> Option<ParseResult<'a, Self>>
    where
        P: Fn(char) -> bool,
    {
        match self.position(predicate) {
            Some(0) => None,
            Some(n) => Some(self.into_parse_result(n)),
            None => {
                if self.data.is_empty() {
                    None
                } else {
                    Some(self.into_parse_result(self.data.len()))
                }
            }
        }
    }
}
