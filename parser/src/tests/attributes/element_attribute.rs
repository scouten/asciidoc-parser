use pretty_assertions_sorted::assert_eq;

use crate::{
    attributes::{element_attribute::ParseShorthand, ElementAttribute},
    strings::CowStr,
    tests::fixtures::attributes::TElementAttribute,
    Parser,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let p = Parser::default();

    let b1 = ElementAttribute::parse(&CowStr::from("abc"), 0, &p, ParseShorthand(false))
        .0
        .unwrap()
        .0;

    let b2 = b1.clone();

    assert_eq!(b1, b2);
}

#[test]
fn empty_source() {
    let p = Parser::default();
    let (maybe_attr, warning_types) =
        ElementAttribute::parse(&CowStr::from(""), 0, &p, ParseShorthand(false));

    assert!(maybe_attr.is_none());
    assert!(warning_types.is_empty());
}

#[test]
fn only_spaces() {
    let p = Parser::default();
    let (maybe_attr, warning_types) =
        ElementAttribute::parse(&CowStr::from("   "), 0, &p, ParseShorthand(false));

    assert!(warning_types.is_empty());

    let (element_attr, offset) = maybe_attr.unwrap();

    assert_eq!(
        element_attr,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "   ",
        }
    );

    assert!(element_attr.block_style().is_none());
    assert!(element_attr.id().is_none());
    assert!(element_attr.roles().is_empty());
    assert!(element_attr.options().is_empty());

    assert_eq!(offset, 3);

    assert!(element_attr.name().is_none());
}

#[test]
fn unquoted_and_unnamed_value() {
    let p = Parser::default();
    let (maybe_mi, warning_types) =
        ElementAttribute::parse(&CowStr::from("abc"), 0, &p, ParseShorthand(false));

    let (element_attr, offset) = maybe_mi.unwrap();
    assert!(warning_types.is_empty());

    assert_eq!(
        element_attr,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "abc",
        }
    );

    assert!(element_attr.name().is_none());
    assert!(element_attr.block_style().is_none());
    assert!(element_attr.id().is_none());
    assert!(element_attr.roles().is_empty());
    assert!(element_attr.options().is_empty());

    assert_eq!(offset, 3);
}

#[test]
fn unquoted_stops_at_comma() {
    let p = Parser::default();
    let (maybe_mi, warning_types) =
        ElementAttribute::parse(&CowStr::from("abc,def"), 0, &p, ParseShorthand(false));

    let (element_attr, offset) = maybe_mi.unwrap();
    assert!(warning_types.is_empty());

    assert_eq!(
        element_attr,
        TElementAttribute {
            name: None,
            shorthand_items: vec![],
            value: "abc",
        }
    );

    assert!(element_attr.name().is_none());
    assert!(element_attr.block_style().is_none());
    assert!(element_attr.id().is_none());
    assert!(element_attr.roles().is_empty());
    assert!(element_attr.options().is_empty());

    assert_eq!(offset, 3);
}

mod quoted_string {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        attributes::{element_attribute::ParseShorthand, ElementAttribute},
        strings::CowStr,
        tests::fixtures::attributes::TElementAttribute,
        warnings::WarningType,
        Parser,
    };

    #[test]
    fn err_unterminated_double_quote() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("\"xxx"), 0, &p, ParseShorthand(false));

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
            ElementAttribute::parse(&CowStr::from("\"abc\"def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc"
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 5);
    }

    #[test]
    fn double_quoted_with_escape() {
        let p = Parser::default();
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            &CowStr::from("\"a\\\"bc\"def"),
            0,
            &p,
            ParseShorthand(false),
        );

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\\\"bc"
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 7);
    }

    #[test]
    fn double_quoted_with_single_quote() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("\"a'bc\"def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a'bc"
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 6);
    }

    #[test]
    fn err_unterminated_single_quote() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("\'xxx"), 0, &p, ParseShorthand(false));

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
            ElementAttribute::parse(&CowStr::from("'abc'def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc"
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 5);
    }

    #[test]
    fn single_quoted_with_escape() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("'a\\'bc'def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\\'bc"
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 7);
    }

    #[test]
    fn single_quoted_with_double_quote() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("'a\"bc'def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "a\"bc"
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 6);
    }
}

mod named {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        attributes::{element_attribute::ParseShorthand, ElementAttribute},
        strings::CowStr,
        tests::fixtures::attributes::TElementAttribute,
        Parser,
    };

    #[test]
    fn simple_named_value() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc=def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: Some("abc"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(element_attr.name().unwrap(), "abc");
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 7);
    }

    #[test]
    fn ignores_spaces_around_equals() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc =  def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: Some("abc"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(element_attr.name().unwrap(), "abc");

        assert_eq!(offset, 10);
    }

    #[test]
    fn numeric_name() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("94-x =def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: Some("94-x"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(element_attr.name().unwrap(), "94-x");
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 9);
    }

    #[test]
    fn quoted_value() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc='def'g"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: Some("abc"),
                shorthand_items: vec![],
                value: "def"
            }
        );

        assert_eq!(element_attr.name().unwrap(), "abc");
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 9);
    }

    #[test]
    fn fallback_if_no_value() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc="), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc="
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 4);
    }

    #[test]
    fn fallback_if_immediate_comma() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc=,def"), 0, &p, ParseShorthand(false));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![],
                value: "abc="
            }
        );

        assert!(element_attr.name().is_none());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 4);
    }
}

