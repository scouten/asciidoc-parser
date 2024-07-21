use std::ops::Deref;

use nom::{
    error::{Error, ErrorKind},
    Err,
};
use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{ContentModel, IsBlock, SimpleBlock},
    tests::fixtures::{blocks::TSimpleBlock, inlines::TInline, TSpan},
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let (_, b1) = SimpleBlock::parse(Span::new("abc")).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    let expected_err = Err::Error(Error::new(Span::new(""), ErrorKind::TakeTill1));

    let actual_err = SimpleBlock::parse(Span::new("")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn only_spaces() {
    let expected_err = Err::Error(Error::new(Span::new("    "), ErrorKind::TakeTill1));

    let actual_err = SimpleBlock::parse(Span::new("    ")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn single_line() {
    let (rem, block) = SimpleBlock::parse(Span::new("abc")).unwrap();

    assert_eq!(block.content_model(), ContentModel::Simple);
    assert_eq!(block.context().deref(), "paragraph");

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 1,
            col: 4,
            offset: 3
        }
    );

    assert_eq!(
        block,
        TSimpleBlock(TInline::Uninterpreted(TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        })),
    );
}

#[test]
fn multiple_lines() {
    let (rem, block) = SimpleBlock::parse(Span::new("abc\ndef")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 2,
            col: 4,
            offset: 7
        }
    );

    assert_eq!(
        block,
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
}

#[test]
fn consumes_blank_lines_after() {
    let (rem, block) = SimpleBlock::parse(Span::new("abc\n\ndef")).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "def",
            line: 3,
            col: 1,
            offset: 5
        }
    );

    assert_eq!(
        block,
        TSimpleBlock(TInline::Uninterpreted(TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }))
    );
}
