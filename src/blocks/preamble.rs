use crate::{
    attributes::Attrlist,
    warnings::{MatchAndWarnings, Warning},
    Span,
};

/// A preamble represents the common elements that can precede any block type
/// (title and attribute list). It is used internally to track those values
/// before the specific block type is fully formed.
#[derive(Debug)]
pub(crate) struct Preamble<'src> {
    /// The block's title, if any.
    pub(crate) title: Option<Span<'src>>,

    /// The block's attribute list, if any.
    #[allow(dead_code)] // TEMPORARY while building
    pub(crate) attrlist: Option<Attrlist<'src>>,

    /// The source span as understood when the preamble content was first
    /// encountered. Does not necessarily end at the end of the block.
    pub(crate) source: Span<'src>,

    /// The source span after reading the optional title and attribute list.
    /// This is the beginning of content for the specific block type.
    pub(crate) block_start: Span<'src>,
}

impl<'src> Preamble<'src> {
    /// (For testing only) Parse the preamble from a raw text constant.
    #[cfg(test)]
    pub(crate) fn new(data: &'src str) -> Self {
        Self::parse(Span::new(data)).item
    }

    /// Parse the title and attribute list for a block, if any.
    pub(crate) fn parse(source: Span<'src>) -> MatchAndWarnings<'src, Self> {
        let warnings: Vec<Warning<'src>> = vec![];
        let source = source.discard_empty_lines();

        // Optimization: If this doesn't start with `.` or `#`, the preamble is empty
        // and we can avoid the cost of `take_normalized_line` below.
        if !(source.starts_with('.') || source.starts_with('#')) {
            return MatchAndWarnings {
                item: Self {
                    title: None,
                    attrlist: None,
                    source,
                    block_start: source,
                },
                warnings,
            };
        }

        // Does this block have a title?
        let maybe_title = source.take_normalized_line();
        let (title, block_start) =
            if maybe_title.item.starts_with('.') && !maybe_title.item.starts_with("..") {
                let title = maybe_title.item.discard(1);
                if title.take_whitespace().item.is_empty() {
                    (Some(title), maybe_title.after)
                } else {
                    (None, source)
                }
            } else {
                (None, source)
            };

        // TO DO: Does this block have an attribute list?

        MatchAndWarnings {
            item: Self {
                title,
                attrlist: None, // temporary
                source,
                block_start,
            },
            warnings,
        }
    }

    /// Return `true` if both `title` and `attrlist` are empty.
    pub(crate) fn is_empty(&self) -> bool {
        self.title.is_none() && self.attrlist.is_none()
    }
}
