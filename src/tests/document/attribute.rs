use pretty_assertions_sorted::assert_eq;

use crate::{
    document::Attribute,
    tests::fixtures::{
        document::{TAttribute, TAttributeValue, TRawAttributeValue},
        TSpan,
    },
    Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let h1 = Attribute::parse(Span::new(":foo: bar", true)).unwrap();
    let h2 = h1.clone();
    assert_eq!(h1, h2);
}

#[test]
fn simple_value() {
    let (rem, attr) = Attribute::parse(Span::new(":foo: bar\nblah", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 10
        }
    );

    assert_eq!(
        attr,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Value(TSpan {
                data: "bar",
                line: 1,
                col: 7,
                offset: 6,
            }),
            source: TSpan {
                data: ":foo: bar\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(attr.value(), TAttributeValue::Value("bar"));
}

#[test]
fn no_value() {
    let (rem, attr) = Attribute::parse(Span::new(":foo:\nblah", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 6
        }
    );

    assert_eq!(
        attr,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Set,
            source: TSpan {
                data: ":foo:\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(attr.value(), TAttributeValue::Set);
}

#[test]
fn unset_prefix() {
    let (rem, attr) = Attribute::parse(Span::new(":!foo:\nblah", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );

    assert_eq!(
        attr,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 3,
                offset: 2,
            },
            value: TRawAttributeValue::Unset,
            source: TSpan {
                data: ":!foo:\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(attr.value(), TAttributeValue::Unset);
}

#[test]
fn unset_postfix() {
    let (rem, attr) = Attribute::parse(Span::new(":foo!:\nblah", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );

    assert_eq!(
        attr,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Unset,
            source: TSpan {
                data: ":foo!:\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(attr.value(), TAttributeValue::Unset);
}

#[test]
fn err_unset_prefix_and_postfix() {
    let err: nom::Err<nom::error::Error<nom_span::Spanned<&str>>> =
        Attribute::parse(Span::new(":!foo!:\nblah", true)).unwrap_err();

    if let nom::Err::Error(e) = err {
        assert_eq!(
            e.input,
            TSpan {
                data: "!:",
                line: 1,
                col: 6,
                offset: 5,
            }
        );

        assert_eq!(e.code, nom::error::ErrorKind::Tag);
    } else {
        panic!("Unexpected error: {err:#?}");
    }
}

#[test]
fn err_invalid_ident1() {
    let err: nom::Err<nom::error::Error<nom_span::Spanned<&str>>> =
        Attribute::parse(Span::new(":@invalid:\nblah", true)).unwrap_err();

    if let nom::Err::Error(e) = err {
        assert_eq!(
            e.input,
            TSpan {
                data: "@invalid:",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(e.code, nom::error::ErrorKind::Tag);
    } else {
        panic!("Unexpected error: {err:#?}");
    }
}

#[test]
fn err_invalid_ident2() {
    let err: nom::Err<nom::error::Error<nom_span::Spanned<&str>>> =
        Attribute::parse(Span::new(":invalid@:\nblah", true)).unwrap_err();

    if let nom::Err::Error(e) = err {
        assert_eq!(
            e.input,
            TSpan {
                data: "@:",
                line: 1,
                col: 9,
                offset: 8,
            }
        );

        assert_eq!(e.code, nom::error::ErrorKind::Tag);
    } else {
        panic!("Unexpected error: {err:#?}");
    }
}

#[test]
fn err_invalid_ident3() {
    let err: nom::Err<nom::error::Error<nom_span::Spanned<&str>>> =
        Attribute::parse(Span::new(":-invalid:\nblah", true)).unwrap_err();

    if let nom::Err::Error(e) = err {
        assert_eq!(
            e.input,
            TSpan {
                data: "-invalid:",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(e.code, nom::error::ErrorKind::Tag);
    } else {
        panic!("Unexpected error: {err:#?}");
    }
}

#[test]
fn value_with_continuation() {
    let (rem, attr) = Attribute::parse(Span::new(":foo: bar +\nblah", true)).unwrap();

    assert_eq!(
        rem,
        TSpan {
            data: "",
            line: 2,
            col: 5,
            offset: 16
        }
    );

    assert_eq!(
        attr,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Value(TSpan {
                data: "bar +\nblah",
                line: 1,
                col: 7,
                offset: 6,
            }),
            source: TSpan {
                data: ":foo: bar +\nblah",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(attr.value(), TAttributeValue::Value("bar\nblah"));
}
