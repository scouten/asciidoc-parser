use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/ROOT/pages/key-concepts.adoc");
// Tracking commit 1767ee1e, current as of 2024-10-26.

non_normative!(
    r#"
= Key Concepts

This page introduces you to some of the concepts and terms you'll encounter as you learn about AsciiDoc.
Each concept will be covered in more depth later in the documentation.
Use this page as a way to start to familiarize yourself with the lingo.

== Document

A document represents the top-level block element in AsciiDoc.
It consists of an optional document header and either a) one or more sections preceded by an optional preamble or b) a sequence of top-level blocks only.

The document can be configured using a document header.
The header is not a block itself, but contributes metadata to the document, such as the document title and document attributes.

== Elements

An element is an identifiable, addressable, and composable chunk of content in a document.
An AsciiDoc document is merely a composition of all the elements it contains.

Elements are a hierarchy of types, where one element may be a specialization of a family of elements.
For example, a sidebar block is a block element, so it shares the traits of all block elements, and also adds some of its own.

Elements include the document itself, sections, blocks, block macros, breaks, and inline phrases and macros.

A [.term]*block element* is stacked vertically (by line) above or below other block elements.
Block elements are typically referred to simply as [.term]*blocks*.
Blocks form the main tree structure of the document.

An [.term]*inline element* is a span of content within a block element or one of its attributes (e.g., a block title).
Inline elements include formatted text (italic, bold, etc), inline macros, and element references.
What fills in the gap between these elements is unsubstituted text.
Inline elements are less structured than block elements as they are more geared towards substitutions than a tree structure.

== Attributes

An attribute is a name/value pair used for storing and disclosing metadata in the AsciiDoc language.
Attributes can be used to influence the syntax, control behavior, customize styles, activate or configure integrations, or store inline replacement content.
Attributes truly set AsciiDoc apart from other lightweight markup languages.

An attribute is actually an abstract term.
There are two concrete classifications of attributes: document attributes and element attributes.

=== Document attributes

Document attributes, as the name implies, are associated directly with the document.
They are used to export information about the document at runtime, control behavior of the processor, and to store reusable values or phrases.
Thus, they are a sort of two-way communication channel with the processor.

Document attributes can be referenced in the content using an attribute reference (wherever the attribute substitution is enabled).
A document attribute can be defined either in the document using an attribute entry (typically in the document header) or from the API or CLI.
Not all document attributes can be modified.

=== Element attributes

Element attributes are metadata on a specific element, like a block or an inline element.
They are defined in an attribute list and only apply to that element.
The placement of the attribute list depends on the element.
The attribute name can either be a string (i.e., a named attribute) or an implicit numerical index (i.e., an unnamed, positional attribute).

//Element attributes are not accessible at all from the content, so they cannot be referenced like document attributes.
Unlike document attributes, element attributes cannot be referenced directly from the content, on the document model.
In other words, they cannot be resolved using an attribute reference.
Element attributes enrich or configure the behavior of an element, such as to apply a role or set the width of an image.
An element attribute is defined using an attribute list on an element, or an available shorthand like a block title line.

"#
);

mod macros {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::MediaType,
        tests::{
            fixtures::{
                TSpan,
                attributes::{TAttrlist, TElementAttribute},
                blocks::{TBlock, TMediaBlock, TSimpleBlock},
                content::TContent,
                document::{TDocument, THeader},
            },
            sdd::{non_normative, verifies},
        },
    };

    non_normative!(
        r#"
== Macros

As you read through this documentation, you'll frequently see references to the term macro.
A macro is a syntax for representing non-text elements or syntax that expands into text using the provided metadata.
See https://en.wikipedia.org/wiki/Macro_(computer_science)[macro^] to learn more about the meaning of this term.

"#
    );

    #[test]
    fn block_macro() {
        verifies!(
            r#"
Here's an example of a block macro:

[source]
----
image::sunset.jpg[Sunset]
----

"#
        );

        assert_eq!(
            Parser::default().parse("image::sunset.jpg[Sunset]\n"),
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0
                    }
                },
                blocks: &[TBlock::Media(TMediaBlock {
                    type_: MediaType::Image,
                    target: TSpan {
                        data: "sunset.jpg",
                        line: 1,
                        col: 8,
                        offset: 7,
                    },
                    macro_attrlist: TAttrlist {
                        attributes: &[TElementAttribute {
                            name: None,
                            shorthand_items: &["Sunset"],
                            value: "Sunset"
                        }],
                        source: TSpan {
                            data: "Sunset",
                            line: 1,
                            col: 19,
                            offset: 18,
                        }
                    },
                    source: TSpan {
                        data: "image::sunset.jpg[Sunset]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                })],
                source: TSpan {
                    data: "image::sunset.jpg[Sunset]",
                    line: 1,
                    col: 1,
                    offset: 0
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn inline_macro() {
        verifies!(
            r#"
Here's an example of an inline macro:

[source]
----
Click the button with the image:star.png[Star] to favorite the project.
----

"#
        );

        assert_eq!(
            Parser::default()
                .parse("Click the button with the image:star.png[Star] to favorite the project.\n"),
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0
                    }
                },
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "Click the button with the image:star.png[Star] to favorite the project.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"Click the button with the <span class="image"><img src="star.png" alt="Star"></span> to favorite the project."#,
                    },
                    source: TSpan {
                        data: "Click the button with the image:star.png[Star] to favorite the project.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                })],
                source: TSpan {
                    data: "Click the button with the image:star.png[Star] to favorite the project.",
                    line: 1,
                    col: 1,
                    offset: 0
                },
                warnings: &[],
            }
        );

        non_normative!(
            r#"
You can think of a macro like a function.
A syntax of macro follows the form of a name, a target which is sometimes optional, and an attribute list consisting of zero or more element attributes enclosed in square brackets.

There are two variations of a macro: block and inline.
In a block macro, the name and target are separated by two colons (`::`) and it must reside on a line by itself.
In an inline macro, the name and target are separated by a single colon (`:`) and it can be alongside text and other inline elements.
A block macro is always parsed, whereas an inline macro is only parsed where the macros substitution is enabled.

"#
        );
    }
}

// NOT COVERED YET:
// == Preprocessor directives

// There's another syntax in AsciiDoc that looks a lot like block macros, only
// they aren't. These are the preprocessor directives.

// A preprocessor directive is a function that controls lines that are fed into
// the parser. A conditional preprocessor directive can configure lines to be
// included or excluded based on the presence of an attribute (ifdef, ifndef) or
// another arbitrary condition (ifeval). An include directive can add additional
// lines to the document taken from another document.

// Preprocessor directives share common traits with a block macro.
// Like a block macro, a preprocessor directive must be on a line by itself.
// While the preprocessor directive can access document attributes, it's not
// otherwise aware of the context around it. It's only a line processor.
// Like a block macro, the include directive can have element attributes, though
// they only apply to the preprocessing operation itself.
