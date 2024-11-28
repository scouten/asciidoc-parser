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
        assert_eq!(mi.item.context().deref(), "section");
        assert!(mi.item.title().is_none());

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
                        title: None
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
                title: None
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
