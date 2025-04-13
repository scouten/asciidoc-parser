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
    let h1 = Attribute::parse(Span::new(":foo: bar")).unwrap();
    let h2 = h1.clone();
    assert_eq!(h1, h2);
}

#[test]
fn simple_value() {
    let mi = Attribute::parse(Span::new(":foo: bar\nblah")).unwrap();

    assert_eq!(
        mi.item,
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

    assert_eq!(mi.item.value(), TAttributeValue::Value("bar"));

    assert_eq!(
        mi.after,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 10
        }
    );
}

#[test]
fn no_value() {
    let mi = Attribute::parse(Span::new(":foo:\nblah")).unwrap();

    assert_eq!(
        mi.item,
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

    assert_eq!(mi.item.value(), TAttributeValue::Set);

    assert_eq!(
        mi.after,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 6
        }
    );
}

#[test]
fn name_with_hyphens() {
    let mi = Attribute::parse(Span::new(":name-with-hyphen:")).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "name-with-hyphen",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Set,
            source: TSpan {
                data: ":name-with-hyphen:",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TAttributeValue::Set);

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 19,
            offset: 18
        }
    );
}

#[test]
fn unset_prefix() {
    let mi = Attribute::parse(Span::new(":!foo:\nblah")).unwrap();

    assert_eq!(
        mi.item,
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

    assert_eq!(mi.item.value(), TAttributeValue::Unset);

    assert_eq!(
        mi.after,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );
}

#[test]
fn unset_postfix() {
    let mi = Attribute::parse(Span::new(":foo!:\nblah")).unwrap();

    assert_eq!(
        mi.item,
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

    assert_eq!(mi.item.value(), TAttributeValue::Unset);

    assert_eq!(
        mi.after,
        TSpan {
            data: "blah",
            line: 2,
            col: 1,
            offset: 7
        }
    );
}

#[test]
fn err_unset_prefix_and_postfix() {
    assert!(Attribute::parse(Span::new(":!foo!:\nblah")).is_none());
}

#[test]
fn err_invalid_ident1() {
    assert!(Attribute::parse(Span::new(":@invalid:\nblah")).is_none());
}

#[test]
fn err_invalid_ident2() {
    assert!(Attribute::parse(Span::new(":invalid@:\nblah")).is_none());
}

#[test]
fn err_invalid_ident3() {
    assert!(Attribute::parse(Span::new(":-invalid:\nblah")).is_none());
}

#[test]
fn value_with_soft_wrap() {
    let mi = Attribute::parse(Span::new(":foo: bar \\\n blah")).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Value(TSpan {
                data: "bar \\\n blah",
                line: 1,
                col: 7,
                offset: 6,
            }),
            source: TSpan {
                data: ":foo: bar \\\n blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TAttributeValue::Value("bar blah"));

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 2,
            col: 6,
            offset: 17
        }
    );
}

#[test]
fn value_with_hard_wrap() {
    let mi = Attribute::parse(Span::new(":foo: bar + \\\n blah")).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "foo",
                line: 1,
                col: 2,
                offset: 1,
            },
            value: TRawAttributeValue::Value(TSpan {
                data: "bar + \\\n blah",
                line: 1,
                col: 7,
                offset: 6,
            }),
            source: TSpan {
                data: ":foo: bar + \\\n blah",
                line: 1,
                col: 1,
                offset: 0,
            }
        }
    );

    assert_eq!(mi.item.value(), TAttributeValue::Value("bar\nblah"));

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 2,
            col: 6,
            offset: 19
        }
    );
}
