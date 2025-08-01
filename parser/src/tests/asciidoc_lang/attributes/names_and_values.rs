use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/attributes/pages/names-and-values.adoc");
// Tracking commit 636ceedc, current as of 2025-04-12.

non_normative!(
    r#"
= Attribute Entry Names and Values

== Valid built-in names

Built-in attribute names are reserved and can't be re-purposed for user-defined attribute names.
The built-in attribute names are listed in the xref:document-attributes-ref.adoc[] and xref:character-replacement-ref.adoc[].

"#
);

mod valid_user_defined_names {
    use crate::{
        Parser, Span,
        document::{Attribute, InterpretedValue},
        tests::{
            fixtures::{
                TSpan,
                document::{TAttribute, TInterpretedValue},
            },
            sdd::verifies,
        },
    };

    verifies!(
        r#"
[#user-defined]
== Valid user-defined names

User-defined attribute names must:

* be at least one character long,
* begin with a word character (a-z, 0-9, or _), and
* only contain word characters and hyphens (-).

A user-defined attribute name cannot contain dots (.) or spaces.
Although uppercase characters are permitted in an attribute name, the name is converted to lowercase before being stored.
For example, `URL-REPO` and `URL-Repo` are treated as `url-repo` when a document is loaded or converted.
A best practice is to only use lowercase letters in the name and avoid starting the name with a number.

"#
    );

    #[test]
    fn at_least_one_character_long() {
        assert!(Attribute::parse(Span::new("::"), &Parser::default()).is_none());

        let mi = Attribute::parse(Span::new(":a:"), &Parser::default()).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "a",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value_source: None,
                value: TInterpretedValue::Set,
                source: TSpan {
                    data: ":a:",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(mi.item.value(), &InterpretedValue::Set);
    }

    #[test]
    fn begin_with_word_character() {
        assert!(Attribute::parse(Span::new(":-abc:"), &Parser::default()).is_none());

        let mi = Attribute::parse(Span::new(":9abc:"), &Parser::default()).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "9abc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value_source: None,
                value: TInterpretedValue::Set,
                source: TSpan {
                    data: ":9abc:",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(mi.item.value(), &InterpretedValue::Set);

        let mi = Attribute::parse(Span::new(":_abc:"), &Parser::default()).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "_abc",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value_source: None,
                value: TInterpretedValue::Set,
                source: TSpan {
                    data: ":_abc:",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(mi.item.value(), &InterpretedValue::Set);
    }

    #[test]
    fn only_contain_word_characters_and_hyphens() {
        assert!(Attribute::parse(Span::new(":abc def:"), &Parser::default()).is_none());
        assert!(Attribute::parse(Span::new(":abc.def:"), &Parser::default()).is_none());

        let mi = Attribute::parse(Span::new(":9ab-cdef:"), &Parser::default()).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "9ab-cdef",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value_source: None,
                value: TInterpretedValue::Set,
                source: TSpan {
                    data: ":9ab-cdef:",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(mi.item.value(), &InterpretedValue::Set);
    }

    #[test]
    fn may_contain_uppercase() {
        // IMPORTANT: We've defined the lower-case normalization as out of scope for
        // the parser crate for now.
        let mi = Attribute::parse(Span::new(":URL-REPO:"), &Parser::default()).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "URL-REPO",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value_source: None,
                value: TInterpretedValue::Set,
                source: TSpan {
                    data: ":URL-REPO:",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(mi.item.value(), &InterpretedValue::Set);

        let mi = Attribute::parse(Span::new(":URL-REPO:"), &Parser::default()).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "URL-REPO",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value_source: None,
                value: TInterpretedValue::Set,
                source: TSpan {
                    data: ":URL-REPO:",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            }
        );

        assert_eq!(mi.item.value(), &InterpretedValue::Set);
    }
}

// TO DO (https://github.com/scouten/asciidoc-parser/issues/219): Determine how or whether to handle default/built-in document attributes values in parser.
//
// For now, assuming that default and built-in values are not in scope for
// parser, but more likely part of an upstream renderer crate. For that reason,
// treating these sections as non-normative.
non_normative!(
    r#"
== Attribute value types and assignment methods

Depending on the attribute, its value may be an empty string, an integer such as 5 or 2, or a string of characters like your name or a URL.
Attributes that accept string values may include references to other attributes and inline macros.
Values can't contain complex, multi-line block elements such as tables or sidebars.

An attribute's value may be assigned by default when the value is left empty in an attribute entry or the value may be assigned explicitly by the user.
The type of value an attribute accepts and whether it uses a default value, has multiple built-in values, accepts a user-defined value, or requires a value to be explicitly assigned depends on the attribute.

=== Built-in values

Many built-in attributes have one or more built-in values.
One of these values may be designated as the attribute's default value.
The AsciiDoc processor will fall back to this default value if you set the attribute but leave the value empty.
Additionally, the processor automatically sets numerous built-in attributes at processing time and assigns them their default values unless you explicitly unset the attribute or assign it another value.
For instance, the processor automatically sets all of the xref:character-replacement-ref.adoc[character replacement attributes].

If you want to use the non-default value of a built-in attribute, you need to set it and assign it an alternative value.

=== Empty string values

The value for built-in boolean attributes is always left empty in an attribute entry since these attributes only turn on or turn off a feature.
During processing, the AsciiDoc processor assigns any activated boolean attributes an _empty string_ value.

=== Explicit values

You must explicitly assign a value to an attribute when:

* it doesn't have a default value,
* you want to override the default value, or
* it's a user-defined attribute.

The type of explicit value a built-in attribute accepts depends on the attribute.
User-defined attributes accept string values.
Long explicit values can be xref:wrap-values.adoc[wrapped].

////
For example,

[source]
----
:keywords: content engineering, branch collisions, 42, {meta-topics}, FTW <1> <2>
----
<1> The xref:header:metadata.adoc#keywords[built-in keywords attribute] doesn't have a default value, so you must explicitly assign it a value when you set it.
<2> Attributes that accept string values may include <<attribute-reference,references to other attributes>>, e.g, `+{meta-topics}+`.
See the xref:document-attributes-ref.adoc[Document Attributes Reference] for information about each built-in attribute's accepted value types.

You must explicitly assign a value to a built-in attribute when you want to override its default value.
For instance, when a section in a document is assigned the appendix style, that section title will be automatically prefixed with a label and a letter that signifies that section's order, e.g., Appendix A, by default.
Let's override the default letter ordering and use a number instead.

[source]
----
:appendix-number: 1
----

Now the first section assigned the appendix style will be prefixed Appendix 1, the second, Appendix 2, and so forth.

=== User-defined values

The value field of a built-in attribute is left empty if it's a boolean attribute.
The value can also be left empty if the attribute has an inferred default value and that's the value you want to use.

When you're setting a built-in attribute, the value may be _empty string_ if it's a boolean attribute, a built-in value, or a user-defined value.
However, if the document attribute is built-in, the value may be _empty
Depending on the type of document attribute--built-in or user-defined--the value may be _empty string_,
Some attributes may not have a value explicitly assigned to them.
When a value is not specified, the value _empty string_ is assumed.
An empty value is often used to set a boolean attribute (thus making an empty value implicitly true).

* it's a built-in attribute doesn't accept any explicitly set values because it only turns on a behavior,
* it's a built-in attribute that uses a default value when its value is left empty, or
* the attribute was set, but not assigned a value by accident.
In this case, it will use its default value if applicable or output an error message when the document is processed.
////
"#
);
