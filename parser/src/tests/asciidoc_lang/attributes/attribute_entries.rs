use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser, Span,
    document::{Attribute, InterpretedValue},
    tests::{
        fixtures::{
            TSpan,
            blocks::{TBlock, TSimpleBlock},
            content::TContent,
            document::{TAttribute, TDocument, THeader, TInterpretedValue},
        },
        sdd::{non_normative, track_file, verifies},
    },
};

track_file!("docs/modules/attributes/pages/attribute-entries.adoc");

non_normative!(
    r#"
= Attribute Entries

== What is an attribute entry?

Before you can use a document attribute in your document, you have to declare it.
An [.term]*attribute entry* is the primary mechanism for defining a document attribute in an AsciiDoc document.
You can think of an attribute entry as a global variable assignment for AsciiDoc.
The document attribute it creates becomes available from that point forward in the document.
Attribute entries are also frequently used to toggle features.

"#
);

#[test]
fn set_boolean_attribute() {
    verifies!(
        r#"
An attribute entry consists of two parts: an attribute *name* and an attribute *value*.
The attribute name comes first, followed by the optional value.
Each attribute entry must be entered on its own line.
An attribute entry starts with an opening colon (`:`), directly followed by the attribute's name, and then a closing colon (`:`).
This [.term]*sets* -- that is, turns on -- the document attribute so you can use it in your document.

[source]
----
:name-of-an-attribute: <.>
----
<.> The attribute's name is directly preceded with a opening colon (`:`) and directly followed by a closing colon (`:`).

"#
    );

    let mi = Attribute::parse(Span::new(":name-of-an-attribute:"), &Parser::default()).unwrap();

    assert_eq!(
        mi.item,
        TAttribute {
            name: TSpan {
                data: "name-of-an-attribute",
                line: 1,
                col: 2,
                offset: 1,
            },
            value_source: None,
            value: TInterpretedValue::Set,
            source: TSpan {
                data: ":name-of-an-attribute:",
                line: 1,
                col: 1,
                offset: 0,
            },
        }
    );

    assert_eq!(mi.item.value(), &InterpretedValue::Set);
}

#[test]
fn explicit_value() {
    verifies!(
        r#"
In many cases, you explicitly assign a value to a document attribute by entering information after its name in the attribute entry.
The value must be offset from the closing colon (`:`) by at least one space.

[source]
----
:name-of-an-attribute: value of the attribute <.>
----
<.> An explicitly assigned value is offset from the closing colon (`:`) by at least one space.
At the end of the value, press kbd:[Enter].

"#
    );

    let mut parser = Parser::default();

    let doc = parser.parse("= Testing\n:name-of-an-attribute: value of the attribute\n\nThe value of the attribute named `name-of-an-attribute` is: {name-of-an-attribute}");

    assert_eq!(
        doc,
        TDocument {
            header: THeader {
                title: Some(TSpan {
                    data: "Testing",
                    line: 1,
                    col: 3,
                    offset: 2,
                },),
                attributes: &[TAttribute {
                    name: TSpan {
                        data: "name-of-an-attribute",
                        line: 2,
                        col: 2,
                        offset: 11,
                    },
                    value_source: Some(TSpan {
                        data: "value of the attribute",
                        line: 2,
                        col: 24,
                        offset: 33,
                    }),
                    value: TInterpretedValue::Value("value of the attribute"),
                    source: TSpan {
                        data: ":name-of-an-attribute: value of the attribute",
                        line: 2,
                        col: 1,
                        offset: 10,
                    },
                },],
                source: TSpan {
                    data: "= Testing\n:name-of-an-attribute: value of the attribute",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "The value of the attribute named `name-of-an-attribute` is: {name-of-an-attribute}",
                        line: 4,
                        col: 1,
                        offset: 57,
                    },
                    rendered: "The value of the attribute named <code>name-of-an-attribute</code> is: value of the attribute",
                },
                source: TSpan {
                    data: "The value of the attribute named `name-of-an-attribute` is: {name-of-an-attribute}",
                    line: 4,
                    col: 1,
                    offset: 57,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },),],
            source: TSpan {
                data: "= Testing\n:name-of-an-attribute: value of the attribute\n\nThe value of the attribute named `name-of-an-attribute` is: {name-of-an-attribute}",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );

    assert_eq!(
        parser.attribute_value("name-of-an-attribute"),
        TInterpretedValue::Value("value of the attribute")
    );
}

