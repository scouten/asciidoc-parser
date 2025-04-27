use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/ROOT/pages/document-structure.adoc");
// Tracking commit 9765f83b, current as of 2024-10-26.

non_normative!(
    r#"
= Document Structure
:page-aliases: document.adoc

On this page, you'll learn about the overall structure of an AsciiDoc document.
Don't worry about the details of the syntax at this point.
That topic will be covered thoroughly later in the documentation.
Right now, we're just aiming to get a sense of what makes up an AsciiDoc document.

"#
);

mod documents {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        tests::{
            fixtures::{
                blocks::{TBlock, TSimpleBlock},
                document::{TAttribute, TDocument, THeader, TRawAttributeValue},
                TContent, TSpan,
            },
            sdd::{non_normative, verifies},
        },
        Parser,
    };

    non_normative!(
        r#"
== Documents

AsciiDoc is a plain text writing format with no boilerplate enclosure or prologue.
An AsciiDoc document may consist of only a single sentence (or even a single character, to be academic).

"#
    );

    #[test]
    fn single_sentence() {
        verifies!(
            r#"
The following example is a valid AsciiDoc document with a single paragraph containing a single sentence:

----
This is a basic AsciiDoc document.
----

"#
        );

        assert_eq!(
            Parser::default().parse("This is a basic AsciiDoc document.\n"),
            TDocument {
                header: THeader {
                    title: None,
                    attributes: vec![],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0
                    }
                },
                source: TSpan {
                    data: "This is a basic AsciiDoc document.",
                    line: 1,
                    col: 1,
                    offset: 0
                },
                blocks: vec![TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "This is a basic AsciiDoc document.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: None
                    },
                    source: TSpan {
                        data: "This is a basic AsciiDoc document.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                })],
                warnings: vec![],
            }
        );
    }

    #[test]
    fn multiple_paragraphs() {
        verifies!(
            r#"
Of course, you can have more content than a single sentence.
What we want to emphasize here is that it's simple to get started.

An AsciiDoc document is a series of blocks stacked on top of one another (by line).
These blocks are typically offset from one another by empty lines (though these may be optional in certain circumstances).

To expand the previous document from one paragraph to two, you'd separate the two paragraphs by an empty line:

----
This is a basic AsciiDoc document.

This document contains two paragraphs.
----

"#
        );

        assert_eq!(
            Parser::default().parse(
                "This is a basic AsciiDoc document.\n\nThis document contains two paragraphs.\n"
            ),
            TDocument {
                header: THeader {
                    title: None,
                    attributes: vec![],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0
                    }
                },
                source: TSpan {
                    data: "This is a basic AsciiDoc document.\n\nThis document contains two paragraphs.",
                    line: 1,
                    col: 1,
                    offset: 0
                },
                blocks: vec![
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "This is a basic AsciiDoc document.",
                                line: 1,
                                col: 1,
                                offset: 0,
                            },
                            rendered: None,
                        },
                        source: TSpan {
                            data: "This is a basic AsciiDoc document.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    }),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "This document contains two paragraphs.",
                                line: 3,
                                col: 1,
                                offset: 36,
                            },
                            rendered: None,
                        },
                        source: TSpan {
                            data: "This document contains two paragraphs.",
                            line: 3,
                            col: 1,
                            offset: 36,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    })],
                warnings: vec![],
            }
        );
    }

    #[test]
    fn header() {
        verifies!(
            r#"
An AsciiDoc document may begin with a document header.
Although the document header is optional, it's often used because it allows you to specify the document title and to set document-wide configuration and reusable text in the form of document attributes.

[source]
----
= Document Title
:reproducible:

This is a basic AsciiDoc document by {author}.

This document contains two paragraphs.
It also has a header that specifies the document title.
----

"#
        );

        assert_eq!(
            Parser::default().parse(
                "= Document Title\n:reproducible:\n\nThis is a basic AsciiDoc document by {author}.\n\nThis document contains two paragraphs.\nIt also has a header that specifies the document title."
            ),
            TDocument {
                header: THeader {
                    title: Some(
                        TSpan {
                            data: "Document Title",
                            line: 1,
                            col: 3,
                            offset: 2,
                        },
                    ),
                    attributes: vec![
                        TAttribute {
                            name: TSpan {
                                data: "reproducible",
                                line: 2,
                                col: 2,
                                offset: 18,
                            },
                            value: TRawAttributeValue::Set,
                            source: TSpan {
                                data: ":reproducible:",
                                line: 2,
                                col: 1,
                                offset: 17,
                            },
                        },
                    ],
                    source: TSpan {
                        data: "= Document Title\n:reproducible:",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: vec![
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "This is a basic AsciiDoc document by {author}.",
                                line: 4,
                                col: 1,
                                offset: 33,
                            },
                            rendered: None,
                        },
                        source: TSpan {
                            data: "This is a basic AsciiDoc document by {author}.",
                            line: 4,
                            col: 1,
                            offset: 33,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    }),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "This document contains two paragraphs.\nIt also has a header that specifies the document title.",
                                line: 6,
                                col: 1,
                                offset: 81,
                            },
                            rendered: None,
                        },
                        source: TSpan {
                            data: "This document contains two paragraphs.\nIt also has a header that specifies the document title.",
                            line: 6,
                            col: 1,
                            offset: 81,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    })
                ],
                source: TSpan {
                    data: "= Document Title\n:reproducible:\n\nThis is a basic AsciiDoc document by {author}.\n\nThis document contains two paragraphs.\nIt also has a header that specifies the document title.",
                    line: 1,
                    col: 1,
                    offset: 0
                },
                warnings: vec![],
            }
        );
    }

    non_normative!(
        r#"
Almost any combination of blocks constitutes a valid AsciiDoc document (with some structural requirements dictated by the xref:document:doctypes.adoc[document type]).
Documents can range from a single sentence to a multi-part book.

"#
    );
}

