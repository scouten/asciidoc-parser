use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    tests::fixtures::{
        blocks::{TBlock, TSimpleBlock},
        inlines::TInline,
        TSpan,
    },
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = Block::parse(Span::new("abc"))
        .unwrap_if_no_warnings()
        .unwrap();

    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn err_empty_source() {
    assert!(Block::parse(Span::new(""))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn err_only_spaces() {
    assert!(Block::parse(Span::new("    "))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn single_line() {
    let mi = Block::parse(Span::new("abc"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }),
            title: None
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(mi.item.content_model(), ContentModel::Simple);
    assert_eq!(mi.item.context().deref(), "paragraph");
    assert_eq!(mi.item.nested_blocks().next(), None);

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 4,
            offset: 3
        }
    );
}

#[test]
fn multiple_lines() {
    let mi = Block::parse(Span::new("abc\ndef"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Sequence(
                vec![
                    TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }),
                    TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 2,
                        col: 1,
                        offset: 4,
                    }),
                ],
                TSpan {
                    data: "abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            ),
            title: None
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "abc\ndef",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 2,
            col: 4,
            offset: 7
        }
    );
}

#[test]
fn consumes_blank_lines_after() {
    let mi = Block::parse(Span::new("abc\n\ndef"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TBlock::Simple(TSimpleBlock {
            inline: TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }),
            title: None
        })
    );

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
        TSpan {
            data: "def",
            line: 3,
            col: 1,
            offset: 5
        }
    );
}
