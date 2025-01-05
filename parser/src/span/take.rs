use super::{MatchedItem, Span};

impl<'src> Span<'src> {
    /// Split this span, consuming the exact character sequence `prefix` if
    /// found.
    ///
    /// Returns `None` if `prefix` is not found.
    pub(crate) fn take_prefix(self, prefix: &str) -> Option<MatchedItem<'src, Self>> {
        if self.data.starts_with(prefix) {
            Some(self.into_parse_result(prefix.len()))
        } else {
            None
        }
    }

    /// Split this span, consuming any white space.
    pub(crate) fn take_whitespace(self) -> MatchedItem<'src, Self> {
        self.take_while(|c| c == ' ' || c == '\t')
    }

    /// Split this span, consuming at least one white space character.
    ///
    /// Returns `None` if the first character is not a space or tab.
    pub(crate) fn take_required_whitespace(self) -> Option<MatchedItem<'src, Self>> {
        let mi = self.take_while(|c| c == ' ' || c == '\t');
        if mi.item.is_empty() {
            None
        } else {
            Some(mi)
        }
    }

    /// Split this span at the first character that doesn't match `predicate`.
    pub(crate) fn take_while<P>(self, predicate: P) -> MatchedItem<'src, Self>
    where
        P: Fn(char) -> bool,
    {
        match self.position(|c| !predicate(c)) {
            Some(n) => self.into_parse_result(n),
            None => self.into_parse_result(self.data.len()),
        }
    }
}
