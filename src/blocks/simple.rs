use crate::{
    blocks::{preamble::Preamble, ContentModel, IsBlock},
    inlines::Inline,
    span::MatchedItem,
    strings::CowStr,
    HasSpan, Span,
};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'src> {
    inline: Inline<'src>,
    source: Span<'src>,
    title: Option<Span<'src>>,
}

impl<'src> SimpleBlock<'src> {
    pub(crate) fn parse(preamble: &Preamble<'src>) -> Option<MatchedItem<'src, Self>> {
        let inline = Inline::parse_lines(preamble.block_start)?;

        Some(MatchedItem {
            item: Self {
                inline: inline.item,
                source: preamble.source.trim_remainder(inline.after),
                title: preamble.title,
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
        &self.source
    }
}
