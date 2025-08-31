use crate::tests::sdd::{non_normative, track_file};

track_file!("docs/modules/macros/pages/autolinks.adoc");

non_normative!(
    r#"
= Autolinks

The AsciiDoc processor will detect common URLs (unless escaped) wherever the macros substitution step is applied and automatically convert them into links.
This page documents the recognized URL schemes and how to disable this behavior on a case-by-case basis.

"#
);

mod url_schemes_for_autolinks {
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
== URL schemes for autolinks

AsciiDoc recognizes the following common URL schemes without the help of any markup:

[#schemes]
"#
    );

    #[test]
    fn http() {
        verifies!(
            r#"
* http
"#
        );

        let doc = Parser::default().parse("http://example.org");

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
                            data: "http://example.org",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"<a href="http://example.org" class="bare">http://example.org</a>"#,
                    },
                    source: TSpan {
                        data: "http://example.org",
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
                    data: "http://example.org",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn https() {
        verifies!(
            r#"
* https
"#
        );

        let doc = Parser::default().parse("https://example.org");

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
                            data: "https://example.org",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"<a href="https://example.org" class="bare">https://example.org</a>"#,
                    },
                    source: TSpan {
                        data: "https://example.org",
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
                    data: "https://example.org",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn ftp() {
        verifies!(
            r#"
* ftp
"#
        );

        let doc = Parser::default().parse("ftp://example.org");

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
                            data: "ftp://example.org",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"<a href="ftp://example.org" class="bare">ftp://example.org</a>"#,
                    },
                    source: TSpan {
                        data: "ftp://example.org",
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
                    data: "ftp://example.org",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn irc() {
        verifies!(
            r#"
* irc
"#
        );

        let doc = Parser::default().parse("irc://example.org");

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
                            data: "irc://example.org",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"<a href="irc://example.org" class="bare">irc://example.org</a>"#,
                    },
                    source: TSpan {
                        data: "irc://example.org",
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
                    data: "irc://example.org",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn mailto() {
        verifies!(
            r#"
* mailto

"#
        );

        let doc = Parser::default().parse("join@discuss.example.org");

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
                            data: "join@discuss.example.org",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"<a href="mailto:join@discuss.example.org">join@discuss.example.org</a>"#,
                    },
                    source: TSpan {
                        data: "join@discuss.example.org",
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
                    data: "join@discuss.example.org",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn https_example() {
        verifies!(
            r#"
The URL in the following example begins with a recognized protocol (i.e., https), so the AsciiDoc processor will automatically turn it into a hyperlink.

[source]
----
include::example$url.adoc[tag=base-co]
----
<.> The trailing period will not get caught up in the link.

The URL is also used as the link text.
If you want to use xref:url-macro.adoc#link-text[custom link text], you must use the xref:url-macro.adoc[URL macro].

"#
        );

        let doc = Parser::default()
            .parse("The homepage for the Asciidoctor Project is https://www.asciidoctor.org.");

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
                            data: "The homepage for the Asciidoctor Project is https://www.asciidoctor.org.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"The homepage for the Asciidoctor Project is <a href="https://www.asciidoctor.org" class="bare">https://www.asciidoctor.org</a>."#,
                    },
                    source: TSpan {
                        data: "The homepage for the Asciidoctor Project is https://www.asciidoctor.org.",
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
                    data: "The homepage for the Asciidoctor Project is https://www.asciidoctor.org.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn angle_brackets() {
        verifies!(
            r#"
In plain text documents, a bare URL is often enclosed in angle brackets.

[source]
----
You'll often see <https://example.org> used in examples.
----

To accommodate this convention, the AsciiDoc processor will still recognize the URL as an autolink, but will discard the angle brackets in the output (as they are not deemed significant).

"#
        );

        let doc =
            Parser::default().parse("You'll often see <https://example.org> used in examples.");

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
                            data: "You'll often see <https://example.org> used in examples.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: r#"You&#8217;ll often see <a href="https://example.org" class="bare">https://example.org</a> used in examples."#,
                    },
                    source: TSpan {
                        data: "You'll often see <https://example.org> used in examples.",
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
                    data: "You'll often see <https://example.org> used in examples.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}
