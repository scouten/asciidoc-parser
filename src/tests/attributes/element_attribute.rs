use nom::{
    error::{Error, ErrorKind},
    Err,
};
use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::ElementAttribute,
    tests::fixtures::{attributes::TElementAttribute, TSpan},
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let (_, b1) = ElementAttribute::parse(Span::new("abc", true)).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    let expected_err = Err::Error(Error::new(Span::new("", true), ErrorKind::IsNot));

    let actual_err = ElementAttribute::parse(Span::new("", true)).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn only_spaces() {
    let (rem, attr) = ElementAttribute::parse(Span::new("   ", true)).unwrap();

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
        attr,
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "   ",
                line: 1,
                col: 1,
                offset: 0,
            },
            source: TSpan {
                data: "   ",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        attr.span(),
        TSpan {
            data: "   ",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}

#[test]
fn unquoted_and_unnamed_value() {
    let (rem, attr) = ElementAttribute::parse(Span::new("abc", true)).unwrap();

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
        attr,
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },
            source: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        attr.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}

#[test]
fn unquoted_stops_at_comma() {
    let (rem, attr) = ElementAttribute::parse(Span::new("abc,def", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: ",def",
            line: 1,
            col: 4,
            offset: 3
        }
    );

    assert_eq!(
        attr,
        TElementAttribute {
            name: None,
            value: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },
            source: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(
        attr.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );
}
