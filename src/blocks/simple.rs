use super::{ContentModel, IsBlock};
use crate::{inlines::Inline, span::MatchedItem, strings::CowStr, HasSpan, Span};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'src> {
    inline: Inline<'src>,
    title: Option<Span<'src>>,
}

impl<'src> SimpleBlock<'src> {
    pub(crate) fn parse(
        source: Span<'src>,
        title: Option<Span<'src>>,
    ) -> Option<MatchedItem<'src, Self>> {
        let inline = Inline::parse_lines(source)?;
        Some(MatchedItem {
            item: Self {
                inline: inline.item,
                title,
            },
            after: inline.after.discard_empty_lines(),
        })
    }

    /// Return the inline content of this block.
    pub fn inline(&self) -> &Inline<'src> {
        &self.inline
    }
}

impl<'src> IsBlock<'src> for SimpleBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Simple
    }

    fn context(&self) -> CowStr<'src> {
        "paragraph".into()
    }

    fn title(&self) -> Option<Span<'src>> {
        self.title
    }
}

impl<'src> HasSpan<'src> for SimpleBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        self.inline.span()
    }
}
