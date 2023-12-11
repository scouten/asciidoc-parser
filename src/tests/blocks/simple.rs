use nom::{
    error::{Error, ErrorKind},
    Err,
};

use crate::blocks::simple::SimpleBlock;

#[test]
fn empty_source() {
    let expected_err: Err<Error<&str>> = Err::Error(Error::new("", ErrorKind::TakeTill1));

    let actual_err = SimpleBlock::from_str("").unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn only_spaces() {
    let expected_err: Err<Error<&str>> = Err::Error(Error::new("   ", ErrorKind::TakeTill1));

    let actual_err = SimpleBlock::from_str("   ").unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn single_line() {
    let expected = SimpleBlock {
        inlines: vec!["abc"],
    };

    assert_eq!(SimpleBlock::from_str("abc"), Ok(("", expected)));
}

#[test]
fn multiple_lines() {
    let expected = SimpleBlock {
        inlines: vec!["abc", "def"],
    };

    assert_eq!(
        SimpleBlock::from_str("abc\ndef\n\nghi"),
        Ok(("\nghi", expected))
    );
}
