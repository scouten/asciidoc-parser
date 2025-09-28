use crate::tests::prelude::*;

track_file!("docs/modules/document/pages/header.adoc");

non_normative!(
    r#"
= Document Header

An AsciiDoc document may begin with a document header.
The document header encapsulates the document title, author and revision information, document-wide attributes, and other document metadata.

"#
);

mod structure {
    // use pretty_assertions_sorted::assert_eq;

    use crate::{Parser, tests::prelude::*};

    non_normative!(
        r#"
== Document header structure

"#
    );

    #[test]
    fn skip_blank_or_comment_lines() {
        verifies!(
            r#"
The optional document header is a series of contiguous lines at the start of the AsciiDoc source, after skipping any empty or comment lines.
If a document has a header, _no content blocks are permitted above it_.
In other words, the document must start with a document header if it has one.

"#
        );

        let doc = Parser::default().parse("\n// comment\n= Title");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: Some(Span {
                        data: "Title",
                        line: 3,
                        col: 3,
                        offset: 14,
                    },),
                    title: Some("Title",),
                    attributes: &[],
                    author_line: None,
                    revision_line: None,
                    comments: &[Span {
                        data: "// comment",
                        line: 2,
                        col: 1,
                        offset: 1,
                    },],
                    source: Span {
                        data: "= Title",
                        line: 3,
                        col: 1,
                        offset: 12,
                    },
                },
                blocks: &[],
                source: Span {
                    data: "\n// comment\n= Title",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    #[ignore]
    fn author_revision_lines() {
        // TO DO (#370): Support for author data model.

        // TO DO (#371): Support for revision data model.
        to_do_verifies!(
            r#"
A header typically begins with a xref:title.adoc[].
When a document title is specified, it may be immediately followed by one or two designated lines of content.
These implicit content lines are used to assign xref:author-information.adoc[] and xref:revision-information.adoc[] to the document.

"#
        );

        todo!();
    }
}
