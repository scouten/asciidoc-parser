use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::ElementAttribute,
    tests::fixtures::{attributes::TElementAttribute, TSpan},
    HasSpan, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let b1 = ElementAttribute::parse(Span::new("abc")).item.unwrap();
    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn empty_source() {
    assert!(ElementAttribute::parse(Span::new(""))
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn only_spaces() {
    let mi = ElementAttribute::parse(Span::new("   "))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
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

    assert!(mi.item.block_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());

    assert_eq!(
        mi.after,
        TSpan {
            data: "",
            line: 1,
            col: 4,
            offset: 3
        }
    );

    assert!(mi.item.name().is_none());

    assert_eq!(
        mi.item.span(),
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
    let mi = ElementAttribute::parse(Span::new("abc"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
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

    assert!(mi.item.name().is_none());
    assert!(mi.item.block_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
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
    let mi = ElementAttribute::parse(Span::new("abc,def"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
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

    assert!(mi.item.name().is_none());
    assert!(mi.item.block_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());

    assert_eq!(
        mi.item.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0,
        }
    );

    assert_eq!(
        mi.after,
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
        tests::fixtures::{attributes::TElementAttribute, warnings::TWarning, TSpan},
        warnings::WarningType,
        HasSpan, Span,
    };

    #[test]
    fn err_unterminated_double_quote() {
        let maw = ElementAttribute::parse(Span::new("\"xxx"));

        assert!(maw.item.is_none());

        assert_eq!(
            maw.warnings,
            vec![TWarning {
                source: TSpan {
                    data: "\"xxx",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warning: WarningType::AttributeValueMissingTerminatingQuote,
            }]
        );
    }

    #[test]
    fn double_quoted_string() {
        let mi = ElementAttribute::parse(Span::new("\"abc\"def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "\"abc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("\"a\\\"bc\"def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "\"a\\\"bc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("\"a'bc\"def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "\"a'bc\"",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let maw = ElementAttribute::parse(Span::new("\'xxx"));

        assert!(maw.item.is_none());

        assert_eq!(
            maw.warnings,
            vec![TWarning {
                source: TSpan {
                    data: "\'xxx",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warning: WarningType::AttributeValueMissingTerminatingQuote,
            }]
        );
    }

    #[test]
    fn single_quoted_string() {
        let mi = ElementAttribute::parse(Span::new("'abc'def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "'abc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("'a\\'bc'def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "'a\\'bc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("'a\"bc'def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "'a\"bc'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("abc=def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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
            mi.item.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "abc=def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("abc =  def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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
            mi.item.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "abc =  def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("94-x =def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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
            mi.item.name().unwrap(),
            TSpan {
                data: "94-x",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "94-x =def",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("abc='def'g"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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
            mi.item.name().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "abc='def'",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("abc="))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "abc=",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = ElementAttribute::parse(Span::new("abc=,def"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "abc=",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        tests::fixtures::{attributes::TElementAttribute, warnings::TWarning, TSpan},
        warnings::WarningType,
        HasSpan, Span,
    };

    #[test]
    fn block_style_only() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new("abc"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
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

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }]
        );

        assert_eq!(
            mi.item.block_style().unwrap(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }

    #[test]
    fn ignore_if_named_attribute() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new("name=block_style#id"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: Some(TSpan {
                    data: "name",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                shorthand_items: vec![],
                value: TSpan {
                    data: "block_style#id",
                    line: 1,
                    col: 6,
                    offset: 5,
                },
                source: TSpan {
                    data: "name=block_style#id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            mi.item.name().unwrap(),
            TSpan {
                data: "name",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert!(mi.item.shorthand_items().is_empty());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "name=block_style#id",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 20,
                offset: 19
            }
        );
    }

    #[test]
    fn error_empty_id() {
        let maw = ElementAttribute::parse_with_shorthand(Span::new("abc#"));

        let mi = maw.item.unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }],
                value: TSpan {
                    data: "abc#",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "abc#",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            maw.warnings,
            vec![TWarning {
                source: TSpan {
                    data: "#",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                warning: WarningType::EmptyShorthandItem,
            }]
        );
    }

    #[test]
    fn error_duplicate_delimiter() {
        let maw = ElementAttribute::parse_with_shorthand(Span::new("abc##id"));

        let mi = maw.item.unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![
                    TSpan {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    TSpan {
                        data: "#id",
                        line: 1,
                        col: 5,
                        offset: 4,
                    }
                ],
                value: TSpan {
                    data: "abc##id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "abc##id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            maw.warnings,
            vec![TWarning {
                source: TSpan {
                    data: "#",
                    line: 1,
                    col: 4,
                    offset: 3,
                },
                warning: WarningType::EmptyShorthandItem,
            }]
        );
    }

    #[test]
    fn id_only() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new("#xyz"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: "#xyz",
                    line: 1,
                    col: 1,
                    offset: 0,
                }],
                value: TSpan {
                    data: "#xyz",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "#xyz",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![TSpan {
                data: "#xyz",
                line: 1,
                col: 1,
                offset: 0,
            }]
        );

        assert!(mi.item.block_style().is_none());

        assert_eq!(
            mi.item.id().unwrap(),
            TSpan {
                data: "xyz",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "#xyz",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn one_role_only() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new(".role1"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: ".role1",
                    line: 1,
                    col: 1,
                    offset: 0,
                }],
                value: TSpan {
                    data: ".role1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: ".role1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![TSpan {
                data: ".role1",
                line: 1,
                col: 1,
                offset: 0,
            }]
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());

        assert_eq!(
            mi.item.roles(),
            vec!(TSpan {
                data: "role1",
                line: 1,
                col: 2,
                offset: 1,
            })
        );

        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ".role1",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn multiple_roles() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new(".role1.role2.role3"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![
                    TSpan {
                        data: ".role1",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    TSpan {
                        data: ".role2",
                        line: 1,
                        col: 7,
                        offset: 6,
                    },
                    TSpan {
                        data: ".role3",
                        line: 1,
                        col: 13,
                        offset: 12,
                    }
                ],
                value: TSpan {
                    data: ".role1.role2.role3",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: ".role1.role2.role3",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![
                TSpan {
                    data: ".role1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                TSpan {
                    data: ".role2",
                    line: 1,
                    col: 7,
                    offset: 6,
                },
                TSpan {
                    data: ".role3",
                    line: 1,
                    col: 13,
                    offset: 12,
                }
            ]
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());

        assert_eq!(
            mi.item.roles(),
            vec!(
                TSpan {
                    data: "role1",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                TSpan {
                    data: "role2",
                    line: 1,
                    col: 8,
                    offset: 7,
                },
                TSpan {
                    data: "role3",
                    line: 1,
                    col: 14,
                    offset: 13,
                }
            )
        );

        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ".role1.role2.role3",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

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
    fn one_option_only() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new("%option1"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![TSpan {
                    data: "%option1",
                    line: 1,
                    col: 1,
                    offset: 0,
                }],
                value: TSpan {
                    data: "%option1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "%option1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![TSpan {
                data: "%option1",
                line: 1,
                col: 1,
                offset: 0,
            }]
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());

        assert_eq!(
            mi.item.options(),
            vec!(TSpan {
                data: "option1",
                line: 1,
                col: 2,
                offset: 1,
            })
        );

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "%option1",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 9,
                offset: 8
            }
        );
    }

    #[test]
    fn multiple_options() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new("%option1%option2%option3"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![
                    TSpan {
                        data: "%option1",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    TSpan {
                        data: "%option2",
                        line: 1,
                        col: 9,
                        offset: 8,
                    },
                    TSpan {
                        data: "%option3",
                        line: 1,
                        col: 17,
                        offset: 16,
                    }
                ],
                value: TSpan {
                    data: "%option1%option2%option3",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "%option1%option2%option3",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![
                TSpan {
                    data: "%option1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                TSpan {
                    data: "%option2",
                    line: 1,
                    col: 9,
                    offset: 8,
                },
                TSpan {
                    data: "%option3",
                    line: 1,
                    col: 17,
                    offset: 16,
                }
            ]
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());

        assert_eq!(
            mi.item.options(),
            vec!(
                TSpan {
                    data: "option1",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                TSpan {
                    data: "option2",
                    line: 1,
                    col: 10,
                    offset: 9,
                },
                TSpan {
                    data: "option3",
                    line: 1,
                    col: 18,
                    offset: 17,
                }
            )
        );

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "%option1%option2%option3",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 25,
                offset: 24
            }
        );
    }

    #[test]
    fn block_style_and_id() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new("appendix#custom-id"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![
                    TSpan {
                        data: "appendix",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    TSpan {
                        data: "#custom-id",
                        line: 1,
                        col: 9,
                        offset: 8,
                    },
                ],
                value: TSpan {
                    data: "appendix#custom-id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "appendix#custom-id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![
                TSpan {
                    data: "appendix",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                TSpan {
                    data: "#custom-id",
                    line: 1,
                    col: 9,
                    offset: 8,
                },
            ]
        );

        assert_eq!(
            mi.item.block_style().unwrap(),
            TSpan {
                data: "appendix",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.item.id().unwrap(),
            TSpan {
                data: "custom-id",
                line: 1,
                col: 10,
                offset: 9,
            }
        );

        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "appendix#custom-id",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

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
    fn id_role_and_option() {
        let mi = ElementAttribute::parse_with_shorthand(Span::new("#rules.prominent%incremental"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![
                    TSpan {
                        data: "#rules",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    TSpan {
                        data: ".prominent",
                        line: 1,
                        col: 7,
                        offset: 6,
                    },
                    TSpan {
                        data: "%incremental",
                        line: 1,
                        col: 17,
                        offset: 16,
                    },
                ],
                value: TSpan {
                    data: "#rules.prominent%incremental",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "#rules.prominent%incremental",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(
            mi.item.shorthand_items(),
            &vec![
                TSpan {
                    data: "#rules",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                TSpan {
                    data: ".prominent",
                    line: 1,
                    col: 7,
                    offset: 6,
                },
                TSpan {
                    data: "%incremental",
                    line: 1,
                    col: 17,
                    offset: 16,
                },
            ]
        );

        assert!(mi.item.block_style().is_none());

        assert_eq!(
            mi.item.id().unwrap(),
            TSpan {
                data: "rules",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(
            mi.item.roles(),
            vec!(TSpan {
                data: "prominent",
                line: 1,
                col: 8,
                offset: 7,
            })
        );

        assert_eq!(
            mi.item.options(),
            vec!(TSpan {
                data: "incremental",
                line: 1,
                col: 18,
                offset: 17,
            })
        );

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "#rules.prominent%incremental",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 29,
                offset: 28
            }
        );
    }
}
