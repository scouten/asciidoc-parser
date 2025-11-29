use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock, ListItemMarker, metadata::BlockMetadata},
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
    anchor_reftext: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> SimpleBlock<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.take_non_empty_lines()?;

        let mut next = metadata.block_start;
        let mut filtered_lines: Vec<&'src str> = vec![];

        while let Some(inline) = next.take_non_empty_line() {
            if !inline.item.starts_with("//") || inline.item.starts_with("///") {
                filtered_lines.push(inline.item.trim_trailing_whitespace().data());
            }

            next = inline.after;
        }

        let item_source = source.item.trim_remainder(next).trim_trailing_whitespace();

        let filtered_lines = filtered_lines.join("\n");
        let mut content: Content<'src> = Content::from_filtered(item_source, filtered_lines);

        SubstitutionGroup::Normal
            .override_via_attrlist(metadata.attrlist.as_ref())
            .apply(&mut content, parser, metadata.attrlist.as_ref());

        Some(MatchedItem {
            item: Self {
                content,
                source: metadata
                    .source
                    .trim_remainder(next)
                    .trim_trailing_whitespace(),
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                anchor_reftext: metadata.anchor_reftext,
                attrlist: metadata.attrlist.clone(),
            },
            after: next.discard_empty_lines(),
        })
    }

    pub(crate) fn parse_for_list_item(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.take_non_empty_lines()?;

        let mut next = metadata.block_start;
        let mut filtered_lines: Vec<&'src str> = vec![];

        while let Some(inline) = next.take_non_empty_line() {
            if let Some(_marker) = ListItemMarker::parse(inline.item) {
                break;
            }

            if !inline.item.starts_with("//") || inline.item.starts_with("///") {
                filtered_lines.push(inline.item.trim_trailing_whitespace().data());
            }

            next = inline.after;
        }

        let item_source = source.item.trim_remainder(next).trim_trailing_whitespace();

        let filtered_lines = filtered_lines.join("\n");
        let mut content: Content<'src> = Content::from_filtered(item_source, filtered_lines);

        SubstitutionGroup::Normal
            .override_via_attrlist(metadata.attrlist.as_ref())
            .apply(&mut content, parser, metadata.attrlist.as_ref());

        Some(MatchedItem {
            item: Self {
                content,
                source: item_source,
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                anchor_reftext: metadata.anchor_reftext,
                attrlist: metadata.attrlist.clone(),
            },
            after: next.discard_empty_lines(),
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
                anchor_reftext: None,
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

    fn anchor_reftext(&'src self) -> Option<Span<'src>> {
        self.anchor_reftext
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

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::ops::Deref;

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
        content::SubstitutionGroup,
        tests::prelude::*,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let mut parser = Parser::default();

        let b1 =
            crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc"), &mut parser).unwrap();

        let b2 = b1.item.clone();
        assert_eq!(b1.item, b2);
    }

    #[test]
    fn empty_source() {
        let mut parser = Parser::default();
        assert!(crate::blocks::SimpleBlock::parse(&BlockMetadata::new(""), &mut parser).is_none());
    }

    #[test]
    fn only_spaces() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::SimpleBlock::parse(&BlockMetadata::new("    "), &mut parser).is_none()
        );
    }

    #[test]
    fn single_line() {
        let mut parser = Parser::default();
        let mi =
            crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc"), &mut parser).unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
                content: Content {
                    original: Span {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "abc",
                },
                source: Span {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            },
        );

        assert_eq!(mi.item.content_model(), ContentModel::Simple);
        assert_eq!(mi.item.raw_context().deref(), "paragraph");
        assert_eq!(mi.item.resolved_context().deref(), "paragraph");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.anchor_reftext().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        let mi = crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc\ndef"), &mut parser)
            .unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
                content: Content {
                    original: Span {
                        data: "abc\ndef",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "abc\ndef",
                },
                source: Span {
                    data: "abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 2,
                col: 4,
                offset: 7
            }
        );
    }

    #[test]
    fn consumes_blank_lines_after() {
        let mut parser = Parser::default();
        let mi = crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc\n\ndef"), &mut parser)
            .unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
                content: Content {
                    original: Span {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "abc",
                },
                source: Span {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 3,
                col: 1,
                offset: 5
            }
        );
    }

    #[test]
    fn overrides_sub_group_via_subs_attribute() {
        let mut parser = Parser::default();
        let mi = crate::blocks::SimpleBlock::parse(
            &BlockMetadata::new("[subs=quotes]\na<b>c *bold*\n\ndef"),
            &mut parser,
        )
        .unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
                content: Content {
                    original: Span {
                        data: "a<b>c *bold*",
                        line: 2,
                        col: 1,
                        offset: 14,
                    },
                    rendered: "a<b>c <strong>bold</strong>",
                },
                source: Span {
                    data: "[subs=quotes]\na<b>c *bold*",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: Some("subs"),
                        value: "quotes",
                        shorthand_items: &[],
                    },],
                    anchor: None,
                    source: Span {
                        data: "subs=quotes",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 4,
                col: 1,
                offset: 28
            }
        );
    }
}
