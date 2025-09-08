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
        sdd::{non_normative, track_file, verifies},
    },
};

track_file!("docs/modules/macros/pages/link-macro-ref.adoc");

non_normative!(
    r#"
= Link, URL, and Mailto Macro Attributes Reference

These attributes apply to the link, URL, and mailto (email) macros.

[%autowidth]
|===
|Attribute |Value(s) |Example Syntax |Comments

"#
);

#[test]
fn id() {
    verifies!(
        r#"
|`id`
|Unique identifier for element in output
|+https://asciidoctor.org[Home,id=home]+
|

"#
    );

    let doc = Parser::default().parse("https://asciidoctor.org[Home,id=home]");

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
                        data: "https://asciidoctor.org[Home,id=home]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<a href=\"https://asciidoctor.org\" id=\"home\">Home</a>",
                },
                source: TSpan {
                    data: "https://asciidoctor.org[Home,id=home]",
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
                data: "https://asciidoctor.org[Home,id=home]",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn role() {
    verifies!(
        r#"
|`role`
|CSS classes available to inline elements
|+https://chat.asciidoc.org[Discuss AsciiDoc,role=teal]+
|

"#
    );

    let doc = Parser::default().parse("https://chat.asciidoc.org[Discuss AsciiDoc,role=teal]");

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
                        data: "https://chat.asciidoc.org[Discuss AsciiDoc,role=teal]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<a href=\"https://chat.asciidoc.org\" class=\"teal\">Discuss AsciiDoc</a>",
                },
                source: TSpan {
                    data: "https://chat.asciidoc.org[Discuss AsciiDoc,role=teal]",
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
                data: "https://chat.asciidoc.org[Discuss AsciiDoc,role=teal]",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn title() {
    verifies!(
        r#"
|`title`
|Description of link, often show as tooltip.
|+https://asciidoctor.org[Home,title=Project home page]+
|

"#
    );

    let doc = Parser::default().parse("https://asciidoctor.org[Home,title=Project home page]");

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
                        data: "https://asciidoctor.org[Home,title=Project home page]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<a href=\"https://asciidoctor.org\">Home</a>",
                },
                source: TSpan {
                    data: "https://asciidoctor.org[Home,title=Project home page]",
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
                data: "https://asciidoctor.org[Home,title=Project home page]",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn window() {
    verifies!(
        r#"
|`window`
|any
|+https://chat.asciidoc.org[Discuss AsciiDoc,window=_blank]+
|The blank window target can also be specified using `^` at the end of the link text.

"#
    );

    let doc = Parser::default().parse("https://chat.asciidoc.org[Discuss AsciiDoc,window=_blank]");

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
                        data: "https://chat.asciidoc.org[Discuss AsciiDoc,window=_blank]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<a href=\"https://chat.asciidoc.org\" target=\"_blank\" rel=\"noopener\">Discuss AsciiDoc</a>",
                },
                source: TSpan {
                    data: "https://chat.asciidoc.org[Discuss AsciiDoc,window=_blank]",
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
                data: "https://chat.asciidoc.org[Discuss AsciiDoc,window=_blank]",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn window_shorthand() {
    verifies!(
        r#"
|`window` +
(shorthand)
|`^`
|+https://example.org[Google, DuckDuckGo, Ecosia^]+ +
+https://chat.asciidoc.org[Discuss AsciiDoc^]+
|

"#
    );

    let doc = Parser::default().parse("https://example.org[Google, DuckDuckGo, Ecosia^]");

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
                        data: "https://example.org[Google, DuckDuckGo, Ecosia^]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<a href=\"https://example.org\" target=\"_blank\" rel=\"noopener\">Google, DuckDuckGo, Ecosia</a>",
                },
                source: TSpan {
                    data: "https://example.org[Google, DuckDuckGo, Ecosia^]",
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
                data: "https://example.org[Google, DuckDuckGo, Ecosia^]",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn opts() {
    verifies!(
        r#"
|`opts`
|Additional options for link creation.
|+https://asciidoctor.org[Home,opts=nofollow]+
|Option names include: `nofollow`, `noopener`
"#
    );

    let doc = Parser::default().parse("https://asciidoctor.org[Home,opts=nofollow]");

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
                        data: "https://asciidoctor.org[Home,opts=nofollow]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<a href=\"https://asciidoctor.org\" rel=\"nofollow\">Home</a>",
                },
                source: TSpan {
                    data: "https://asciidoctor.org[Home,opts=nofollow]",
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
                data: "https://asciidoctor.org[Home,opts=nofollow]",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

non_normative!(
    r#"
|===
"#
);
