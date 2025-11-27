use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{Block, ContentModel, IsBlock, ListItemMarker, SimpleBlock, metadata::BlockMetadata},
    internal::debug::DebugSliceReference,
    span::MatchedItem,
    strings::CowStr,
    warnings::Warning,
};

/// A list item is a special kind of block that contains one or more blocks
/// attached to it. In the simplest case, this will be a single [`SimpleBlock`]
/// with the principal text for the list item. In other cases, it may be any
/// number of blocks of any type which, together, form an entry in a list which
/// is the immediate parent of this block.
///
/// [`SimpleBlock`]: crate::blocks::SimpleBlock
#[derive(Clone, Eq, PartialEq)]
pub struct ListItem<'src> {
    marker: ListItemMarker<'src>,
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
    anchor: Option<Span<'src>>,
    anchor_reftext: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> ListItem<'src> {
    #[allow(unused)] // TEMPORARY while building
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
        _warnings: &mut Vec<Warning<'src>>,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.discard_empty_lines();

        let marker_mi = ListItemMarker::parse(source)?;
        let marker = marker_mi.item;

        let mut blocks: Vec<Block<'src>> = vec![];

        // TEMPORARY Q&D simple block parse to bootstrap.

        let hack_no_metadata = BlockMetadata {
            title_source: None,
            title: None,
            anchor: None,
            anchor_reftext: None,
            attrlist: None,
            source: marker_mi.after,
            block_start: marker_mi.after,
        };

        let simple_block_mi = SimpleBlock::parse_for_list_item(&hack_no_metadata, parser)?;
        blocks.push(Block::Simple(simple_block_mi.item));

        let after = simple_block_mi.after;
        let source = source.trim_remainder(after).trim_trailing_whitespace();

        Some(MatchedItem {
            item: Self {
                marker,
                blocks,
                source,
                anchor: metadata.anchor,
                anchor_reftext: metadata.anchor_reftext,
                attrlist: metadata.attrlist.clone(),
            },
            after,
        })
    }

    #[allow(unused)] // TEMPORARY while building
    pub(crate) fn list_item_marker(&'src self) -> &'src ListItemMarker<'src> {
        &self.marker
    }
}

impl<'src> IsBlock<'src> for ListItem<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn raw_context(&self) -> CowStr<'src> {
        "list_item".into()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.blocks.iter()
    }

    fn title_source(&'src self) -> Option<Span<'src>> {
        None
    }

    fn title(&self) -> Option<&str> {
        None
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

impl<'src> HasSpan<'src> for ListItem<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

impl std::fmt::Debug for ListItem<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListItem")
            .field("marker", &self.marker)
            .field("blocks", &DebugSliceReference(&self.blocks))
            .field("source", &self.source)
            .field("anchor", &self.anchor)
            .field("anchor_reftext", &self.anchor_reftext)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]
    #![allow(clippy::unwrap_used)]

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        HasSpan,
        blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
        span::MatchedItem,
        tests::prelude::*,
        warnings::Warning,
    };

    fn li_parse<'a>(source: &'a str) -> Option<MatchedItem<'a, crate::blocks::ListItem<'a>>> {
        let mut parser = crate::Parser::default();
        let mut warnings: Vec<Warning<'a>> = vec![];

        let metadata = BlockMetadata::parse(crate::Span::new(source), &mut parser).item;

        let result =
            crate::blocks::list_item::ListItem::parse(&metadata, &mut parser, &mut warnings);

        assert!(warnings.is_empty());

        result
    }

    #[test]
    fn hyphen() {
        assert!(li_parse("-xyz").is_none());
        assert!(li_parse("-- x").is_none());

        let li = li_parse("- blah").unwrap();

        assert_eq!(
            li.item,
            ListItem {
                marker: ListItemMarker::Hyphen(Span {
                    data: "-",
                    line: 1,
                    col: 1,
                    offset: 0,
                },),
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "blah",
                            line: 1,
                            col: 3,
                            offset: 2,
                        },
                        rendered: "blah",
                    },
                    source: Span {
                        data: "blah",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "- blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            }
        );

        assert_eq!(li.item.content_model(), ContentModel::Compound);
        assert_eq!(li.item.raw_context().as_ref(), "list_item");

        let mut li_blocks = li.item.nested_blocks();

        assert_eq!(
            li_blocks.next().unwrap(),
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "blah",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },
                    rendered: "blah",
                },
                source: Span {
                    data: "blah",
                    line: 1,
                    col: 3,
                    offset: 2,
                },
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            })
        );
        assert!(li_blocks.next().is_none());

        assert!(li.item.title_source().is_none());
        assert!(li.item.title().is_none());
        assert!(li.item.anchor().is_none());
        assert!(li.item.anchor_reftext().is_none());
        assert!(li.item.attrlist().is_none());

        assert_eq!(
            li.item.span(),
            Span {
                data: "- blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            li.after,
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6,
            }
        );

        assert_eq!(
            format!("{:#?}", li.item),
            "ListItem {\n    marker: ListItemMarker::Hyphen(\n        Span {\n            data: \"-\",\n            line: 1,\n            col: 1,\n            offset: 0,\n        },\n    ),\n    blocks: &[\n        Block::Simple(\n            SimpleBlock {\n                content: Content {\n                    original: Span {\n                        data: \"blah\",\n                        line: 1,\n                        col: 3,\n                        offset: 2,\n                    },\n                    rendered: \"blah\",\n                },\n                source: Span {\n                    data: \"blah\",\n                    line: 1,\n                    col: 3,\n                    offset: 2,\n                },\n                title_source: None,\n                title: None,\n                anchor: None,\n                anchor_reftext: None,\n                attrlist: None,\n            },\n        ),\n    ],\n    source: Span {\n        data: \"- blah\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n    anchor: None,\n    anchor_reftext: None,\n    attrlist: None,\n}"
        );
    }

    #[test]
    fn asterisks() {
        assert!(li_parse("*").is_none());
        assert!(li_parse("*xyz").is_none());
        assert!(li_parse("*- xyz").is_none());

        let li = li_parse("* blah").unwrap();

        assert_eq!(
            li.item,
            ListItem {
                marker: ListItemMarker::Asterisks(Span {
                    data: "*",
                    line: 1,
                    col: 1,
                    offset: 0,
                },),
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "blah",
                            line: 1,
                            col: 3,
                            offset: 2,
                        },
                        rendered: "blah",
                    },
                    source: Span {
                        data: "blah",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "* blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            }
        );

        assert_eq!(
            li.item.span(),
            Span {
                data: "* blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            li.after,
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6,
            }
        );
    }
}
