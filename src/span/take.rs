use nom::InputIter;

use super::{ParseResult, Span};

impl<'a> Span<'a> {
    /// Split this span, consuming the exact character sequence `prefix` if
    /// found.
    ///
    /// Returns `None` if `prefix` is not found.
    ///
    /// NOM REFACTOR: Replacement for `tag`.
    #[allow(dead_code)]
    pub(crate) fn take_prefix(self, prefix: &str) -> Option<ParseResult<'a, Self>> {
        if self.data.starts_with(prefix) {
            Some(self.into_parse_result(prefix.len()))
        } else {
            None
        }
    }

    /// Split this span, consuming any white space.
    ///
    /// NOM REFACTOR: Replacement for `space0`.
    #[allow(dead_code)]
    pub(crate) fn take_whitespace(self) -> ParseResult<'a, Self> {
        self.take_while(|c| c == ' ' || c == '\t')
    }

    /// Split this span, consuming at least one white space character.
    ///
    /// Returns `None` if the first character is not a space or tab.
    ///
    /// NOM REFACTOR: Replacement for `space1`.
    #[allow(dead_code)]
    pub(crate) fn take_required_whitespace(self) -> Option<ParseResult<'a, Self>> {
        let pr = self.take_while(|c| c == ' ' || c == '\t');
        if pr.t.is_empty() {
            None
        } else {
            Some(pr)
        }
    }

    /// Split this span at the first character that doesn't match `predicate`.
    ///
    /// NOM REFACTOR: Replacement for `is_not`.
    pub(crate) fn take_while<P>(self, predicate: P) -> ParseResult<'a, Self>
    where
        P: Fn(char) -> bool,
    {
        match self.data.position(|c| !predicate(c)) {
            Some(n) => self.into_parse_result(n),
            None => self.into_parse_result(self.data.len()),
        }
    }
}
