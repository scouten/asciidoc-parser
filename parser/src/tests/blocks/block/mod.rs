mod compound_delimited;
mod r#macro;
mod raw_delimited;
mod section;
mod simple;

mod content_model {
    use crate::blocks::ContentModel;

    #[test]
    fn impl_copy() {
        // Silly test to mark the #[derive(...)] line as covered.
        let c1 = ContentModel::Simple;
        let c2 = c1;
        assert_eq!(c1, c2);
    }
}

mod error_cases {
    use std::ops::Deref;

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser, Span,
        blocks::{Block, ContentModel, IsBlock, SectionBlock, preamble::Preamble},
        span::{HasSpan, content::SubstitutionGroup},
        tests::fixtures::{
            TSpan,
            attributes::{TAttrlist, TElementAttribute},
            blocks::{TBlock, TSectionBlock, TSimpleBlock},
            content::TContent,
            warnings::TWarning,
        },
        warnings::{MatchAndWarnings, WarningType},
    };

    #[test]
    fn missing_block_after_title_line() {
        let mut parser = Parser::default();

        let MatchAndWarnings { item: mi, warnings } = SectionBlock::parse(
            &Preamble::new("=== Section Title\n\nabc\n\n.ancestor section== Section 2\n\ndef"),
            &mut parser,
        )
        .unwrap();

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().deref(), "section");
        assert_eq!(mi.item.resolved_context().deref(), "section");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item,
            TSectionBlock {
                level: 2,
                section_title: TSpan {
                    data: "Section Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                blocks: vec![
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 19,
                            },
                            rendered: "abc",
                        },
                        source: TSpan {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 19,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    }),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: ".ancestor section== Section 2",
                                line: 5,
                                col: 1,
                                offset: 24,
                            },
                            rendered: ".ancestor section== Section 2",
                        },
                        source: TSpan {
                            data: ".ancestor section== Section 2",
                            line: 5,
                            col: 1,
                            offset: 24,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "def",
                                line: 7,
                                col: 1,
                                offset: 55,
                            },
                            rendered: "def",
                        },
                        source: TSpan {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 55,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                source: TSpan {
                    // TO DO: Fix bug that includes blank lines.
                    data: "=== Section Title\n\nabc\n\n.ancestor section== Section 2\n\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 7,
                col: 4,
                offset: 58
            }
        );

        assert_eq!(
            warnings,
            vec![TWarning {
                source: TSpan {
                    data: ".ancestor section== Section 2\n\ndef",
                    line: 5,
                    col: 1,
                    offset: 24,
                },
                warning: WarningType::MissingBlockAfterTitleOrAttributeList,
            },]
        );
    }

    #[test]
    fn missing_close_brace_on_attrlist() {
        let mut parser = Parser::default();

        let mi = Block::parse(
            Span::new("[incomplete attrlist\n=== Section Title (except it isn't)\n\nabc\n"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(mi.item.content_model(), ContentModel::Simple);
        assert_eq!(mi.item.raw_context().deref(), "paragraph");
        assert_eq!(mi.item.resolved_context().deref(), "paragraph");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "[incomplete attrlist\n=== Section Title (except it isn't)",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "[incomplete attrlist\n=== Section Title (except it isn&#8217;t)",
                },
                source: TSpan {
                    data: "[incomplete attrlist\n=== Section Title (except it isn't)",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "abc\n",
                line: 4,
                col: 1,
                offset: 58
            }
        );
    }

    #[test]
    fn attrlist_warning_carried_forward() {
        let mut parser = Parser::default();

        let MatchAndWarnings { item: mi, warnings } = Block::parse(
            Span::new("[alt=\"Sunset\"width=300]\n=== Section Title (except it isn't)\n\nabc\n"),
            &mut parser,
        );

        let mi = mi.unwrap();

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().deref(), "section");
        assert_eq!(mi.item.resolved_context().deref(), "section");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.attrlist().unwrap(),
            TAttrlist {
                attributes: vec![TElementAttribute {
                    name: Some("alt"),
                    shorthand_items: vec![],
                    value: "Sunset"
                },],
                source: TSpan {
                    data: "alt=\"Sunset\"width=300",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            }
        );

        assert_eq!(
            mi.item,
            TBlock::Section(TSectionBlock {
                level: 2,
                section_title: TSpan {
                    data: "Section Title (except it isn't)",
                    line: 2,
                    col: 5,
                    offset: 28,
                },
                blocks: vec![TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "abc",
                            line: 4,
                            col: 1,
                            offset: 61,
                        },
                        rendered: "abc",
                    },
                    source: TSpan {
                        data: "abc",
                        line: 4,
                        col: 1,
                        offset: 61,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "[alt=\"Sunset\"width=300]\n=== Section Title (except it isn't)\n\nabc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: Some(TAttrlist {
                    attributes: vec![TElementAttribute {
                        name: Some("alt"),
                        shorthand_items: vec![],
                        value: "Sunset"
                    },],
                    source: TSpan {
                        data: "alt=\"Sunset\"width=300",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            },)
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 5,
                col: 1,
                offset: 65
            }
        );

        assert_eq!(
            warnings,
            vec![TWarning {
                source: TSpan {
                    data: "alt=\"Sunset\"width=300",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
                warning: WarningType::MissingCommaAfterQuotedAttributeValue,
            },]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn TEMP_not_title() {
        // IMPORTANT: This test will fail once we implement support for list items.
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new(". abc\ndef"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: ". abc\ndef",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: ". abc\ndef",
                },
                source: TSpan {
                    data: ". abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ". abc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
