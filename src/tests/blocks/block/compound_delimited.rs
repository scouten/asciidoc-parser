mod example {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
        Span,
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "example");
        assert!(mi.item.nested_blocks().next().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw = Block::parse(Span::new("====\nblock1\n\nblock2\n===="));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },),),),
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },),),),
                ),
                context: "example",
                source: TSpan {
                    data: "====\nblock1\n\nblock2\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "example");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 5,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block2",
                line: 4,
                col: 1,
                offset: 13,
            },),),)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        let maw = Block::parse(Span::new("====\nblock1\n\n=====\nblock2\n=====\n===="));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },),),),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                            TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                        ),),),),
                        context: "example",
                        source: TSpan {
                            data: "=====\nblock2\n=====\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                    })
                ),
                context: "example",
                source: TSpan {
                    data: "====\nblock1\n\n=====\nblock2\n=====\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "example");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 5,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                    TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                ),),),),
                context: "example",
                source: TSpan {
                    data: "=====\nblock2\n=====\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
            })
        );

        assert!(blocks.next().is_none());
    }
}

mod open {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
        Span,
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "open");
        assert!(mi.item.nested_blocks().next().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw = Block::parse(Span::new("--\nblock1\n\nblock2\n--"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 3,
                    },),),),
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 11,
                    },),),),
                ),
                context: "open",
                source: TSpan {
                    data: "--\nblock1\n\nblock2\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "open");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 3,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block2",
                line: 4,
                col: 1,
                offset: 11,
            },),),)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        let maw = Block::parse(Span::new("--\nblock1\n\n---\nblock2\n---\n--"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 3,
                    },),),),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                            TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 15,
                            },
                        ),),),),
                        context: "open",
                        source: TSpan {
                            data: "---\nblock2\n---\n",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                    })
                ),
                context: "open",
                source: TSpan {
                    data: "--\nblock1\n\n---\nblock2\n---\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "open");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 3,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                    TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 15,
                    },
                ),),),),
                context: "open",
                source: TSpan {
                    data: "---\nblock2\n---\n",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
            })
        );

        assert!(blocks.next().is_none());
    }
}

mod sidebar {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
        Span,
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "sidebar");
        assert!(mi.item.nested_blocks().next().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw = Block::parse(Span::new("****\nblock1\n\nblock2\n****"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },),),),
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },),),),
                ),
                context: "sidebar",
                source: TSpan {
                    data: "****\nblock1\n\nblock2\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "sidebar");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 5,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block2",
                line: 4,
                col: 1,
                offset: 13,
            },),),)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        let maw = Block::parse(Span::new("****\nblock1\n\n*****\nblock2\n*****\n****"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },),),),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                            TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                        ),),),),
                        context: "sidebar",
                        source: TSpan {
                            data: "*****\nblock2\n*****\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                    })
                ),
                context: "sidebar",
                source: TSpan {
                    data: "****\nblock1\n\n*****\nblock2\n*****\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "sidebar");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 5,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                    TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                ),),),),
                context: "sidebar",
                source: TSpan {
                    data: "*****\nblock2\n*****\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
            })
        );

        assert!(blocks.next().is_none());
    }
}

mod quote {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
        Span,
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "quote");
        assert!(mi.item.nested_blocks().next().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw = Block::parse(Span::new("____\nblock1\n\nblock2\n____"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },),),),
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block2",
                        line: 4,
                        col: 1,
                        offset: 13,
                    },),),),
                ),
                context: "quote",
                source: TSpan {
                    data: "____\nblock1\n\nblock2\n____",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "quote");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 5,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block2",
                line: 4,
                col: 1,
                offset: 13,
            },),),)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        let maw = Block::parse(Span::new("____\nblock1\n\n_____\nblock2\n_____\n____"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                        data: "block1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },),),),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                            TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                        ),),),),
                        context: "quote",
                        source: TSpan {
                            data: "_____\nblock2\n_____\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                    })
                ),
                context: "quote",
                source: TSpan {
                    data: "____\nblock1\n\n_____\nblock2\n_____\n____",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().as_ref(), "quote");

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "block1",
                line: 2,
                col: 1,
                offset: 5,
            },),),)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                    TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                ),),),),
                context: "quote",
                source: TSpan {
                    data: "_____\nblock2\n_____\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
            })
        );

        assert!(blocks.next().is_none());
    }
}
