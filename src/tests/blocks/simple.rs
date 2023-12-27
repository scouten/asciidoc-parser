use nom::{
    error::{Error, ErrorKind},
    Err,
};

use crate::{blocks::SimpleBlock, Span};

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
    let expected = SimpleBlock {
        inlines: vec![Span::new("abc", true)],
    };

    let (rem, block) = SimpleBlock::parse(Span::new("abc", true)).unwrap();

    assert_eq!(rem.line(), 1);
    assert_eq!(rem.col(), 4);
    assert_eq!(*rem.data(), "");

    assert_eq!(block, expected);
}

#[test]
fn multiple_lines() {
    let (rem, block) = SimpleBlock::parse(Span::new("abc\ndef", true)).unwrap();

    assert_eq!(rem.line(), 2);
    assert_eq!(rem.col(), 4);
    assert_eq!(*rem.data(), "");

    assert_eq!(block.inlines.len(), 2);

    assert_eq!(block.inlines[0].line(), 1);
    assert_eq!(block.inlines[0].col(), 1);
    assert_eq!(*block.inlines[0].data(), "abc");

    assert_eq!(block.inlines[1].line(), 2);
    assert_eq!(block.inlines[1].col(), 1);
    assert_eq!(*block.inlines[1].data(), "def");
}

#[test]
fn consumes_blank_lines_after() {
    let expected = SimpleBlock {
        inlines: vec![Span::new("abc", true)],
    };

    let (rem, block) = SimpleBlock::parse(Span::new("abc\n\ndef", true)).unwrap();

    assert_eq!(rem.line(), 3);
    assert_eq!(rem.col(), 1);
    assert_eq!(*rem.data(), "def");

    assert_eq!(block, expected);
}
