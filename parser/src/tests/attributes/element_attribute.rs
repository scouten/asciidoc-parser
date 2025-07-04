use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::ElementAttribute,
    tests::fixtures::{attributes::TElementAttribute, TSpan},
    HasSpan, Parser, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let p = Parser::default();
    let b1 = ElementAttribute::parse(Span::new("abc"), &p).item.unwrap();
    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn empty_source() {
    let p = Parser::default();
    assert!(ElementAttribute::parse(Span::new(""), &p)
        .unwrap_if_no_warnings()
        .is_none());
}

#[test]
fn only_spaces() {
    let p = Parser::default();
    let mi = ElementAttribute::parse(Span::new("   "), &p)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "   ",
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
    let p = Parser::default();
    let mi = ElementAttribute::parse(Span::new("abc"), &p)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "abc",
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
    let p = Parser::default();
    let mi = ElementAttribute::parse(Span::new("abc,def"), &p)
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(
        mi.item,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "abc",
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
        HasSpan, Parser, Span,
    };

    #[test]
    fn err_unterminated_double_quote() {
        let p = Parser::default();
        let maw = ElementAttribute::parse(Span::new("\"xxx"), &p);

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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("\"abc\"def"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("\"a\\\"bc\"def"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\\\"bc",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("\"a'bc\"def"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a'bc",
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
        let p = Parser::default();
        let maw = ElementAttribute::parse(Span::new("\'xxx"), &p);

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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("'abc'def"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("'a\\'bc'def"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\\'bc",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("'a\"bc'def"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\"bc",
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
        HasSpan, Parser, Span,
    };

    #[test]
    fn simple_named_value() {
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("abc=def"), &p)
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
                value: "def",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("abc =  def"), &p)
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
                value: "def",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("94-x =def"), &p)
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
                value: "def",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("abc='def'g"), &p)
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
                value: "def",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("abc="), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc=",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse(Span::new("abc=,def"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc=",
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
        HasSpan, Parser, Span,
    };

    #[test]
    fn block_style_only() {
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new("abc"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc"],
                value: "abc",
                source: TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());

        assert_eq!(mi.item.shorthand_items(), vec!["abc"]);

        assert_eq!(mi.item.block_style().unwrap(), "abc");

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
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new("name=block_style#id"), &p)
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
                value: "block_style#id",
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
        let p = Parser::default();
        let maw = ElementAttribute::parse_with_shorthand(Span::new("abc#"), &p);

        let mi = maw.item.unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc"],
                value: "abc#",
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
        let p = Parser::default();
        let maw = ElementAttribute::parse_with_shorthand(Span::new("abc##id"), &p);

        let mi = maw.item.unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc", "#id"],
                value: "abc##id",
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
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new("#xyz"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["#xyz"],
                value: "#xyz",
                source: TSpan {
                    data: "#xyz",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec!["#xyz"]);
        assert!(mi.item.block_style().is_none());
        assert_eq!(mi.item.id().unwrap(), "xyz");
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
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new(".role1"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![".role1",],
                value: ".role1",
                source: TSpan {
                    data: ".role1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec![".role1"]);
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert_eq!(mi.item.roles(), vec!("role1"));
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
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new(".role1.role2.role3"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![".role1", ".role2", ".role3"],
                value: ".role1.role2.role3",
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
            vec![".role1", ".role2", ".role3"]
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert_eq!(mi.item.roles(), vec!("role1", "role2", "role3",));
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
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new("%option1"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["%option1"],
                value: "%option1",
                source: TSpan {
                    data: "%option1",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec!["%option1"]);
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert_eq!(mi.item.options(), vec!("option1"));

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
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new("%option1%option2%option3"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["%option1", "%option2", "%option3"],
                value: "%option1%option2%option3",
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
            vec!["%option1", "%option2", "%option3"]
        );

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert_eq!(mi.item.options(), vec!("option1", "option2", "option3"));

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
        let p = Parser::default();
        let mi = ElementAttribute::parse_with_shorthand(Span::new("appendix#custom-id"), &p)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["appendix", "#custom-id"],
                value: "appendix#custom-id",
                source: TSpan {
                    data: "appendix#custom-id",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec!["appendix", "#custom-id"]);
        assert_eq!(mi.item.block_style().unwrap(), "appendix",);
        assert_eq!(mi.item.id().unwrap(), "custom-id",);
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
        let p = Parser::default();
        let mi =
            ElementAttribute::parse_with_shorthand(Span::new("#rules.prominent%incremental"), &p)
                .unwrap_if_no_warnings()
                .unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["#rules", ".prominent", "%incremental"],
                value: "#rules.prominent%incremental",
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
            vec!["#rules", ".prominent", "%incremental"]
        );

        assert!(mi.item.block_style().is_none());
        assert_eq!(mi.item.id().unwrap(), "rules");
        assert_eq!(mi.item.roles(), vec!("prominent"));
        assert_eq!(mi.item.options(), vec!("incremental"));

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
