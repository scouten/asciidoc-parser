use crate::{
    Parser, document::InterpretedValue, parser::ModificationContext,
    tests::fixtures::document::TInterpretedValue, warnings::WarningType,
};

#[test]
fn default_is_unset() {
    let p = Parser::default();
    assert_eq!(p.attribute_value("foo"), InterpretedValue::Unset);
}

#[test]
fn with_intrinsic_attribute() {
    let p = Parser::default().with_intrinsic_attribute("foo", "bar", ModificationContext::Anywhere);

    assert_eq!(
        p.attribute_value("foo"),
        InterpretedValue::Value("bar".to_owned())
    );

    assert_eq!(p.attribute_value("foo2"), InterpretedValue::Unset);

    assert!(p.is_attribute_set("foo"));
    assert!(!p.is_attribute_set("foo2"));
    assert!(!p.is_attribute_set("xyz"));
}

#[test]
fn with_intrinsic_attribute_set() {
    let p =
        Parser::default().with_intrinsic_attribute_bool("foo", true, ModificationContext::Anywhere);

    assert_eq!(p.attribute_value("foo"), InterpretedValue::Set);
    assert_eq!(p.attribute_value("foo2"), InterpretedValue::Unset);

    assert!(p.is_attribute_set("foo"));
    assert!(!p.is_attribute_set("foo2"));
    assert!(!p.is_attribute_set("xyz"));
}

#[test]
fn with_intrinsic_attribute_unset() {
    let p = Parser::default().with_intrinsic_attribute_bool(
        "foo",
        false,
        ModificationContext::Anywhere,
    );

    assert_eq!(p.attribute_value("foo"), InterpretedValue::Unset);
    assert_eq!(p.attribute_value("foo2"), InterpretedValue::Unset);

    assert!(!p.is_attribute_set("foo"));
    assert!(!p.is_attribute_set("foo2"));
    assert!(!p.is_attribute_set("xyz"));
}

#[test]
fn can_not_override_locked_default_value() {
    let mut parser = Parser::default();

    let doc = parser.parse(":sp: not a space!");

    assert_eq!(
        doc.warnings().next().unwrap().warning,
        WarningType::AttributeValueIsLocked("sp".to_owned())
    );

    assert_eq!(parser.attribute_value("sp"), TInterpretedValue::Value(" "));
}
