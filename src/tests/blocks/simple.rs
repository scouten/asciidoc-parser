use nom::{
    error::{Error, ErrorKind},
    Err,
};

use crate::{
    blocks::SimpleBlock,
    tests::fixtures::{blocks::TSimpleBlock, TSpan},
    Span,
};

#[test]
fn empty_source() {
    let expected_err = Err::Error(Error::new(Span::new("", true), ErrorKind::TakeTill1));

    let actual_err = SimpleBlock::parse(Span::new("", true)).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn only_spaces() {
    let expected_err = Err::Error(Error::new(Span::new("    ", true), ErrorKind::TakeTill1));

    let actual_err = SimpleBlock::parse(Span::new("    ", true)).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn single_line() {
    let (rem, block) = SimpleBlock::parse(Span::new("abc", true)).unwrap();

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
        TSimpleBlock {
            inlines: vec![TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }],
        }
    );
}

#[test]
fn multiple_lines() {
    let (rem, block) = SimpleBlock::parse(Span::new("abc\ndef", true)).unwrap();

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
        TSimpleBlock {
            inlines: vec![
                TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                TSpan {
                    data: "def",
                    line: 2,
                    col: 1,
                    offset: 4,
                }
            ],
        }
    );
}

#[test]
fn consumes_blank_lines_after() {
    let (rem, block) = SimpleBlock::parse(Span::new("abc\n\ndef", true)).unwrap();

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
        TSimpleBlock {
            inlines: vec![TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }],
        }
    );
}
