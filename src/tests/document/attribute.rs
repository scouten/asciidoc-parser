use pretty_assertions_sorted::assert_eq;

use crate::{
    document::Attribute,
    tests::fixtures::{
        document::{TAttribute, TAttributeValue},
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
    let (rem, block) = Attribute::parse(Span::new(":foo: bar\nblah", true)).unwrap();

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
        block,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TAttributeValue::Value(TSpan {
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
}

#[test]
fn no_value() {
    let (rem, block) = Attribute::parse(Span::new(":foo:\nblah", true)).unwrap();

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
        block,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TAttributeValue::Set,
            source: TSpan {
                data: ":foo:\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );
}

#[test]
fn unset_prefix() {
    let (rem, block) = Attribute::parse(Span::new(":!foo:\nblah", true)).unwrap();

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
        block,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 3,
                offset: 2,
            },
            value: TAttributeValue::Unset,
            source: TSpan {
                data: ":!foo:\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );
}

#[test]
fn unset_postfix() {
    let (rem, block) = Attribute::parse(Span::new(":foo!:\nblah", true)).unwrap();

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
        block,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TAttributeValue::Unset,
            source: TSpan {
                data: ":foo!:\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );
}
