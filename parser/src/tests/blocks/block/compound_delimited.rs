mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser, Span,
        blocks::Block,
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TSimpleBlock},
            content::TContent,
            warnings::TWarning,
        },
        warnings::WarningType,
    };

    #[test]
    fn err_invalid_delimiter() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("==="), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "===",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "===",
                },
                source: TSpan {
                    data: "===",
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

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("====x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "====x",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "====x",
                },
                source: TSpan {
                    data: "====x",
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

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("****x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "****x",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<strong>*</strong>*x",
                },
                source: TSpan {
                    data: "****x",
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

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("____x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "____x",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "____x",
                },
                source: TSpan {
                    data: "____x",
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
    }

    #[test]
    fn err_unterminated() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("====\nblah blah blah"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "====\nblah blah blah",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "====\nblah blah blah",
                },
                source: TSpan {
                    data: "====\nblah blah blah",
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
            maw.warnings,
            vec![TWarning {
                source: TSpan {
                    data: "====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warning: WarningType::UnterminatedDelimitedBlock,
            }]
        );
    }
}

mod example {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        HasSpan, Parser, Span,
        blocks::{Block, ContentModel, IsBlock},
        span::content::SubstitutionGroup,
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            content::TContent,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("====\n===="), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[],
                context: "example",
                source: TSpan {
                    data: "====\n====",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "example");
        assert_eq!(mi.item.resolved_context().as_ref(), "example");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "====\n====",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn multiple_blocks() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("====\nblock1\n\nblock2\n===="), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 13,
                            },
                            rendered: "block2",
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                context: "example",
                source: TSpan {
                    data: "====\nblock1\n\nblock2\n====",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "example");
        assert_eq!(mi.item.resolved_context().as_ref(), "example");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },
                    rendered: "block2",
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "====\nblock1\n\nblock2\n====",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn title() {
        let mut parser = Parser::default();

        let maw = Block::parse(
            Span::new(".block title \n====\nblock1\n\nblock2\n===="),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 3,
                                col: 1,
                                offset: 19,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 3,
                            col: 1,
                            offset: 19,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 27,
                            },
                            rendered: "block2",
                        },
                        source: TSpan {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 27,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                context: "example",
                source: TSpan {
                    data: ".block title \n====\nblock1\n\nblock2\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: Some(TSpan {
                    data: "block title",
                    line: 1,
                    col: 2,
                    offset: 1,
                }),
                title: Some("block title"),
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "example");
        assert_eq!(mi.item.resolved_context().as_ref(), "example");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.title_source().unwrap(),
            TSpan {
                data: "block title",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(mi.item.title_source().unwrap().data(), "block title");
        assert_eq!(mi.item.title().unwrap(), "block title");

        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 3,
                        col: 1,
                        offset: 19,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 3,
                    col: 1,
                    offset: 19,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 27,
                    },
                    rendered: "block2",
                },
                source: TSpan {
                    data: "block2",
                    line: 5,
                    col: 1,
                    offset: 27,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ".block title \n====\nblock1\n\nblock2\n====",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn nested_blocks() {
        let mut parser = Parser::default();

        let maw = Block::parse(
            Span::new("====\nblock1\n\n=====\nblock2\n=====\n===="),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: &[TBlock::Simple(TSimpleBlock {
                            content: TContent {
                                original: TSpan {
                                    data: "block2",
                                    line: 5,
                                    col: 1,
                                    offset: 19,
                                },
                                rendered: "block2",
                            },
                            source: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },)],
                        context: "example",
                        source: TSpan {
                            data: "=====\nblock2\n=====",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },)
                ],
                context: "example",
                source: TSpan {
                    data: "====\nblock1\n\n=====\nblock2\n=====\n====",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "example");
        assert_eq!(mi.item.resolved_context().as_ref(), "example");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        rendered: "block2",
                    },
                    source: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "example",
                source: TSpan {
                    data: "=====\nblock2\n=====",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "====\nblock1\n\n=====\nblock2\n=====\n====",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod open {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        HasSpan, Parser, Span,
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            content::TContent,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("--\n--"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[],
                context: "open",
                source: TSpan {
                    data: "--\n--",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "open");
        assert_eq!(mi.item.resolved_context().as_ref(), "open");
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "--\n--",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn multiple_blocks() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("--\nblock1\n\nblock2\n--"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 3,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 11,
                            },
                            rendered: "block2",
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                context: "open",
                source: TSpan {
                    data: "--\nblock1\n\nblock2\n--",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "open");
        assert_eq!(mi.item.resolved_context().as_ref(), "open");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 3,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 11,
                    },
                    rendered: "block2",
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "--\nblock1\n\nblock2\n--",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn nested_blocks() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("--\nblock1\n\n---\nblock2\n---\n--"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 3,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "---\nblock2\n---",
                                line: 4,
                                col: 1,
                                offset: 11,
                            },
                            rendered: "---\nblock2\n---",
                        },
                        source: TSpan {
                            data: "---\nblock2\n---",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },)
                ],
                context: "open",
                source: TSpan {
                    data: "--\nblock1\n\n---\nblock2\n---\n--",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "open");
        assert_eq!(mi.item.resolved_context().as_ref(), "open");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 3,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "---\nblock2\n---",
                        line: 4,
                        col: 1,
                        offset: 11,
                    },
                    rendered: "---\nblock2\n---",
                },
                source: TSpan {
                    data: "---\nblock2\n---",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "--\nblock1\n\n---\nblock2\n---\n--",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod sidebar {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        HasSpan, Parser, Span,
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            content::TContent,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("****\n****"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[],
                context: "sidebar",
                source: TSpan {
                    data: "****\n****",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "sidebar");
        assert_eq!(mi.item.resolved_context().as_ref(), "sidebar");
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "****\n****",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn multiple_blocks() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("****\nblock1\n\nblock2\n****"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 13,
                            },
                            rendered: "block2",
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                context: "sidebar",
                source: TSpan {
                    data: "****\nblock1\n\nblock2\n****",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "sidebar");
        assert_eq!(mi.item.resolved_context().as_ref(), "sidebar");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },
                    rendered: "block2",
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "****\nblock1\n\nblock2\n****",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn nested_blocks() {
        let mut parser = Parser::default();

        let maw = Block::parse(
            Span::new("****\nblock1\n\n*****\nblock2\n*****\n****"),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: &[TBlock::Simple(TSimpleBlock {
                            content: TContent {
                                original: TSpan {
                                    data: "block2",
                                    line: 5,
                                    col: 1,
                                    offset: 19,
                                },
                                rendered: "block2",
                            },
                            source: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },)],
                        context: "sidebar",
                        source: TSpan {
                            data: "*****\nblock2\n*****",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    })
                ],
                context: "sidebar",
                source: TSpan {
                    data: "****\nblock1\n\n*****\nblock2\n*****\n****",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "sidebar");
        assert_eq!(mi.item.resolved_context().as_ref(), "sidebar");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        rendered: "block2",
                    },
                    source: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "sidebar",
                source: TSpan {
                    data: "*****\nblock2\n*****",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "****\nblock1\n\n*****\nblock2\n*****\n****",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod quote {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        HasSpan, Parser, Span,
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            content::TContent,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("____\n____"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[],
                context: "quote",
                source: TSpan {
                    data: "____\n____",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "quote");
        assert_eq!(mi.item.resolved_context().as_ref(), "quote");
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "____\n____",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn multiple_blocks() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("____\nblock1\n\nblock2\n____"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 13,
                            },
                            rendered: "block2",
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ],
                context: "quote",
                source: TSpan {
                    data: "____\nblock1\n\nblock2\n____",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "quote");
        assert_eq!(mi.item.resolved_context().as_ref(), "quote");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },
                    rendered: "block2",
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "____\nblock1\n\nblock2\n____",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn nested_blocks() {
        let mut parser = Parser::default();

        let maw = Block::parse(
            Span::new("____\nblock1\n\n_____\nblock2\n_____\n____"),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[
                    TBlock::Simple(TSimpleBlock {
                        content: TContent {
                            original: TSpan {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: &[TBlock::Simple(TSimpleBlock {
                            content: TContent {
                                original: TSpan {
                                    data: "block2",
                                    line: 5,
                                    col: 1,
                                    offset: 19,
                                },
                                rendered: "block2",
                            },
                            source: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },),],
                        context: "quote",
                        source: TSpan {
                            data: "_____\nblock2\n_____",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        attrlist: None,
                    })
                ],
                context: "quote",
                source: TSpan {
                    data: "____\nblock1\n\n_____\nblock2\n_____\n____",
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

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "quote");
        assert_eq!(mi.item.resolved_context().as_ref(), "quote");
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: &[TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: TSpan {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        rendered: "block2",
                    },
                    source: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "quote",
                source: TSpan {
                    data: "_____\nblock2\n_____",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "____\nblock1\n\n_____\nblock2\n_____\n____",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
