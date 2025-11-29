use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{Block, ContentModel, IsBlock, ListItem, ListItemMarker, metadata::BlockMetadata},
    internal::debug::DebugSliceReference,
    span::MatchedItem,
    strings::CowStr,
    warnings::Warning,
};

/// A list contains a sequence of items prefixed with symbol, such as a disc
/// (aka bullet). Each individual item in the list is represented by a
/// [`ListItem`].
///
/// [`ListItem`]: crate::blocks::ListItem
#[derive(Clone, Eq, PartialEq)]
pub struct ListBlock<'src> {
    type_: ListType,
    items: Vec<Block<'src>>,
    source: Span<'src>,
    title_source: Option<Span<'src>>,
    title: Option<String>,
    anchor: Option<Span<'src>>,
    anchor_reftext: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> ListBlock<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
        warnings: &mut Vec<Warning<'src>>,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.discard_empty_lines();

        let mut items: Vec<Block<'src>> = vec![];
        let mut next_item_source = source;
        let mut first_marker: Option<ListItemMarker<'src>> = None;

        loop {
            // TEMPORARY: Ignore block metadata for list items.
            let list_item_metadata = BlockMetadata {
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
                source: next_item_source,
                block_start: next_item_source,
            };

            let Some(mut list_item_mi) = ListItem::parse(&list_item_metadata, parser, warnings)
            else {
                break;
            };

            let marker = list_item_mi.item.list_item_marker();

            // If this item's marker doesn't match the existing list marker, we are changing
            // levels in the list hierarchy.
            if let Some(ref first_marker) = first_marker {
                if !first_marker.is_match_for(&marker) {
                    // TEMPORARY assume we're adding a new nesting level. Unimplemented to see if we
                    // need to unwind.
                    if let Some(nested_list) =
                        ListBlock::parse(&list_item_metadata, parser, warnings)
                    {
                        list_item_mi =
                            ListItem::from_nested_list(nested_list, first_marker.clone());
                    } else {
                        panic!("I was expecting this list to parse");
                    }
                }
            } else {
                first_marker = Some(marker);
            }

            items.push(Block::ListItem(list_item_mi.item));
            next_item_source = list_item_mi.after.discard_empty_lines();
        }

        if items.is_empty() {
            return None;
        }

        let Some(first_marker) = first_marker else {
            return None;
        };

        let type_ = match first_marker {
            ListItemMarker::Asterisks(_) => ListType::Unordered,
            ListItemMarker::Hyphen(_) => ListType::Unordered,
            ListItemMarker::Dots(_) => ListType::Ordered,
        };

        Some(MatchedItem {
            item: Self {
                type_,
                items,
                source: source
                    .trim_remainder(next_item_source)
                    .trim_trailing_line_end()
                    .trim_trailing_whitespace(),
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                anchor_reftext: metadata.anchor_reftext,
                attrlist: metadata.attrlist.clone(),
            },
            after: next_item_source,
        })
    }

    /// Returns the type of this list.
    pub fn type_(&self) -> ListType {
        self.type_
    }
}

impl<'src> IsBlock<'src> for ListBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn raw_context(&self) -> CowStr<'src> {
        "list".into()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.items.iter()
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

impl<'src> HasSpan<'src> for ListBlock<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

impl std::fmt::Debug for ListBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListBlock")
            .field("type_", &self.type_)
            .field("items", &DebugSliceReference(&self.items))
            .field("source", &self.source)
            .field("title_source", &self.title_source)
            .field("title", &self.title)
            .field("anchor", &self.anchor)
            .field("anchor_reftext", &self.anchor_reftext)
            .field("attrlist", &self.attrlist)
            .finish()
    }
}

/// Represents the type of a list.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ListType {
    /// An unordered list is a list with items prefixed with symbol, such as a
    /// disc (aka bullet).
    Unordered,

    /// An ordered list is a list with items prefixed with a number or other
    /// sequential mark.
    Ordered,
}

