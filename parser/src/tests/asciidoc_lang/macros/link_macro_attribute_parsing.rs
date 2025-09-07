use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/link-macro-attribute-parsing.adoc");

non_normative!(
    r#"
= Link & URL Macro Attribute Parsing

If named attributes are detected between the square brackets of a link or URL macro, that text is parsed as an attribute list.
This page explains the conditions when this occurs and how to write the link text so it is recognized as a single positional attribute.

"#
);

mod link_text_alongside_named_attributes {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        tests::{
            fixtures::{
                TSpan,
                blocks::{TBlock, TSimpleBlock},
                content::TContent,
                document::{TDocument, THeader},
            },
            sdd::{non_normative, verifies},
        },
    };

    non_normative!(
        r#"
== Link text alongside named attributes

"#
    );

    #[test]
    fn whole_text() {
        verifies!(
            r#"
Normally, the whole text between the square brackets of a link macro is treated as the link text (i.e., the first positional attribute).

[source]
----
https://chat.asciidoc.org[Discuss AsciiDoc]
----

"#
        );

        let doc = Parser::default().parse("https://chat.asciidoc.org[Discuss AsciiDoc]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "https://chat.asciidoc.org[Discuss AsciiDoc]",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<a href=\"https://chat.asciidoc.org\">Discuss AsciiDoc</a>",
                    },
                    source: TSpan {
                        data: "https://chat.asciidoc.org[Discuss AsciiDoc]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "https://chat.asciidoc.org[Discuss AsciiDoc]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn attrlist_with_role_and_target() {
        verifies!(
            r#"
However, if the text contains an equals sign (`=`), the text is parsed as an xref:attributes:element-attributes.adoc#attribute-list[attribute list].
The exact rules for attribute list parsing and positional attributes are rather complex, and discussed on xref:attributes:positional-and-named-attributes.adoc[].
To be sure the link text is recognized properly, you can apply these two simple checks:

* contains no comma (`,`) or equals sign (`=`) or
* enclosed in double quotes (`"`)

There are several other situations in which text before the first comma may be recognized as the link text.
Let's consider some examples.

The following example shows a URL macro with custom link text alongside named attributes.

[source]
----
https://chat.asciidoc.org[Discuss AsciiDoc,role=resource,window=_blank]
----

"#
        );

        let doc = Parser::default().parse(
            "https://chat.asciidoc.org[Discuss AsciiDoc,role=resource,window=_blank]
",
        );

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "https://chat.asciidoc.org[Discuss AsciiDoc,role=resource,window=_blank]",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<a href=\"https://chat.asciidoc.org\" class=\"resource\" target=\"_blank\" rel=\"noopener\">Discuss AsciiDoc</a>",
                    },
                    source: TSpan {
                        data: "https://chat.asciidoc.org[Discuss AsciiDoc,role=resource,window=_blank]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "https://chat.asciidoc.org[Discuss AsciiDoc,role=resource,window=_blank]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn attrlist_with_quoted_title() {
        verifies!(
            r#"
Let's consider a case where the link text contains a comma and the macro also has named attributes.
In this case, you must enclose the link text in double quotes so that it is capture in its entirety as the first positional attribute.

[source]
----
https://example.org["Google, DuckDuckGo, Ecosia",role=teal]
----

"#
        );

        let doc = Parser::default()
            .parse("https://example.org[\"Google, DuckDuckGo, Ecosia\",role=teal]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "https://example.org[\"Google, DuckDuckGo, Ecosia\",role=teal]",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<a href=\"https://example.org\" class=\"teal\">Google, DuckDuckGo, Ecosia</a>",
                    },
                    source: TSpan {
                        data: "https://example.org[\"Google, DuckDuckGo, Ecosia\",role=teal]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "https://example.org[\"Google, DuckDuckGo, Ecosia\",role=teal]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn quoted_title_only() {
        verifies!(
            r#"
Similarly, if the link text contains an equals sign, you can enclose the link text in double quotes to ensure the parser recognizes it as the first positional attribute.

[source]
----
https://example.org["1=2 posits the problem of inequality"]
----

"#
        );

        let doc = Parser::default()
            .parse("https://example.org[\"1=2 posits the problem of inequality\"]");

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "https://example.org[\"1=2 posits the problem of inequality\"]",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<a href=\"https://example.org\">1=2 posits the problem of inequality</a>",
                    },
                    source: TSpan {
                        data: "https://example.org[\"1=2 posits the problem of inequality\"]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "https://example.org[\"1=2 posits the problem of inequality\"]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn quoted_title_with_escaped_quote() {
        verifies!(
            r##"
If the quoted link text itself contains the quote character used to enclose the text, escape the quote character in the text by prefixing it with a backslash.

[source]
----
https://example.org["href=\"#top\" attribute"] creates link to top of page
----

"##
        );

        let doc = Parser::default().parse(
            "https://example.org[\"href=\\\"#top\\\" attribute\"] creates link to top of page",
        );

        assert_eq!(
            doc,
            TDocument {
                header: THeader {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: TSpan {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "https://example.org[\"href=\\\"#top\\\" attribute\"] creates link to top of page",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "<a href=\"https://example.org\">href=\"#top\" attribute</a> creates link to top of page",
                    },
                    source: TSpan {
                        data: "https://example.org[\"href=\\\"#top\\\" attribute\"] creates link to top of page",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "https://example.org[\"href=\\\"#top\\\" attribute\"] creates link to top of page",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}
