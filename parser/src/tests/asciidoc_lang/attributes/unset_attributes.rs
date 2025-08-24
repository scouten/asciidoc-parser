use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/attributes/pages/unset-attributes.adoc");

non_normative!(
    r#"
= Unset Document Attributes
:navtitle: Unset Attributes

Document attributes--built-in, boolean, and custom--can be unset in the document header and document body.

"#
);

mod unset_in_header {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        document::InterpretedValue,
        parser::ModificationContext,
        tests::sdd::{non_normative, verifies},
    };

    non_normative!(
        r#"
== Unset a document attribute in the header

"#
    );

    #[test]
    fn unset_syntax() {
        verifies!(
            r#"
Document attributes are unset by adding a bang symbol (`!`) directly in front of (preferred) or after the attribute's name.
Like when setting an attribute in a document header, the attribute entry must be on its own line.
Don't add a value to the entry.

[source]
----
= Title
:!name: <.>
:name!: <.>
----
<.> An attribute is unset when a `!` is prefixed to its name (preferred).
<.> An attribute is unset when a `!` is appended to its name.

"#
        );

        let mut parser = Parser::default()
            .with_intrinsic_attribute("name1", "name1", ModificationContext::Anywhere)
            .with_intrinsic_attribute("name2", "name2", ModificationContext::Anywhere)
            .with_intrinsic_attribute("name3", "name3", ModificationContext::Anywhere);

        parser.parse("= Title\n:!name1:\n:name2!:");

        assert_eq!(parser.attribute_value("name1"), InterpretedValue::Unset);

        assert_eq!(parser.attribute_value("name2"), InterpretedValue::Unset);

        assert_eq!(
            parser.attribute_value("name3"),
            InterpretedValue::Value("name3".to_owned())
        );
    }
}
