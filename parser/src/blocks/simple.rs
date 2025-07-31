use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
    content::{Content, SubstitutionGroup},
    span::MatchedItem,
    strings::CowStr,
};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'src> {
    content: Content<'src>,
    source: Span<'src>,
    title_source: Option<Span<'src>>,
    title: Option<String>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> SimpleBlock<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.take_non_empty_lines()?;

        // TO DO: Allow overrides for SubstitutionGroup.
        let mut content: Content<'src> = source.item.into();
        SubstitutionGroup::Normal.apply(&mut content, parser, metadata.attrlist.as_ref());

        Some(MatchedItem {
            item: Self {
                content,
                source: metadata
                    .source
                    .trim_remainder(source.after)
                    .trim_trailing_whitespace(),
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                attrlist: metadata.attrlist.clone(),
            },
            after: source.after.discard_empty_lines(),
        })
    }

    pub(crate) fn parse_fast(
        source: Span<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = source.take_non_empty_lines()?;

        let mut content: Content<'src> = source.item.into();
        SubstitutionGroup::Normal.apply(&mut content, parser, None);

        Some(MatchedItem {
            item: Self {
                content,
                source: source.item,
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },
            after: source.after.discard_empty_lines(),
        })
    }

    /// Return the interpreted content of this block.
    pub fn content(&self) -> &Content<'src> {
        &self.content
    }
}

impl<'src> IsBlock<'src> for SimpleBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Simple
    }

    fn raw_context(&self) -> CowStr<'src> {
        "paragraph".into()
    }

    fn title_source(&'src self) -> Option<Span<'src>> {
        self.title_source
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        self.anchor
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for SimpleBlock<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}
