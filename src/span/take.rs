use nom::InputIter;

use super::{ParseResult, Span};

impl<'a> Span<'a> {
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