mod parse_with_shorthand {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        attributes::{element_attribute::ParseShorthand, ElementAttribute},
        strings::CowStr,
        tests::fixtures::attributes::TElementAttribute,
        warnings::WarningType,
        Parser,
    };

    #[test]
    fn block_style_only() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc"), 0, &p, ParseShorthand(true));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc"],
                value: "abc"
            }
        );

        assert!(element_attr.name().is_none());
        assert_eq!(element_attr.shorthand_items(), vec!["abc"]);
        assert_eq!(element_attr.block_style().unwrap(), "abc");
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 3);
    }

    #[test]
    fn ignore_if_named_attribute() {
        let p = Parser::default();
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            &CowStr::from("name=block_style#id"),
            0,
            &p,
            ParseShorthand(true),
        );

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: Some("name"),
                shorthand_items: vec![],
                value: "block_style#id"
            }
        );

        assert_eq!(element_attr.name().unwrap(), "name");
        assert!(element_attr.shorthand_items().is_empty());
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 19);
    }

    #[test]
    fn error_empty_id() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc#"), 0, &p, ParseShorthand(true));

        let (element_attr, offset) = maybe_mi.unwrap();

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc"],
                value: "abc#"
            }
        );

        assert_eq!(offset, 4);
        assert_eq!(warning_types, vec![WarningType::EmptyShorthandItem]);
    }

    #[test]
    fn error_duplicate_delimiter() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("abc##id"), 0, &p, ParseShorthand(true));

        let (element_attr, offset) = maybe_mi.unwrap();

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["abc", "#id"],
                value: "abc##id"
            }
        );

        assert_eq!(offset, 7);
        assert_eq!(warning_types, vec![WarningType::EmptyShorthandItem]);
    }

    #[test]
    fn id_only() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("#xyz"), 0, &p, ParseShorthand(true));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["#xyz"],
                value: "#xyz"
            }
        );

        assert!(element_attr.name().is_none());
        assert_eq!(element_attr.shorthand_items(), vec!["#xyz"]);
        assert!(element_attr.block_style().is_none());
        assert_eq!(element_attr.id().unwrap(), "xyz");
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 4);
    }

    #[test]
    fn one_role_only() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from(".role1"), 0, &p, ParseShorthand(true));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![".role1",],
                value: ".role1"
            }
        );

        assert!(element_attr.name().is_none());
        assert_eq!(element_attr.shorthand_items(), vec![".role1"]);
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert_eq!(element_attr.roles(), vec!("role1"));
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 6);
    }

    #[test]
    fn multiple_roles() {
        let p = Parser::default();
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            &CowStr::from(".role1.role2.role3"),
            0,
            &p,
            ParseShorthand(true),
        );

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec![".role1", ".role2", ".role3"],
                value: ".role1.role2.role3"
            }
        );

        assert!(element_attr.name().is_none());

        assert_eq!(
            element_attr.shorthand_items(),
            vec![".role1", ".role2", ".role3"]
        );

        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert_eq!(element_attr.roles(), vec!("role1", "role2", "role3",));
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 18);
    }

    #[test]
    fn one_option_only() {
        let p = Parser::default();
        let (maybe_mi, warning_types) =
            ElementAttribute::parse(&CowStr::from("%option1"), 0, &p, ParseShorthand(true));

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["%option1"],
                value: "%option1"
            }
        );

        assert!(element_attr.name().is_none());
        assert_eq!(element_attr.shorthand_items(), vec!["%option1"]);
        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert_eq!(element_attr.options(), vec!("option1"));

        assert_eq!(offset, 8);
    }

    #[test]
    fn multiple_options() {
        let p = Parser::default();
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            &CowStr::from("%option1%option2%option3"),
            0,
            &p,
            ParseShorthand(true),
        );

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["%option1", "%option2", "%option3"],
                value: "%option1%option2%option3"
            }
        );

        assert!(element_attr.name().is_none());

        assert_eq!(
            element_attr.shorthand_items(),
            vec!["%option1", "%option2", "%option3"]
        );

        assert!(element_attr.block_style().is_none());
        assert!(element_attr.id().is_none());
        assert!(element_attr.roles().is_empty());
        assert_eq!(
            element_attr.options(),
            vec!("option1", "option2", "option3")
        );

        assert_eq!(offset, 24);
    }

    #[test]
    fn block_style_and_id() {
        let p = Parser::default();
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            &CowStr::from("appendix#custom-id"),
            0,
            &p,
            ParseShorthand(true),
        );

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["appendix", "#custom-id"],
                value: "appendix#custom-id"
            }
        );

        assert!(element_attr.name().is_none());
        assert_eq!(
            element_attr.shorthand_items(),
            vec!["appendix", "#custom-id"]
        );
        assert_eq!(element_attr.block_style().unwrap(), "appendix",);
        assert_eq!(element_attr.id().unwrap(), "custom-id",);
        assert!(element_attr.roles().is_empty());
        assert!(element_attr.options().is_empty());

        assert_eq!(offset, 18);
    }

    #[test]
    fn id_role_and_option() {
        let p = Parser::default();
        let (maybe_mi, warning_types) = ElementAttribute::parse(
            &CowStr::from("#rules.prominent%incremental"),
            0,
            &p,
            ParseShorthand(true),
        );

        let (element_attr, offset) = maybe_mi.unwrap();
        assert!(warning_types.is_empty());

        assert_eq!(
            element_attr,
            TElementAttribute {
                name: None,
                shorthand_items: vec!["#rules", ".prominent", "%incremental"],
                value: "#rules.prominent%incremental"
            }
        );

        assert!(element_attr.name().is_none());

        assert_eq!(
            element_attr.shorthand_items(),
            vec!["#rules", ".prominent", "%incremental"]
        );

        assert!(element_attr.block_style().is_none());
        assert_eq!(element_attr.id().unwrap(), "rules");
        assert_eq!(element_attr.roles(), vec!("prominent"));
        assert_eq!(element_attr.options(), vec!("incremental"));

        assert_eq!(offset, 28);
    }
}
