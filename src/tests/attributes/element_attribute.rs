use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::ElementAttribute,
    tests::fixtures::{attributes::TElementAttribute, TSpan},
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = ElementAttribute::parse(Span::new("abc")).unwrap();
    let b2 = b1.t.clone();
    assert_eq!(b1.t, b2);
}

#[test]
fn empty_source() {
    assert!(ElementAttribute::parse(Span::new("")).is_none());
}

#[test]
fn only_spaces() {
    let pr = ElementAttribute::parse(Span::new("   ")).unwrap();

    assert_eq!(
        pr.t,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
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

    assert!(pr.t.block_style().is_none());

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 4,
            offset: 3
        }
    );

    assert!(pr.t.name().is_none());

    assert_eq!(
        pr.t.span(),
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
    let pr = ElementAttribute::parse(Span::new("abc")).unwrap();

    assert_eq!(
        pr.t,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
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

    assert!(pr.t.name().is_none());
    assert!(pr.t.block_style().is_none());

    assert_eq!(
        pr.t.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: "",
            line: 1,
            col: 4,
            offset: 3
        }
    );
}

#[test]
fn unquoted_stops_at_comma() {
    let pr = ElementAttribute::parse(Span::new("abc,def")).unwrap();

    assert_eq!(
        pr.t,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
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

    assert!(pr.t.name().is_none());
    assert!(pr.t.block_style().is_none());

    assert_eq!(
        pr.t.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        pr.rem,
        TSpan {
            data: ",def",
            line: 1,
            col: 4,
            offset: 3
        }
    );
}

mod quoted_string {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        attributes::ElementAttribute,
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        HasSpan, Span,
    };

    #[test]
    fn err_unterminated_double_quote() {
        assert!(ElementAttribute::parse(Span::new("\"xxx")).is_none());
    }

    #[test]
    fn double_quoted_string() {
        let pr = ElementAttribute::parse(Span::new("\"abc\"def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "\"abc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );
    }

    #[test]
    fn double_quoted_with_escape() {
        let pr = ElementAttribute::parse(Span::new("\"a\\\"bc\"def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "\"a\\\"bc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn double_quoted_with_single_quote() {
        let pr = ElementAttribute::parse(Span::new("\"a'bc\"def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "\"a'bc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn err_unterminated_single_quote() {
        assert!(ElementAttribute::parse(Span::new("'xxx")).is_none());
    }

    #[test]
    fn single_quoted_string() {
        let pr = ElementAttribute::parse(Span::new("'abc'def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "'abc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );
    }

    #[test]
    fn single_quoted_with_escape() {
        let pr = ElementAttribute::parse(Span::new("'a\\'bc'def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "'a\\'bc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn single_quoted_with_double_quote() {
        let pr = ElementAttribute::parse(Span::new("'a\"bc'def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "'a\"bc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
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
        let pr = ElementAttribute::parse(Span::new("abc=def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: Some(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
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
            pr.t.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc=def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn ignores_spaces_around_equals() {
        let pr = ElementAttribute::parse(Span::new("abc =  def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: Some(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
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
            pr.t.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc =  def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );
    }

    #[test]
    fn numeric_name() {
        let pr = ElementAttribute::parse(Span::new("94-x =def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: Some(TSpan {
                    data: "94-x",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
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
            pr.t.name().unwrap(),
            TSpan {
                data: "94-x",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "94-x =def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 10,
                offset: 9
            }
        );
    }

    #[test]
    fn quoted_value() {
        let pr = ElementAttribute::parse(Span::new("abc='def'g")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: Some(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
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
            pr.t.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc='def'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "g",
                line: 1,
                col: 10,
                offset: 9
            }
        );
    }

    #[test]
    fn fallback_if_no_value() {
        let pr = ElementAttribute::parse(Span::new("abc=")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc=",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn fallback_if_immediate_comma() {
        let pr = ElementAttribute::parse(Span::new("abc=,def")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
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

        assert!(pr.t.name().is_none());
        assert!(pr.t.block_style().is_none());

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc=",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ",def",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }
}

mod parse_with_shorthand {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        attributes::ElementAttribute,
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        HasSpan, Span,
    };

    #[test]
    fn block_style_only() {
        let pr = ElementAttribute::parse_with_shorthand(Span::new("abc")).unwrap();

        assert_eq!(
            pr.t,
            TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }],
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

        assert!(pr.t.name().is_none());

        assert_eq!(
            pr.t.shorthand_items(),
            &vec![TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }]
        );

        assert_eq!(
            pr.t.block_style().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.t.span(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }
}
