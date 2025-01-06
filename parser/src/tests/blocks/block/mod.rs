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
        blocks::{preamble::Preamble, Block, ContentModel, IsBlock, SectionBlock},
        span::HasSpan,
        tests::fixtures::{
            attributes::{TAttrlist, TElementAttribute},
            blocks::{TBlock, TSectionBlock, TSimpleBlock},
            inlines::TInline,
            warnings::TWarning,
            TSpan,
        },
        warnings::{MatchAndWarnings, WarningType},
        Span,
    };

    #[test]
    fn missing_block_after_title_line() {
        let MatchAndWarnings { item: mi, warnings } = SectionBlock::parse(&Preamble::new(
            "=== Section Title\n\nabc\n\n.ancestor section== Section 2\n\ndef",
        ))
        .unwrap();

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().deref(), "section");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

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
                        inline: TInline::Uninterpreted(TSpan {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 19,
                        }),
                        source: TSpan {
                            data: "abc\n",
                            line: 3,
                            col: 1,
                            offset: 19,
                        },
                        title: None,
                        attrlist: None,
                    }),
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: ".ancestor section== Section 2",
                            line: 5,
                            col: 1,
                            offset: 24,
                        },),
                        source: TSpan {
                            data: ".ancestor section== Section 2\n",
                            line: 5,
                            col: 1,
                            offset: 24,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 55,
                        },),
                        source: TSpan {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 55,
                        },
                        title: None,
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
        let mi = Block::parse(Span::new(
            "[incomplete attrlist\n=== Section Title (except it isn't)\n\nabc\n",
        ))
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(mi.item.content_model(), ContentModel::Simple);
        assert_eq!(mi.item.raw_context().deref(), "paragraph");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                inline: TInline::Sequence(
                    vec![
                        TInline::Uninterpreted(TSpan {
                            data: "[incomplete attrlist",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },),
                        TInline::Uninterpreted(TSpan {
                            data: "=== Section Title (except it isn't)",
                            line: 2,
                            col: 1,
                            offset: 21,
                        },),
                    ],
                    TSpan {
                        data: "[incomplete attrlist\n=== Section Title (except it isn't)\n",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                ),
                source: TSpan {
                    data: "[incomplete attrlist\n=== Section Title (except it isn't)\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
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
        let MatchAndWarnings { item: mi, warnings } = Block::parse(Span::new(
            "[alt=\"Sunset\"width=300]\n=== Section Title (except it isn't)\n\nabc\n",
        ));

        let mi = mi.unwrap();

        dbg!(&mi);

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().deref(), "section");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.title().is_none());

        assert_eq!(
            mi.item.attrlist().unwrap(),
            TAttrlist {
                attributes: vec![TElementAttribute {
                    name: Some(TSpan {
                        data: "alt",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },),
                    shorthand_items: vec![],
                    value: TSpan {
                        data: "Sunset",
                        line: 1,
                        col: 7,
                        offset: 6,
                    },
                    source: TSpan {
                        data: "alt=\"Sunset\"",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
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
                    inline: TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 4,
                        col: 1,
                        offset: 61,
                    },),
                    source: TSpan {
                        data: "abc\n",
                        line: 4,
                        col: 1,
                        offset: 61,
                    },
                    title: None,
                    attrlist: None,
                },),],
                source: TSpan {
                    data: "[alt=\"Sunset\"width=300]\n=== Section Title (except it isn't)\n\nabc\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: Some(TAttrlist {
                    attributes: vec![TElementAttribute {
                        name: Some(TSpan {
                            data: "alt",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },),
                        shorthand_items: vec![],
                        value: TSpan {
                            data: "Sunset",
                            line: 1,
                            col: 7,
                            offset: 6,
                        },
                        source: TSpan {
                            data: "alt=\"Sunset\"",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
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
                    data: "width=300",
                    line: 1,
                    col: 14,
                    offset: 13,
                },
                warning: WarningType::MissingCommaAfterQuotedAttributeValue,
            },]
        );
    }

    #[test]
    #[allow(non_snake_case)]
    fn TEMP_not_title() {
        // IMPORTANT: This test will fail once we implement support for list items.
        let mi = Block::parse(Span::new(". abc\ndef"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                inline: TInline::Sequence(
                    vec![
                        TInline::Uninterpreted(TSpan {
                            data: ". abc",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },),
                        TInline::Uninterpreted(TSpan {
                            data: "def",
                            line: 2,
                            col: 1,
                            offset: 6,
                        },),
                    ],
                    TSpan {
                        data: ". abc\ndef",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                ),
                source: TSpan {
                    data: ". abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
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