mod lines {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        document::Attribute,
        tests::{
            fixtures::{
                document::{TAttribute, TInterpretedValue, TRawAttributeValue},
                TSpan,
            },
            sdd::verifies,
        },
        Span,
    };

    #[test]
    fn section_title() {
        verifies!(
            r#"
== Lines

The line is a significant construct in AsciiDoc.
A line is defined as text that's separated on either side by either a newline character or the boundary of the document.
Many aspects of the syntax must occupy a whole line.
That's why we say AsciiDoc is a line-oriented language.

For example, a section title must be on a line by itself.
The same is true for an attribute entry, a block title, a block attribute list, a block macro, a list item, a block delimiter, and so forth.

.Example of a section title, which must occupy a single line
[source]
----
== Section Title
----

"#
        );

        let span = Span::new("== Section Title\n");
        let l = span.take_line();

        assert_eq!(
            l.after,
            TSpan {
                data: "",
                line: 2,
                col: 1,
                offset: 17
            }
        );

        assert_eq!(
            l.item,
            TSpan {
                data: "== Section Title",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn one_line_attribute() {
        verifies!(
            r#"
.Example of an attribute entry, which must also occupy at least one line
[source]
-----
:name: value
-----

"#
        );

        let mi = Attribute::parse(Span::new(":name: value\n")).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "name",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value: TRawAttributeValue::Value(TSpan {
                    data: "value",
                    line: 1,
                    col: 8,
                    offset: 7,
                }),
                source: TSpan {
                    data: ":name: value",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 2,
                col: 1,
                offset: 13
            }
        );
    }

    #[test]
    fn two_line_attribute() {
        verifies!(
            r#"
.Example of an attribute entry that extends to two lines
[source]
-----
:name: value \
more value
-----

"#
        );

        let mi = Attribute::parse(Span::new(":name: value \\\nmore value\n")).unwrap();

        assert_eq!(
            mi.item,
            TAttribute {
                name: TSpan {
                    data: "name",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                value: TRawAttributeValue::Value(TSpan {
                    data: "value \\\nmore value",
                    line: 1,
                    col: 8,
                    offset: 7,
                }),
                source: TSpan {
                    data: ":name: value \\\nmore value",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            }
        );

        assert_eq!(
            mi.item.value(),
            TInterpretedValue::Value("value more value")
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 3,
                col: 1,
                offset: 26
            }
        );
    }

    // No test cases:

    // Empty lines can also be significant.
    // A single empty line separates the header from the body.
    // Many blocks are also separated by an empty line, as you saw in the two
    // paragraph example earlier.

    // In contrast, lines within paragraph content are insignificant.
    // Keep these points in mind as you're learning about the AsciiDoc syntax.
}

mod blocks {
    // No test cases:

    // == Blocks

    // Blocks in an AsciiDoc document lay down the document structure.
    // Some blocks may contain other blocks, so the document structure is
    // inherently hierarchical (i.e., a tree structure). You can preview
    // this section structure, for example, by enabling the automatic table
    // of contents. Examples of blocks include paragraphs, sections, lists,
    // delimited blocks, tables, and block macros.

    // Blocks are easy to identify because they're usually offset from other
    // blocks by an empty line (though not always required). Blocks always
    // start on a new line, terminate at the end of a line, and are aligned
    // to the left margin.

    // Every block can have one or more lines of block metadata.
    // This metadata can be in the form of block attributes, a block anchor, or
    // a block title. These metadata lines must be above and directly
    // adjacent to the block itself.

    // Sections, non-verbatim delimited blocks, and AsciiDoc table cells may
    // contain other blocks. Despite the fact that blocks form a hierarchy,
    // even nested blocks start at the left margin. By requiring blocks to
    // start at the left margin, it avoids the tedium of having to track and
    // maintain levels of indentation and makes the content more reusable.
}

mod text_and_inline_elements {
    // No test cases:

    // == Text and inline elements

    // Surrounded by the markers, delimiters, and metadata lines is the text.
    // The text is the main focus of a document and the reason the AsciiDoc
    // syntax gives it so much room to breathe. Text is most often found in
    // the lines of a block (e.g., paragraph), the block title (e.g.,
    // section title), and in list items, though there are other places
    // where it can exist.

    // Text is subject to substitutions.
    // Substitutions interpret markup as text formatting, replace macros with
    // text or non-text elements, expand attribute references, and perform
    // other sorts of text replacement.

    // Normal text is subject to all substitutions, unless specified otherwise.
    // Verbatim text is subject to a minimal set of substitutions to allow it to
    // be displayed in the output as it appears in the source. It's also
    // possible to disable all substitutions in order to pass the text
    // through to the output unmodified (i.e., raw). The parsing of text
    // ends up being a mix of inline elements and other forms of
    // transformations.
}

mod encodings_and_asciidoc_files {
    // == Encodings and AsciiDoc files

    // An AsciiDoc file is a text file that has the _.adoc_ file extension
    // (e.g., [.path]_document.adoc_). Most AsciiDoc processors assume the
    // text in the file uses UTF-8 encoding. .[line-through]#UTF-16
    // encodings are supported only if the file starts with a BOM.#
    // *UNSUPPORTED: The UTF-16 encoding is not directly supported by the
    // `asciidoc-parser` crate.*

    // An AsciiDoc processor can process AsciiDoc from a string (i.e., character
    // sequence). However, most of the time you'll save your AsciiDoc documents
    // to a file.
}
