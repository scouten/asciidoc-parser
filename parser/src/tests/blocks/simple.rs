use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
    content::SubstitutionGroup,
    tests::fixtures::{
        Span,
        attributes::{Attrlist, ElementAttribute},
        blocks::TSimpleBlock,
        content::TContent,
    },
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let mut parser = Parser::default();

    let b1 = crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc"), &mut parser).unwrap();

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
    assert!(crate::blocks::SimpleBlock::parse(&BlockMetadata::new("    "), &mut parser).is_none());
}

#[test]
fn single_line() {
    let mut parser = Parser::default();
    let mi = crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc"), &mut parser).unwrap();

    assert_eq!(
        mi.item,
        TSimpleBlock {
            content: TContent {
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
    let mi =
        crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc\ndef"), &mut parser).unwrap();

    assert_eq!(
        mi.item,
        TSimpleBlock {
            content: TContent {
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
    let mi =
        crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc\n\ndef"), &mut parser).unwrap();

    assert_eq!(
        mi.item,
        TSimpleBlock {
            content: TContent {
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
        TSimpleBlock {
            content: TContent {
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
            attrlist: Some(Attrlist {
                attributes: &[ElementAttribute {
                    name: Some("subs"),
                    value: "quotes",
                    shorthand_items: &[],
                },],
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
