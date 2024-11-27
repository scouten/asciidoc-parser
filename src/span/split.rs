use super::{MatchedItem, Span};

impl<'src> Span<'src> {
    /// Split the current span into a [`MatchedItem<Span>`] at the
    /// given position.
    pub(crate) fn into_parse_result(self, mut at_index: usize) -> MatchedItem<'src, Self> {
        // Avoid panic if `at_index` is out of range.
        at_index = self.data.len().min(at_index);

        MatchedItem {
            item: self.slice_to(..at_index),
            after: self.slice_from(at_index..),
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
    pub(crate) fn split_at_match_non_empty<P>(self, predicate: P) -> Option<MatchedItem<'src, Self>>
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
