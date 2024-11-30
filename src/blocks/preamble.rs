use crate::{
    attributes::Attrlist,
    span::MatchedItem,
    warnings::{MatchAndWarnings, Warning},
    Span,
};

/// A preamble represents the common elements that can precede any block type
/// (title and attribute list). It is used internally to track those values
/// before the specific block type is fully formed.
pub(crate) struct Preamble<'src> {
    /// The block's title, if any.
    pub(crate) title: Option<Span<'src>>,

    /// The block's attribute list, if any.
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
        let mut warnings: Vec<Warning<'src>> = vec![];
        let source = source.discard_empty_lines();

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

        // Does this block have an attribute list?
        let (attrlist, block_start) = if let Some(MatchAndWarnings {
            item:
                MatchedItem {
                    item: attrlist,
                    after: block_start,
                },
            warnings: mut attrlist_warnings,
        }) = parse_maybe_attrlist_line(block_start)
        {
            if !attrlist_warnings.is_empty() {
                warnings.append(&mut attrlist_warnings);
            }

            (Some(attrlist), block_start)
        } else {
            (None, block_start)
        };

        MatchAndWarnings {
            item: Self {
                title,
                attrlist,
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

fn parse_maybe_attrlist_line(
    source: Span<'_>,
) -> Option<MatchAndWarnings<'_, MatchedItem<'_, Attrlist<'_>>>> {
    let first_char = source.chars().next()?;
    if first_char != '[' {
        return None;
    }

    let MatchedItem {
        item: line,
        after: block_start,
    } = source.take_normalized_line();

    if !line.ends_with(']') {
        return None;
    }

    // Drop opening and closing braces now that we know they are there.
    let attrlist_src = line.slice(1..line.len() - 1);

    let MatchAndWarnings {
        item: MatchedItem {
            item: attrlist,
            after: _,
        },
        warnings,
    } = Attrlist::parse(attrlist_src);

    Some(MatchAndWarnings {
        item: MatchedItem {
            item: attrlist,
            after: block_start,
        },
        warnings,
    })
}
