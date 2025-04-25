use crate::{document::InterpretedValue, parser::ModificationContext, strings::CowStr, Parser};

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
        InterpretedValue::Value(CowStr::from("bar"))
    );

    assert_eq!(p.attribute_value("foo2"), InterpretedValue::Unset);
}

#[test]
fn with_intrinsic_attribute_set() {
    let p =
        Parser::default().with_intrinsic_attribute_bool("foo", true, ModificationContext::Anywhere);

    assert_eq!(p.attribute_value("foo"), InterpretedValue::Set);
    assert_eq!(p.attribute_value("foo2"), InterpretedValue::Unset);
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
}
