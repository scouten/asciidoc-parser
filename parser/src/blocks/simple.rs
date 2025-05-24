use crate::{
    attributes::Attrlist,
    blocks::{preamble::Preamble, ContentModel, IsBlock},
    span::{content::SubstitutionGroup, MatchedItem},
    strings::CowStr,
    Content, HasSpan, Parser, Span,
};

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'src> {
    content: Content<'src>,
    source: Span<'src>,
    title: Option<Span<'src>>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> SimpleBlock<'src> {
    pub(crate) fn parse(
        preamble: &Preamble<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = preamble.block_start.take_non_empty_lines()?;

        // TO DO: Allow overrides for SubstitutionGroup.
        let mut content: Content<'src> = source.item.into();
        SubstitutionGroup::Normal.apply(&mut content, parser);

        Some(MatchedItem {
            item: Self {
                content,
                source: preamble
                    .source
                    .trim_remainder(source.after)
                    .trim_trailing_whitespace(),
                title: preamble.title,
                anchor: preamble.anchor,
                attrlist: preamble.attrlist.clone(),
            },
            after: source.after.discard_empty_lines(),
        })
    }

    pub(crate) fn parse_fast(
        source: Span<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = source.take_non_empty_lines()?;

        // TO DO: Allow overrides for SubstitutionGroup.
        let mut content: Content<'src> = source.item.into();
        SubstitutionGroup::Normal.apply(&mut content, parser);

        Some(MatchedItem {
            item: Self {
                content,
                source: source.item,
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

    fn title(&self) -> Option<Span<'src>> {
        self.title
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        self.anchor
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for SimpleBlock<'src> {
    fn span(&'src self) -> &'src Span<'src> {
        &self.source
    }
}
