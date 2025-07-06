use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::{element_attribute::ParseShorthand, ElementAttribute},
    tests::fixtures::{attributes::TElementAttribute, TSpan},
    Parser, Span,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let p = Parser::default();
    let b1 = ElementAttribute::parse(Span::new("abc"), &p, ParseShorthand(false))
        .0
        .unwrap();
    let b2 = b1.item.clone();
    assert_eq!(b1.item, b2);
}

#[test]
fn empty_source() {
    let p = Parser::default();
    let (maybe_attr, warning_types) =
        ElementAttribute::parse(Span::new(""), &p, ParseShorthand(false));

    assert!(maybe_attr.is_none());
    assert!(warning_types.is_empty());
}

#[test]
fn only_spaces() {
    let p = Parser::default();
    let (maybe_attr, warning_types) =
        ElementAttribute::parse(Span::new("   "), &p, ParseShorthand(false));

    assert!(warning_types.is_empty());

    let mi = maybe_attr.unwrap();

    assert_eq!(
        mi.item,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "   ",
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
}

#[test]
fn unquoted_and_unnamed_value() {
    let p = Parser::default();
    let (maybe_mi, warning_types) =
        ElementAttribute::parse(Span::new("abc"), &p, ParseShorthand(false));

    let mi = maybe_mi.unwrap();
    assert!(warning_types.is_empty());

    assert_eq!(
        mi.item,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "abc",
        }
    );

    assert!(mi.item.name().is_none());
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
}

#[test]
fn unquoted_stops_at_comma() {
    let p = Parser::default();
    let (maybe_mi, warning_types) =
        ElementAttribute::parse(Span::new("abc,def"), &p, ParseShorthand(false));

    let mi = maybe_mi.unwrap();
    assert!(warning_types.is_empty());

    assert_eq!(
        mi.item,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "abc",
        }
    );

    assert!(mi.item.name().is_none());
    assert!(mi.item.block_style().is_none());
    assert!(mi.item.id().is_none());
    assert!(mi.item.roles().is_empty());
    assert!(mi.item.options().is_empty());

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
        attributes::{element_attribute::ParseShorthand, ElementAttribute},
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        warnings::WarningType,
        Parser, Span,
    };

    #[test]
    fn err_unterminated_double_quote() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("\"xxx"), &p, ParseShorthand(false));

        assert!(maybe_mi.is_none());

        assert_eq!(
            warning_types,
            vec![WarningType::AttributeValueMissingTerminatingQuote]
        );
    }

    #[test]
    fn double_quoted_string() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("\"abc\"def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc"
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("\"a\\\"bc\"def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\\\"bc"
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("\"a'bc\"def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a'bc"
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("\'xxx"), &p, ParseShorthand(false));

        assert!(maybe_mi.is_none());

        assert_eq!(
            warning_types,
            vec![WarningType::AttributeValueMissingTerminatingQuote]
        );
    }

    #[test]
    fn single_quoted_string() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("'abc'def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc"
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("'a\\'bc'def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\\'bc"
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("'a\"bc'def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\"bc"
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        attributes::{element_attribute::ParseShorthand, ElementAttribute},
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        Parser, Span,
    };

    #[test]
    fn simple_named_value() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc=def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: Some("abc"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(mi.item.name().unwrap(), "abc");
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc =  def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: Some("abc"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(mi.item.name().unwrap(), "abc");

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("94-x =def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: Some("94-x"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(mi.item.name().unwrap(), "94-x");

        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc='def'g"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: Some("abc"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(mi.item.name().unwrap(), "abc");
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc="), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc="
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc=,def"), &p, ParseShorthand(false));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc="
            }
        );

        assert!(mi.item.name().is_none());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        attributes::{element_attribute::ParseShorthand, ElementAttribute},
        tests::fixtures::{attributes::TElementAttribute, TSpan},
        warnings::WarningType,
        Parser, Span,
    };

    #[test]
    fn block_style_only() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc"],
                value: "abc"
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec!["abc"]);
        assert_eq!(mi.item.block_style().unwrap(), "abc");
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
    }

    #[test]
    fn ignore_if_named_attribute() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("name=block_style#id"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: Some("name"),
                shorthand_items: vec![],
                value: "block_style#id"
            }
        );

        assert_eq!(mi.item.name().unwrap(), "name");
        assert!(mi.item.shorthand_items().is_empty());
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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

    #[ignore]
    #[test]
    fn error_empty_id() {
        // Disabling this test for now (05 Jul 2025): May not be possible to show this
        // error after refactoring Attrlist to apply attribute value substitutions.
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc#"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc"],
                value: "abc#"
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

        assert_eq!(warning_types, vec![WarningType::EmptyShorthandItem]);
    }

    #[ignore]
    #[test]
    fn error_duplicate_delimiter() {
        // Disabling this test for now (05 Jul 2025): May not be possible to show this
        // error after refactoring Attrlist to apply attribute value substitutions.
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("abc##id"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc", "#id"],
                value: "abc##id"
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

        assert_eq!(warning_types, vec![WarningType::EmptyShorthandItem]);
    }

    #[test]
    fn id_only() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("#xyz"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["#xyz"],
                value: "#xyz"
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec!["#xyz"]);
        assert!(mi.item.block_style().is_none());
        assert_eq!(mi.item.id().unwrap(), "xyz");
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new(".role1"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![".role1",],
                value: ".role1"
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec![".role1"]);
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert_eq!(mi.item.roles(), vec!("role1"));
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new(".role1.role2.role3"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec![".role1", ".role2", ".role3"],
                value: ".role1.role2.role3"
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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("%option1"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["%option1"],
                value: "%option1"
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec!["%option1"]);
        assert!(mi.item.block_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert_eq!(mi.item.options(), vec!("option1"));

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
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            Span::new("%option1%option2%option3"),
            &p,
            ParseShorthand(true),
        );

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["%option1", "%option2", "%option3"],
                value: "%option1%option2%option3"
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
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(Span::new("appendix#custom-id"), &p, ParseShorthand(true));

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["appendix", "#custom-id"],
                value: "appendix#custom-id"
            }
        );

        assert!(mi.item.name().is_none());
        assert_eq!(mi.item.shorthand_items(), vec!["appendix", "#custom-id"]);
        assert_eq!(mi.item.block_style().unwrap(), "appendix",);
        assert_eq!(mi.item.id().unwrap(), "custom-id",);
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

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
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            Span::new("#rules.prominent%incremental"),
            &p,
            ParseShorthand(true),
        );

        let mi = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            mi.item,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["#rules", ".prominent", "%incremental"],
                value: "#rules.prominent%incremental"
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
