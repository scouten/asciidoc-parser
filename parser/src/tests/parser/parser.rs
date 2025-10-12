use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    attributes::Attrlist,
    blocks::{Block, IsBlock},
    parser::{
        CharacterReplacementType, IconRenderParams, ImageRenderParams, InlineSubstitutionRenderer,
        LinkRenderParams, ModificationContext, QuoteScope, QuoteType, SpecialCharacter,
    },
    tests::prelude::*,
    warnings::WarningType,
};

#[test]
fn default_is_unset() {
    let p = Parser::default();
    assert_eq!(p.attribute_value("foo"), InterpretedValue::Unset);
}

#[test]
fn with_intrinsic_attribute() {
    let p = Parser::default().with_intrinsic_attribute("foo", "bar", ModificationContext::Anywhere);

    assert_eq!(p.attribute_value("foo"), InterpretedValue::Value("bar"));
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

    assert_eq!(parser.attribute_value("sp"), InterpretedValue::Value(" "));
}

/// A simple test renderer that modifies special characters differently
/// from the default HTML renderer.
#[derive(Debug)]
struct TestRenderer;

impl InlineSubstitutionRenderer for TestRenderer {
    fn render_special_character(&self, type_: SpecialCharacter, dest: &mut String) {
        // Custom rendering: wrap special characters in brackets.
        match type_ {
            SpecialCharacter::Lt => dest.push_str("[LT]"),
            SpecialCharacter::Gt => dest.push_str("[GT]"),
            SpecialCharacter::Ampersand => dest.push_str("[AMP]"),
        }
    }

    fn render_quoted_substitition(
        &self,
        _type_: QuoteType,
        _scope: QuoteScope,
        _attrlist: Option<Attrlist<'_>>,
        _id: Option<String>,
        body: &str,
        dest: &mut String,
    ) {
        dest.push_str(body);
    }

    fn render_character_replacement(&self, _type_: CharacterReplacementType, dest: &mut String) {
        dest.push_str("[CHAR]");
    }

    fn render_line_break(&self, dest: &mut String) {
        dest.push_str("[BR]");
    }

    fn render_image(&self, _params: &ImageRenderParams, dest: &mut String) {
        dest.push_str("[IMAGE]");
    }

    fn image_uri(
        &self,
        target_image_path: &str,
        _parser: &Parser,
        _asset_dir_key: Option<&str>,
    ) -> String {
        target_image_path.to_string()
    }

    fn render_icon(&self, _params: &IconRenderParams, dest: &mut String) {
        dest.push_str("[ICON]");
    }

    fn render_link(&self, _params: &LinkRenderParams, dest: &mut String) {
        dest.push_str("[LINK]");
    }

    fn render_anchor(&self, id: &str, _reftext: Option<String>, dest: &mut String) {
        dest.push_str(&format!("[ANCHOR:{}]", id));
    }
}

#[test]
fn with_inline_substitution_renderer() {
    let mut parser = Parser::default().with_inline_substitution_renderer(TestRenderer);

    // Parse a simple document with special characters.
    let doc = parser.parse("Hello & goodbye < world > test");

    // The document should parse successfully.
    assert_eq!(doc.warnings().count(), 0);

    // Get the first block from the document.
    let block = doc.nested_blocks().next().unwrap();

    let Block::Simple(simple_block) = block else {
        panic!("Expected simple block, got: {block:?}");
    };

    // Our custom renderer should show [AMP], [LT], and [GT] instead of HTML
    // entities.
    assert_eq!(
        simple_block.content().rendered(),
        "Hello [AMP] goodbye [LT] world [GT] test"
    );
}
