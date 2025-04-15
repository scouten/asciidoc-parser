mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::Block,
        tests::fixtures::{
            blocks::{TBlock, TSimpleBlock},
            warnings::TWarning,
            TSpan,
        },
        warnings::WarningType,
        Span,
    };

    #[test]
    fn err_invalid_delimiter() {
        let mi = Block::parse(Span::new("==="))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "===",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "===",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        let mi = Block::parse(Span::new("====x"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "====x",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "====x",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        let mi = Block::parse(Span::new("****x"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "****x",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "****x",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        let mi = Block::parse(Span::new("____x"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "____x",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "____x",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );
    }

    #[test]
    fn err_unterminated() {
        let maw = Block::parse(Span::new("====\nblah blah blah"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "====\nblah blah blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                source: TSpan {
                    data: "====\nblah blah blah",
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
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            TSpan,
        },
        HasSpan, Span,
    };

    #[test]
    fn empty() {
        let maw = Block::parse(Span::new("====\n===="));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "example",
                source: TSpan {
                    data: "====\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

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
        let maw = Block::parse(Span::new("====\nblock1\n\nblock2\n===="));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ),
                context: "example",
                source: TSpan {
                    data: "====\nblock1\n\nblock2\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
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
        let maw = Block::parse(Span::new(".block title \n====\nblock1\n\nblock2\n===="));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 3,
                            col: 1,
                            offset: 19,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 3,
                            col: 1,
                            offset: 19,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 27,
                        },
                        source: TSpan {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 27,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ),
                context: "example",
                source: TSpan {
                    data: ".block title \n====\nblock1\n\nblock2\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: Some(TSpan {
                    data: "block title",
                    line: 1,
                    col: 2,
                    offset: 1,
                }),
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
            mi.item.title().unwrap(),
            TSpan {
                data: "block title",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 3,
                    col: 1,
                    offset: 19,
                },
                source: TSpan {
                    data: "block1",
                    line: 3,
                    col: 1,
                    offset: 19,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block2",
                    line: 5,
                    col: 1,
                    offset: 27,
                },
                source: TSpan {
                    data: "block2",
                    line: 5,
                    col: 1,
                    offset: 27,
                },
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
        let maw = Block::parse(Span::new("====\nblock1\n\n=====\nblock2\n=====\n===="));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock {
                            content: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            source: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },)),
                        context: "example",
                        source: TSpan {
                            data: "=====\nblock2\n=====",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },)
                ),
                context: "example",
                source: TSpan {
                    data: "====\nblock1\n\n=====\nblock2\n=====\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock {
                    content: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    source: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),),
                context: "example",
                source: TSpan {
                    data: "=====\nblock2\n=====",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
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
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            TSpan,
        },
        HasSpan, Span,
    };

    #[test]
    fn empty() {
        let maw = Block::parse(Span::new("--\n--"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "open",
                source: TSpan {
                    data: "--\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        let maw = Block::parse(Span::new("--\nblock1\n\nblock2\n--"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ),
                context: "open",
                source: TSpan {
                    data: "--\nblock1\n\nblock2\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
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
        let maw = Block::parse(Span::new("--\nblock1\n\n---\nblock2\n---\n--"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "---\nblock2\n---",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        source: TSpan {
                            data: "---\nblock2\n---",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },)
                ),
                context: "open",
                source: TSpan {
                    data: "--\nblock1\n\n---\nblock2\n---\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "---\nblock2\n---",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
                source: TSpan {
                    data: "---\nblock2\n---",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
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
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            TSpan,
        },
        HasSpan, Span,
    };

    #[test]
    fn empty() {
        let maw = Block::parse(Span::new("****\n****"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "sidebar",
                source: TSpan {
                    data: "****\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        let maw = Block::parse(Span::new("****\nblock1\n\nblock2\n****"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ),
                context: "sidebar",
                source: TSpan {
                    data: "****\nblock1\n\nblock2\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
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
        let maw = Block::parse(Span::new("****\nblock1\n\n*****\nblock2\n*****\n****"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock {
                            content: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            source: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },)),
                        context: "sidebar",
                        source: TSpan {
                            data: "*****\nblock2\n*****",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    })
                ),
                context: "sidebar",
                source: TSpan {
                    data: "****\nblock1\n\n*****\nblock2\n*****\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock {
                    content: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    source: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),),
                context: "sidebar",
                source: TSpan {
                    data: "*****\nblock2\n*****",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
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
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            TSpan,
        },
        HasSpan, Span,
    };

    #[test]
    fn empty() {
        let maw = Block::parse(Span::new("____\n____"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "quote",
                source: TSpan {
                    data: "____\n____",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        let maw = Block::parse(Span::new("____\nblock1\n\nblock2\n____"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        source: TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                ),
                context: "quote",
                source: TSpan {
                    data: "____\nblock1\n\nblock2\n____",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                source: TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
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
        let maw = Block::parse(Span::new("____\nblock1\n\n_____\nblock2\n_____\n____"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        content: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        source: TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock {
                            content: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            source: TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title: None,
                            anchor: None,
                            attrlist: None,
                        },),),
                        context: "quote",
                        source: TSpan {
                            data: "_____\nblock2\n_____",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
                        anchor: None,
                        attrlist: None,
                    })
                ),
                context: "quote",
                source: TSpan {
                    data: "____\nblock1\n\n_____\nblock2\n_____\n____",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                content: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                source: TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock {
                    content: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    source: TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),),
                context: "quote",
                source: TSpan {
                    data: "_____\nblock2\n_____",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
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
