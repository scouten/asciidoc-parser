//! This file fills in a few coverage gaps after doing spec-driven development
//! (SDD) for macro parsing.

mod inline_link {
    use pretty_assertions_sorted::assert_eq;

    use crate::{Parser, tests::prelude::*};

    #[test]
    fn escape_angle_bracket_autolink_before_lt() {
        let doc =
            Parser::default().parse("You'll often see \\<https://example.org> used in examples.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "You'll often see \\<https://example.org> used in examples.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "You&#8217;ll often see &lt;https://example.org&gt; used in examples.",
                    },
                    source: Span {
                        data: "You'll often see \\<https://example.org> used in examples.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "You'll often see \\<https://example.org> used in examples.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn escape_angle_bracket_autolink_before_scheme() {
        let doc =
            Parser::default().parse("You'll often see <\\https://example.org> used in examples.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "You'll often see <\\https://example.org> used in examples.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "You&#8217;ll often see &lt;https://example.org&gt; used in examples.",
                    },
                    source: Span {
                        data: "You'll often see <\\https://example.org> used in examples.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "You'll often see <\\https://example.org> used in examples.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn empty_inside_angle_brackets() {
        let doc = Parser::default().parse("There's no actual link <https://> in here.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "There's no actual link <https://> in here.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "There&#8217;s no actual link &lt;https://&gt; in here.",
                    },
                    source: Span {
                        data: "There's no actual link <https://> in here.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "There's no actual link <https://> in here.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn hide_uri_scheme() {
        let doc = Parser::default().parse("= Test Page\n:hide-uri-scheme:\n\nWe don't want you to know that this is HTTP: <https://example.com> just now.");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: Some(Span {
                        data: "Test Page",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },),
                    title: Some("Test Page",),
                    attributes: &[Attribute {
                        name: Span {
                            data: "hide-uri-scheme",
                            line: 2,
                            col: 2,
                            offset: 13,
                        },
                        value_source: None,
                        value: InterpretedValue::Set,
                        source: Span {
                            data: ":hide-uri-scheme:",
                            line: 2,
                            col: 1,
                            offset: 12,
                        },
                    },],
                    source: Span {
                        data: "= Test Page\n:hide-uri-scheme:",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "We don't want you to know that this is HTTP: <https://example.com> just now.",
                            line: 4,
                            col: 1,
                            offset: 31,
                        },
                        rendered: "We don&#8217;t want you to know that this is HTTP: <a href=\"https://example.com\" class=\"bare\">example.com</a> just now.",
                    },
                    source: Span {
                        data: "We don't want you to know that this is HTTP: <https://example.com> just now.",
                        line: 4,
                        col: 1,
                        offset: 31,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "= Test Page\n:hide-uri-scheme:\n\nWe don't want you to know that this is HTTP: <https://example.com> just now.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn link_with_semicolon_suffix() {
        let doc = Parser::default().parse(
            "You shouldn't visit https://example.com; it's just there to illustrate examples.",
        );

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "You shouldn't visit https://example.com; it's just there to illustrate examples.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "You shouldn&#8217;t visit <a href=\"https://example.com\" class=\"bare\">https://example.com</a>; it&#8217;s just there to illustrate examples.",
                    },
                    source: Span {
                        data: "You shouldn't visit https://example.com; it's just there to illustrate examples.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "You shouldn't visit https://example.com; it's just there to illustrate examples.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn link_with_paren_and_colon_suffix() {
        let doc = Parser::default().parse(
            "You shouldn't visit that site (https://example.com): it's just there to illustrate examples.",
        );

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "You shouldn't visit that site (https://example.com): it's just there to illustrate examples.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "You shouldn&#8217;t visit that site (<a href=\"https://example.com\" class=\"bare\">https://example.com</a>): it&#8217;s just there to illustrate examples.",
                    },
                    source: Span {
                        data: "You shouldn't visit that site (https://example.com): it's just there to illustrate examples.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "You shouldn't visit that site (https://example.com): it's just there to illustrate examples.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn named_attributes_without_link_text_and_hide_uri_scheme() {
        let doc = Parser::default()
            .parse("= Test\n:hide-uri-scheme:\n\nhttps://chat.asciidoc.org[role=button,window=_blank,opts=nofollow]");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: Some(Span {
                        data: "Test",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },),
                    title: Some("Test",),
                    attributes: &[Attribute {
                        name: Span {
                            data: "hide-uri-scheme",
                            line: 2,
                            col: 2,
                            offset: 8,
                        },
                        value_source: None,
                        value: InterpretedValue::Set,
                        source: Span {
                            data: ":hide-uri-scheme:",
                            line: 2,
                            col: 1,
                            offset: 7,
                        },
                    },],
                    source: Span {
                        data: "= Test\n:hide-uri-scheme:",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "https://chat.asciidoc.org[role=button,window=_blank,opts=nofollow]",
                            line: 4,
                            col: 1,
                            offset: 26,
                        },
                        rendered: "<a href=\"https://chat.asciidoc.org\" class=\"bare button\" target=\"_blank\" rel=\"nofollow\" noopener>chat.asciidoc.org</a>",
                    },
                    source: Span {
                        data: "https://chat.asciidoc.org[role=button,window=_blank,opts=nofollow]",
                        line: 4,
                        col: 1,
                        offset: 26,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "= Test\n:hide-uri-scheme:\n\nhttps://chat.asciidoc.org[role=button,window=_blank,opts=nofollow]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}

mod link_macro {
    use pretty_assertions_sorted::assert_eq;

    use crate::{Parser, tests::prelude::*};

    #[test]
    fn escape_link_macro() {
        let doc =
            Parser::default().parse("A link macro looks like this: \\link:target[link text].");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "A link macro looks like this: \\link:target[link text].",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "A link macro looks like this: link:target[link text].",
                    },
                    source: Span {
                        data: "A link macro looks like this: \\link:target[link text].",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "A link macro looks like this: \\link:target[link text].",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn empty_mailto_link() {
        let doc = Parser::default().parse("mailto:[,Subscribe me]");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: None,
                    title: None,
                    attributes: &[],
                    source: Span {
                        data: "",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "mailto:[,Subscribe me]",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "mailto:[,Subscribe me]",
                    },
                    source: Span {
                        data: "mailto:[,Subscribe me]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "mailto:[,Subscribe me]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn empty_link_text_with_hide_uri_scheme() {
        let doc = Parser::default()
            .parse("= Test Document\n:hide-uri-scheme:\n\nlink:https://example.com[]");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: Some(Span {
                        data: "Test Document",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },),
                    title: Some("Test Document",),
                    attributes: &[Attribute {
                        name: Span {
                            data: "hide-uri-scheme",
                            line: 2,
                            col: 2,
                            offset: 17,
                        },
                        value_source: None,
                        value: InterpretedValue::Set,
                        source: Span {
                            data: ":hide-uri-scheme:",
                            line: 2,
                            col: 1,
                            offset: 16,
                        },
                    },],
                    source: Span {
                        data: "= Test Document\n:hide-uri-scheme:",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "link:https://example.com[]",
                            line: 4,
                            col: 1,
                            offset: 35,
                        },
                        rendered: "<a href=\"https://example.com\" class=\"bare\">example.com</a>",
                    },
                    source: Span {
                        data: "link:https://example.com[]",
                        line: 4,
                        col: 1,
                        offset: 35,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "= Test Document\n:hide-uri-scheme:\n\nlink:https://example.com[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }

    #[test]
    fn empty_mailto_link_text_with_hide_uri_scheme() {
        let doc = Parser::default()
            .parse("= Test Document\n:hide-uri-scheme:\n\nlink:mailto:fred@example.com[]");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: Some(Span {
                        data: "Test Document",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },),
                    title: Some("Test Document",),
                    attributes: &[Attribute {
                        name: Span {
                            data: "hide-uri-scheme",
                            line: 2,
                            col: 2,
                            offset: 17,
                        },
                        value_source: None,
                        value: InterpretedValue::Set,
                        source: Span {
                            data: ":hide-uri-scheme:",
                            line: 2,
                            col: 1,
                            offset: 16,
                        },
                    },],
                    source: Span {
                        data: "= Test Document\n:hide-uri-scheme:",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "link:mailto:fred@example.com[]",
                            line: 4,
                            col: 1,
                            offset: 35,
                        },
                        rendered: "<a href=\"mailto:fred@example.com\" class=\"bare\">fred@example.com</a>",
                    },
                    source: Span {
                        data: "link:mailto:fred@example.com[]",
                        line: 4,
                        col: 1,
                        offset: 35,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "= Test Document\n:hide-uri-scheme:\n\nlink:mailto:fred@example.com[]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}
