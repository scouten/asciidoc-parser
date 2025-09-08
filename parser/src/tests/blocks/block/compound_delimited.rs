mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        tests::fixtures::{
            Span,
            blocks::{Block, CompoundDelimitedBlock, SimpleBlock},
            content::Content,
            warnings::Warning,
        },
        warnings::WarningType,
    };

    #[test]
    fn err_invalid_delimiter() {
        let mut parser = Parser::default();

        let mi = crate::blocks::Block::parse(crate::Span::new("==="), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "===",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "===",
                },
                source: Span {
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

        let mi = crate::blocks::Block::parse(crate::Span::new("====x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "====x",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "====x",
                },
                source: Span {
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

        let mi = crate::blocks::Block::parse(crate::Span::new("****x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "****x",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "<strong>*</strong>*x",
                },
                source: Span {
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

        let mi = crate::blocks::Block::parse(crate::Span::new("____x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "____x",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "____x",
                },
                source: Span {
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
        let maw =
            crate::blocks::Block::parse(crate::Span::new("====\nblah blah blah"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "blah blah blah",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        rendered: "blah blah blah",
                    },
                    source: Span {
                        data: "blah blah blah",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),],
                context: "example",
                source: Span {
                    data: "====\nblah blah blah",
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
            maw.warnings,
            vec![Warning {
                source: Span {
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
        HasSpan, Parser,
        blocks::{ContentModel, IsBlock},
        content::SubstitutionGroup,
        tests::fixtures::{
            Span,
            blocks::{Block, CompoundDelimitedBlock, SimpleBlock},
            content::Content,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = crate::blocks::Block::parse(crate::Span::new("====\n===="), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[],
                context: "example",
                source: Span {
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
            Span {
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
        let maw = crate::blocks::Block::parse(
            crate::Span::new("====\nblock1\n\nblock2\n===="),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 13,
                            },
                            rendered: "block2",
                        },
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },
                    rendered: "block2",
                },
                source: Span {
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
            Span {
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

        let maw = crate::blocks::Block::parse(
            crate::Span::new(".block title \n====\nblock1\n\nblock2\n===="),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 3,
                                col: 1,
                                offset: 19,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 27,
                            },
                            rendered: "block2",
                        },
                        source: Span {
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
                source: Span {
                    data: ".block title \n====\nblock1\n\nblock2\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: Some(Span {
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
            Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 3,
                        col: 1,
                        offset: 19,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 27,
                    },
                    rendered: "block2",
                },
                source: Span {
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
            Span {
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

        let maw = crate::blocks::Block::parse(
            crate::Span::new("====\nblock1\n\n=====\nblock2\n=====\n===="),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::CompoundDelimited(CompoundDelimitedBlock {
                        blocks: &[Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "block2",
                                    line: 5,
                                    col: 1,
                                    offset: 19,
                                },
                                rendered: "block2",
                            },
                            source: Span {
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
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        rendered: "block2",
                    },
                    source: Span {
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
                source: Span {
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
            Span {
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
        HasSpan, Parser,
        blocks::{ContentModel, IsBlock},
        tests::fixtures::{
            Span,
            blocks::{Block, CompoundDelimitedBlock, SimpleBlock},
            content::Content,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = crate::blocks::Block::parse(crate::Span::new("--\n--"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[],
                context: "open",
                source: Span {
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
            Span {
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
        let maw =
            crate::blocks::Block::parse(crate::Span::new("--\nblock1\n\nblock2\n--"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 3,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 11,
                            },
                            rendered: "block2",
                        },
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 3,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 11,
                    },
                    rendered: "block2",
                },
                source: Span {
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
            Span {
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
        let maw = crate::blocks::Block::parse(
            crate::Span::new("--\nblock1\n\n---\nblock2\n---\n--"),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 3,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "---\nblock2\n---",
                                line: 4,
                                col: 1,
                                offset: 11,
                            },
                            rendered: "---\nblock2\n---",
                        },
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 3,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "---\nblock2\n---",
                        line: 4,
                        col: 1,
                        offset: 11,
                    },
                    rendered: "---\nblock2\n---",
                },
                source: Span {
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
            Span {
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
        HasSpan, Parser,
        blocks::{ContentModel, IsBlock},
        tests::fixtures::{
            Span,
            blocks::{Block, CompoundDelimitedBlock, SimpleBlock},
            content::Content,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = crate::blocks::Block::parse(crate::Span::new("****\n****"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[],
                context: "sidebar",
                source: Span {
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
            Span {
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
        let maw = crate::blocks::Block::parse(
            crate::Span::new("****\nblock1\n\nblock2\n****"),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 13,
                            },
                            rendered: "block2",
                        },
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },
                    rendered: "block2",
                },
                source: Span {
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
            Span {
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

        let maw = crate::blocks::Block::parse(
            crate::Span::new("****\nblock1\n\n*****\nblock2\n*****\n****"),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::CompoundDelimited(CompoundDelimitedBlock {
                        blocks: &[Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "block2",
                                    line: 5,
                                    col: 1,
                                    offset: 19,
                                },
                                rendered: "block2",
                            },
                            source: Span {
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
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        rendered: "block2",
                    },
                    source: Span {
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
                source: Span {
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
            Span {
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
        HasSpan, Parser,
        blocks::{ContentModel, IsBlock},
        tests::fixtures::{
            Span,
            blocks::{Block, CompoundDelimitedBlock, SimpleBlock},
            content::Content,
        },
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = crate::blocks::Block::parse(crate::Span::new("____\n____"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[],
                context: "quote",
                source: Span {
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
            Span {
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
        let maw = crate::blocks::Block::parse(
            crate::Span::new("____\nblock1\n\nblock2\n____"),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block2",
                                line: 4,
                                col: 1,
                                offset: 13,
                            },
                            rendered: "block2",
                        },
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },
                    rendered: "block2",
                },
                source: Span {
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
            Span {
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

        let maw = crate::blocks::Block::parse(
            crate::Span::new("____\nblock1\n\n_____\nblock2\n_____\n____"),
            &mut parser,
        );

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "block1",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            rendered: "block1",
                        },
                        source: Span {
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
                    Block::CompoundDelimited(CompoundDelimitedBlock {
                        blocks: &[Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "block2",
                                    line: 5,
                                    col: 1,
                                    offset: 19,
                                },
                                rendered: "block2",
                            },
                            source: Span {
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
                        source: Span {
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
                source: Span {
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
            &Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "block1",
                },
                source: Span {
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
            &Block::CompoundDelimited(CompoundDelimitedBlock {
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        rendered: "block2",
                    },
                    source: Span {
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
                source: Span {
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
            Span {
                data: "____\nblock1\n\n_____\nblock2\n_____\n____",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
