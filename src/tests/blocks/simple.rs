use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{ContentModel, IsBlock, SimpleBlock},
    tests::fixtures::{blocks::TSimpleBlock, inlines::TInline, TSpan},
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = SimpleBlock::parse(Span::new("abc")).unwrap();
    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn empty_source() {
    assert!(SimpleBlock::parse(Span::new("")).is_none());
}

#[test]
fn only_spaces() {
    assert!(SimpleBlock::parse(Span::new("    ")).is_none());
}

#[test]
fn single_line() {
    let mi = SimpleBlock::parse(Span::new("abc")).unwrap();

    assert_eq!(
        mi.item,
        TSimpleBlock(TInline::Uninterpreted(TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        })),
    );

    assert_eq!(mi.item.content_model(), ContentModel::Simple);
    assert_eq!(mi.item.context().deref(), "paragraph");

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
    let mi = SimpleBlock::parse(Span::new("abc\ndef")).unwrap();

    assert_eq!(
        mi.item,
        TSimpleBlock(TInline::Sequence(
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
                })
            ],
            TSpan {
                data: "abc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            }
        ))
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
    let mi = SimpleBlock::parse(Span::new("abc\n\ndef")).unwrap();

    assert_eq!(
        mi.item,
        TSimpleBlock(TInline::Uninterpreted(TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }))
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