impl std::fmt::Debug for ListType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListType::Unordered => write!(f, "ListType::Unordered"),
            ListType::Ordered => write!(f, "ListType::Ordered"),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]
    #![allow(clippy::unwrap_used)]

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        HasSpan,
        blocks::{ContentModel, IsBlock, ListType, metadata::BlockMetadata},
        content::SubstitutionGroup,
        span::MatchedItem,
        tests::prelude::*,
        warnings::Warning,
    };

    fn list_parse<'a>(source: &'a str) -> Option<MatchedItem<'a, crate::blocks::ListBlock<'a>>> {
        let mut parser = crate::Parser::default();
        let mut warnings: Vec<Warning<'a>> = vec![];

        let metadata = BlockMetadata::parse(crate::Span::new(source), &mut parser).item;

        let result = crate::blocks::list::ListBlock::parse(&metadata, &mut parser, &mut warnings);

        assert!(warnings.is_empty());

        result
    }

    #[test]
    fn basic_case() {
        assert!(list_parse("-xyz").is_none());
        assert!(list_parse("-- x").is_none());

        let list = list_parse("- blah").unwrap();

        assert_eq!(
            list.item,
            ListBlock {
                type_: ListType::Unordered,
                items: &[Block::ListItem(ListItem {
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
                },),],
                source: Span {
                    data: "- blah",
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

        assert_eq!(list.item.type_(), ListType::Unordered);
        assert_eq!(list.item.content_model(), ContentModel::Compound);
        assert_eq!(list.item.raw_context().as_ref(), "list");

        let mut list_blocks = list.item.nested_blocks();

        let list_item = list_blocks.next().unwrap();

        assert_eq!(
            list_item,
            &Block::ListItem(ListItem {
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
            })
        );

        assert_eq!(list_item.content_model(), ContentModel::Compound);
        assert_eq!(list_item.raw_context().as_ref(), "list_item");

        let mut li_blocks = list_item.nested_blocks();

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

        assert!(list_item.title_source().is_none());
        assert!(list_item.title().is_none());
        assert!(list_item.anchor().is_none());
        assert!(list_item.anchor_reftext().is_none());
        assert!(list_item.attrlist().is_none());
        assert_eq!(list_item.substitution_group(), SubstitutionGroup::Normal);
        assert_eq!(
            list_item.span(),
            Span {
                data: "- blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(list_blocks.next().is_none());

        assert!(list.item.title_source().is_none());
        assert!(list.item.title().is_none());
        assert!(list.item.anchor().is_none());
        assert!(list.item.anchor_reftext().is_none());
        assert!(list.item.attrlist().is_none());

        assert_eq!(
            format!("{:#?}", list.item),
            "ListBlock {\n    type_: ListType::Unordered,\n    items: &[\n        Block::ListItem(\n            ListItem {\n                marker: ListItemMarker::Hyphen(\n                    Span {\n                        data: \"-\",\n                        line: 1,\n                        col: 1,\n                        offset: 0,\n                    },\n                ),\n                blocks: &[\n                    Block::Simple(\n                        SimpleBlock {\n                            content: Content {\n                                original: Span {\n                                    data: \"blah\",\n                                    line: 1,\n                                    col: 3,\n                                    offset: 2,\n                                },\n                                rendered: \"blah\",\n                            },\n                            source: Span {\n                                data: \"blah\",\n                                line: 1,\n                                col: 3,\n                                offset: 2,\n                            },\n                            title_source: None,\n                            title: None,\n                            anchor: None,\n                            anchor_reftext: None,\n                            attrlist: None,\n                        },\n                    ),\n                ],\n                source: Span {\n                    data: \"- blah\",\n                    line: 1,\n                    col: 1,\n                    offset: 0,\n                },\n                anchor: None,\n                anchor_reftext: None,\n                attrlist: None,\n            },\n        ),\n    ],\n    source: Span {\n        data: \"- blah\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n    title_source: None,\n    title: None,\n    anchor: None,\n    anchor_reftext: None,\n    attrlist: None,\n}"
        );

        assert_eq!(
            list.after,
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6,
            }
        );
    }

    #[test]
    fn list_type_impl_debug() {
        assert_eq!(format!("{:#?}", ListType::Unordered), "ListType::Unordered");
        assert_eq!(format!("{:#?}", ListType::Ordered), "ListType::Ordered");
    }
}
