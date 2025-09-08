mod compound_delimited;
mod media;
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
        Parser,
        blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
        content::SubstitutionGroup,
        span::HasSpan,
        tests::fixtures::{
            Span,
            attributes::{Attrlist, ElementAttribute},
            blocks::{Block, SectionBlock, SimpleBlock},
            content::Content,
            warnings::TWarning,
        },
        warnings::{MatchAndWarnings, WarningType},
    };

    #[test]
    fn missing_block_after_title_line() {
        let mut parser = Parser::default();

        let MatchAndWarnings { item: mi, warnings } = crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("=== Section Title\n\nabc\n\n.ancestor section== Section 2\n\ndef"),
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item,
            SectionBlock {
                level: 2,
                section_title: Span {
                    data: "Section Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 19,
                            },
                            rendered: "abc",
                        },
                        source: Span {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 19,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    }),
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: ".ancestor section== Section 2",
                                line: 5,
                                col: 1,
                                offset: 24,
                            },
                            rendered: ".ancestor section== Section 2",
                        },
                        source: Span {
                            data: ".ancestor section== Section 2",
                            line: 5,
                            col: 1,
                            offset: 24,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "def",
                                line: 7,
                                col: 1,
                                offset: 55,
                            },
                            rendered: "def",
                        },
                        source: Span {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 55,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                source: Span {
                    // TO DO: Fix bug that includes blank lines.
                    data: "=== Section Title\n\nabc\n\n.ancestor section== Section 2\n\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 7,
                col: 4,
                offset: 58
            }
        );

        assert_eq!(
            warnings,
            vec![TWarning {
                source: Span {
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

        let mi = crate::blocks::Block::parse(
            crate::Span::new("[incomplete attrlist\n=== Section Title (except it isn't)\n\nabc\n"),
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "[incomplete attrlist\n=== Section Title (except it isn't)",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "[incomplete attrlist\n=== Section Title (except it isn&#8217;t)",
                },
                source: Span {
                    data: "[incomplete attrlist\n=== Section Title (except it isn't)",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            mi.after,
            Span {
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

        let MatchAndWarnings { item: mi, warnings } = crate::blocks::Block::parse(
            crate::Span::new(
                "[alt=\"Sunset\"width=300]\n=== Section Title (except it isn't)\n\nabc\n",
            ),
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.attrlist().unwrap(),
            Attrlist {
                attributes: &[ElementAttribute {
                    name: Some("alt"),
                    shorthand_items: &[],
                    value: "Sunset"
                },],
                source: Span {
                    data: "alt=\"Sunset\"width=300",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            }
        );

        assert_eq!(
            mi.item,
            Block::Section(SectionBlock {
                level: 2,
                section_title: Span {
                    data: "Section Title (except it isn't)",
                    line: 2,
                    col: 5,
                    offset: 28,
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "abc",
                            line: 4,
                            col: 1,
                            offset: 61,
                        },
                        rendered: "abc",
                    },
                    source: Span {
                        data: "abc",
                        line: 4,
                        col: 1,
                        offset: 61,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "[alt=\"Sunset\"width=300]\n=== Section Title (except it isn't)\n\nabc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: Some("alt"),
                        shorthand_items: &[],
                        value: "Sunset"
                    },],
                    source: Span {
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
            Span {
                data: "",
                line: 5,
                col: 1,
                offset: 65
            }
        );

        assert_eq!(
            warnings,
            vec![TWarning {
                source: Span {
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

        let mi = crate::blocks::Block::parse(crate::Span::new(". abc\ndef"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: ". abc\ndef",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: ". abc\ndef",
                },
                source: Span {
                    data: ". abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(
            mi.item.span(),
            Span {
                data: ". abc\ndef",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