#[test]
fn header_substitutions_applied() {
    verifies!(
        r#"
Take note that xref:attribute-entry-substitutions.adoc[header substitutions] automatically get applied to the value by default.
That means you don't need to escape special characters such in an HTML tag.
"#
    );

    let mut parser = Parser::default();

    let doc = parser.parse("= Testing\n:lt-attribute: <\n\nThe value of the attribute named `lt-attribute` is: {lt-attribute}");

    assert_eq!(
        doc,
        TDocument {
            header: THeader {
                title: Some(TSpan {
                    data: "Testing",
                    line: 1,
                    col: 3,
                    offset: 2,
                },),
                attributes: &[TAttribute {
                    name: TSpan {
                        data: "lt-attribute",
                        line: 2,
                        col: 2,
                        offset: 11,
                    },
                    value_source: Some(TSpan {
                        data: "<",
                        line: 2,
                        col: 16,
                        offset: 25,
                    },),
                    value: TInterpretedValue::Value("&lt;"),
                    source: TSpan {
                        data: ":lt-attribute: <",
                        line: 2,
                        col: 1,
                        offset: 10,
                    },
                },],
                source: TSpan {
                    data: "= Testing\n:lt-attribute: <",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "The value of the attribute named `lt-attribute` is: {lt-attribute}",
                        line: 4,
                        col: 1,
                        offset: 28,
                    },
                    rendered: "The value of the attribute named <code>lt-attribute</code> is: &lt;",
                },
                source: TSpan {
                    data: "The value of the attribute named `lt-attribute` is: {lt-attribute}",
                    line: 4,
                    col: 1,
                    offset: 28,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },),],
            source: TSpan {
                data: "= Testing\n:lt-attribute: <\n\nThe value of the attribute named `lt-attribute` is: {lt-attribute}",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn reference_existing_attributes() {
    verifies!(
        r#"
It also means you can reference the value of attributes which have already been defined when defining the value of an attribute.
Attribute references in the value of an attribute entry are resolved immediately.

[source]
----
:url-org: https://example.org/projects
:url-project: {url-org}/project-name <.>
----
<.> You can reuse the value of an attribute which has already been set using using an attribute reference in the value.

"#
    );

    let mut parser = Parser::default();

    let doc = parser
        .parse(":url-org: https://example.org/projects\n:url-project: {url-org}/project-name");

    assert_eq!(
        doc,
        TDocument {
            header: THeader {
                title: None,
                attributes: &[
                    TAttribute {
                        name: TSpan {
                            data: "url-org",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                        value_source: Some(TSpan {
                            data: "https://example.org/projects",
                            line: 1,
                            col: 11,
                            offset: 10,
                        },),
                        value: TInterpretedValue::Value("https://example.org/projects",),
                        source: TSpan {
                            data: ":url-org: https://example.org/projects",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                    },
                    TAttribute {
                        name: TSpan {
                            data: "url-project",
                            line: 2,
                            col: 2,
                            offset: 40,
                        },
                        value_source: Some(TSpan {
                            data: "{url-org}/project-name",
                            line: 2,
                            col: 15,
                            offset: 53,
                        },),
                        value: TInterpretedValue::Value(
                            "https://example.org/projects/project-name",
                        ),
                        source: TSpan {
                            data: ":url-project: {url-org}/project-name",
                            line: 2,
                            col: 1,
                            offset: 39,
                        },
                    },
                ],
                source: TSpan {
                    data: ":url-org: https://example.org/projects\n:url-project: {url-org}/project-name",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
            },
            blocks: &[],
            source: TSpan {
                data: ":url-org: https://example.org/projects\n:url-project: {url-org}/project-name",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );

    assert_eq!(
        parser.attribute_value("url-project"),
        TInterpretedValue::Value("https://example.org/projects/project-name")
    );
}
