use super::Span;

/// When a parse request is successful, this data structure conveys the matched
/// item and subsequent input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct MatchedItem<'src, T> {
    /// The matched item.
    pub(crate) item: T,

    /// Remaining input span after matched item.
    pub(crate) after: Span<'src>,
}

impl<'src> MatchedItem<'src, Span<'src>> {
    /// Discard any instances of the given character from the beginning of
    /// `self.after`.
    pub(crate) fn trim_after_start_matches(&self, c: char) -> Self {
        if let Some(after) = self.after.strip_prefix(c) {
            let prefix_len = self.after.len() - after.len();
            let after = self.after.slice_from(prefix_len..);
            Self {
                item: self.item,
                after,
            }
        } else {
            *self
        }
    }

    /// Discard any instances of the given character from the end of
    /// `self.item`.
    pub(crate) fn trim_item_end_matches(&self, c: char) -> Self {
        if let Some(source) = self.item.strip_suffix(c) {
            let source = self.item.slice(0..source.len());
            Self {
                item: source,
                after: self.after,
            }
        } else {
            *self
        }
    }

    /// Discard any trailing spaces from `self.item`.
    pub(crate) fn trim_item_trailing_spaces(&self) -> Self {
        let source = self.item.trim_end_matches(' ');
        let source = self.item.slice(0..source.len());
        Self {
            item: source,
            after: self.after,
        }
    }
}

impl<'src, U> Default for MatchedItem<'src, U>
where
    U: Default,
{
    fn default() -> Self {
        Self {
            item: U::default(),
            after: Span::default(),
        }
    }
}

impl<U> MatchedItem<'_, Option<U>>
where
    U: Default,
{
    /// Unwrap the item if it's `Some(value)`, otherwise return `U::default()`.
    ///
    /// This method is only available when `T` is `Option<impl Default>`.
    #[inline(always)]
    #[allow(unused)] // TEMPORARY while building
    pub(crate) fn unwrap_item_or_default(self) -> U {
        self.item.unwrap_or_default()
    }
}
