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
    let (_, b1) = ElementAttribute::parse(Span::new("abc")).unwrap();
    let b2 = b1.clone();
    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    let expected_err = Err::Error(Error::new(Span::new(""), ErrorKind::IsNot));

    let actual_err = ElementAttribute::parse(Span::new("")).unwrap_err();

    assert_eq!(expected_err, actual_err);
}

#[test]
fn only_spaces() {
    let (rem, attr) = ElementAttribute::parse(Span::new("   ")).unwrap();

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

    assert!(attr.name().is_none());

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
    let (rem, attr) = ElementAttribute::parse(Span::new("abc")).unwrap();

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

    assert!(attr.name().is_none());

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
    let (rem, attr) = ElementAttribute::parse(Span::new("abc,def")).unwrap();

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

    assert!(attr.name().is_none());

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

mod quoted_string {
    use nom::{error::ErrorKind, Err};
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        attributes::ElementAttribute,
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        HasSpan, Span,
    };

    #[test]
    fn err_unterminated_double_quote() {
        let err = ElementAttribute::parse(Span::new("\"xxx")).unwrap_err();

        let Err::Error(e) = err else {
            panic!("Expected Err::Error: {err:#?}");
        };

        assert_eq!(e.code, ErrorKind::Char);

        assert_eq!(
            e.input,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn double_quoted_string() {
        let (rem, attr) = ElementAttribute::parse(Span::new("\"abc\"def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "abc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                source: TSpan {
                    data: "\"abc\"",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "\"abc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn double_quoted_with_escape() {
        let (rem, attr) = ElementAttribute::parse(Span::new("\"a\\\"bc\"def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "a\\\"bc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                source: TSpan {
                    data: "\"a\\\"bc\"",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "\"a\\\"bc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn double_quoted_with_single_quote() {
        let (rem, attr) = ElementAttribute::parse(Span::new("\"a'bc\"def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "a'bc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                source: TSpan {
                    data: "\"a'bc\"",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "\"a'bc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn err_unterminated_single_quote() {
        let err = ElementAttribute::parse(Span::new("'xxx")).unwrap_err();

        let Err::Error(e) = err else {
            panic!("Expected Err::Error: {err:#?}");
        };

        assert_eq!(e.code, ErrorKind::Char);

        assert_eq!(
            e.input,
            TSpan {
                data: "\'xxx",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn single_quoted_string() {
        let (rem, attr) = ElementAttribute::parse(Span::new("'abc'def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "abc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                source: TSpan {
                    data: "'abc'",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "'abc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn single_quoted_with_escape() {
        let (rem, attr) = ElementAttribute::parse(Span::new("'a\\'bc'def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "a\\'bc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                source: TSpan {
                    data: "'a\\'bc'",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "'a\\'bc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn single_quoted_with_double_quote() {
        let (rem, attr) = ElementAttribute::parse(Span::new("'a\"bc'def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "a\"bc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                source: TSpan {
                    data: "'a\"bc'",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "'a\"bc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod named {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        attributes::ElementAttribute,
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        HasSpan, Span,
    };

    #[test]
    fn simple_named_value() {
        let (rem, attr) = ElementAttribute::parse(Span::new("abc=def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: Some(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                value: TSpan {
                    data: "def",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                source: TSpan {
                    data: "abc=def",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            attr.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            attr.span(),
            TSpan {
                data: "abc=def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn ignores_spaces_around_equals() {
        let (rem, attr) = ElementAttribute::parse(Span::new("abc =  def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: Some(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                value: TSpan {
                    data: "def",
                    line: 1,
                    col: 8,
                    offset: 7,
                },
                source: TSpan {
                    data: "abc =  def",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            attr.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            attr.span(),
            TSpan {
                data: "abc =  def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn numeric_name() {
        let (rem, attr) = ElementAttribute::parse(Span::new("94-x =def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 10,
                offset: 9
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: Some(TSpan {
                    data: "94-x",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                value: TSpan {
                    data: "def",
                    line: 1,
                    col: 7,
                    offset: 6,
                },
                source: TSpan {
                    data: "94-x =def",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            attr.name().unwrap(),
            TSpan {
                data: "94-x",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            attr.span(),
            TSpan {
                data: "94-x =def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn quoted_value() {
        let (rem, attr) = ElementAttribute::parse(Span::new("abc='def'g")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "g",
                line: 1,
                col: 10,
                offset: 9
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: Some(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                value: TSpan {
                    data: "def",
                    line: 1,
                    col: 6,
                    offset: 5,
                },
                source: TSpan {
                    data: "abc='def'",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            attr.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            attr.span(),
            TSpan {
                data: "abc='def'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn fallback_if_no_value() {
        let (rem, attr) = ElementAttribute::parse(Span::new("abc=")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "abc=",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "abc=",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "abc=",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn fallback_if_immediate_comma() {
        let (rem, attr) = ElementAttribute::parse(Span::new("abc=,def")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: ",def",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            attr,
            TElementAttribute {
                name: None,
                value: TSpan {
                    data: "abc=",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "abc=",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(attr.name().is_none());

        assert_eq!(
            attr.span(),
            TSpan {
                data: "abc=",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
