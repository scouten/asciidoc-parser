//! This file fills in a few coverage gaps after doing spec-driven development
//! (SDD) for macro behaviors.

mod inline_link {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TSimpleBlock},
            content::TContent,
            document::{TDocument, THeader},
        },
    };

    #[test]
    fn escape_angle_bracket_autolink_before_lt() {
        let doc =
            Parser::default().parse("You'll often see \\<https://example.org> used in examples.");

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
                            data: "You'll often see \\<https://example.org> used in examples.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "You&#8217;ll often see &lt;https://example.org&gt; used in examples.",
                    },
                    source: TSpan {
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
                source: TSpan {
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
                            data: "You'll often see <\\https://example.org> used in examples.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "You&#8217;ll often see &lt;https://example.org&gt; used in examples.",
                    },
                    source: TSpan {
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
                source: TSpan {
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
                            data: "There's no actual link <https://> in here.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "There&#8217;s no actual link &lt;https://&gt; in here.",
                    },
                    source: TSpan {
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
                source: TSpan {
                    data: "There's no actual link <https://> in here.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
            }
        );
    }
}
