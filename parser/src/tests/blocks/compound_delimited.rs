mod is_valid_delimiter {
    use crate::{blocks::CompoundDelimitedBlock, Span};

    #[test]
    fn comment() {
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "////"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "/////"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "/////////"
        )));

        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "///"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "//-/"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "////-"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "//////////x"
        )));
    }

    #[test]
    fn example() {
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "===="
        )));
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "====="
        )));
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "======="
        )));

        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "==="
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "==-="
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "====-"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "==========x"
        )));
    }

    #[test]
    fn listing() {
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "----"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "-----"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "---------"
        )));
    }

    #[test]
    fn literal() {
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "...."
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "....."
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "........."
        )));
    }

    #[test]
    fn sidebar() {
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "****"
        )));
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "*****"
        )));
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "*********"
        )));

        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "***"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "**-*"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "****-"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "**********x"
        )));
    }

    #[test]
    fn table() {
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "|==="
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            ",==="
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            ":==="
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "!==="
        )));
    }

    #[test]
    fn pass() {
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "++++"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "+++++"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "+++++++++"
        )));
    }

    #[test]
    fn quote() {
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "____"
        )));
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "_____"
        )));
        assert!(CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "_________"
        )));

        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "___"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "__-_"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "____-"
        )));
        assert!(!CompoundDelimitedBlock::is_valid_delimiter(&Span::new(
            "_________x"
        )));
    }
}

mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, CompoundDelimitedBlock},
        tests::fixtures::{warnings::TWarning, TSpan},
        warnings::WarningType,
    };

    #[test]
    fn err_invalid_delimiter() {
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("///")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("////x")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("--x")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("****x")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("__\n__")).is_none());
    }

    #[test]
    fn err_unterminated() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new("====\nblah blah blah")).unwrap();

        assert!(maw.item.is_none());

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

mod comment {
    use crate::blocks::{preamble::Preamble, CompoundDelimitedBlock};

    #[test]
    fn empty() {
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("////\n////")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new("////\nline1  \nline2\n////")).is_none()
        );
    }
}

mod example {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, CompoundDelimitedBlock, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
    };

    #[test]
    fn empty() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new("====\n====")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "example",
                source: TSpan {
                    data: "====\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "example");
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw =
            CompoundDelimitedBlock::parse(&Preamble::new("====\nblock1\n\nblock2\n====")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },),
                        source: TSpan {
                            data: "block2\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
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
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "example");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },),
                source: TSpan {
                    data: "block2\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new(
            "====\nblock1\n\n=====\nblock2\n=====\n====",
        ))
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock {
                            inline: TInline::Uninterpreted(TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },),
                            source: TSpan {
                                data: "block2\n",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title: None,
                            attrlist: None,
                        },),),
                        context: "example",
                        source: TSpan {
                            data: "=====\nblock2\n=====\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
                        attrlist: None,
                    })
                ),
                context: "example",
                source: TSpan {
                    data: "====\nblock1\n\n=====\nblock2\n=====\n====",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "example");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },),
                    source: TSpan {
                        data: "block2\n",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title: None,
                    attrlist: None,
                },),),
                context: "example",
                source: TSpan {
                    data: "=====\nblock2\n=====\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());
    }
}

mod listing {
    use crate::blocks::{preamble::Preamble, CompoundDelimitedBlock};

    #[test]
    fn empty() {
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("----\n----")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new("----\nline1  \nline2\n----")).is_none()
        );
    }
}

mod literal {
    use crate::blocks::{preamble::Preamble, CompoundDelimitedBlock};

    #[test]
    fn empty() {
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("....\n....")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new("....\nline1  \nline2\n....")).is_none()
        );
    }
}

mod open {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, CompoundDelimitedBlock, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
    };

    #[test]
    fn empty() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new("--\n--")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "open",
                source: TSpan {
                    data: "--\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "open");
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw =
            CompoundDelimitedBlock::parse(&Preamble::new("--\nblock1\n\nblock2\n--")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },),
                        source: TSpan {
                            data: "block2\n",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        title: None,
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
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "open");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 11,
                },),
                source: TSpan {
                    data: "block2\n",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        // Spec says three hyphens does NOT mark an open block.
        let maw =
            CompoundDelimitedBlock::parse(&Preamble::new("--\nblock1\n\n---\nblock2\n---\n--"))
                .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 3,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Sequence(
                            vec![
                                TInline::Uninterpreted(TSpan {
                                    data: "---",
                                    line: 4,
                                    col: 1,
                                    offset: 11,
                                }),
                                TInline::Uninterpreted(TSpan {
                                    data: "block2",
                                    line: 5,
                                    col: 1,
                                    offset: 15,
                                }),
                                TInline::Uninterpreted(TSpan {
                                    data: "---",
                                    line: 6,
                                    col: 1,
                                    offset: 22,
                                }),
                            ],
                            TSpan {
                                data: "---\nblock2\n---\n",
                                line: 4,
                                col: 1,
                                offset: 11,
                            }
                        ),
                        source: TSpan {
                            data: "---\nblock2\n---\n",
                            line: 4,
                            col: 1,
                            offset: 11,
                        },
                        title: None,
                        attrlist: None,
                    })
                ),
                context: "open",
                source: TSpan {
                    data: "--\nblock1\n\n---\nblock2\n---\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "open");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 3,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 3,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Sequence(
                    vec![
                        TInline::Uninterpreted(TSpan {
                            data: "---",
                            line: 4,
                            col: 1,
                            offset: 11,
                        }),
                        TInline::Uninterpreted(TSpan {
                            data: "block2",
                            line: 5,
                            col: 1,
                            offset: 15,
                        }),
                        TInline::Uninterpreted(TSpan {
                            data: "---",
                            line: 6,
                            col: 1,
                            offset: 22,
                        }),
                    ],
                    TSpan {
                        data: "---\nblock2\n---\n",
                        line: 4,
                        col: 1,
                        offset: 11,
                    }
                ),
                source: TSpan {
                    data: "---\nblock2\n---\n",
                    line: 4,
                    col: 1,
                    offset: 11,
                },
                title: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());
    }
}

mod sidebar {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, CompoundDelimitedBlock, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
    };

    #[test]
    fn empty() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new("****\n****")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "sidebar",
                source: TSpan {
                    data: "****\n****",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "sidebar");
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw =
            CompoundDelimitedBlock::parse(&Preamble::new("****\nblock1\n\nblock2\n****")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },),
                        source: TSpan {
                            data: "block2\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
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
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "sidebar");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },),
                source: TSpan {
                    data: "block2\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new(
            "****\nblock1\n\n*****\nblock2\n*****\n****",
        ))
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock {
                            inline: TInline::Uninterpreted(TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },),
                            source: TSpan {
                                data: "block2\n",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title: None,
                            attrlist: None,
                        },),),
                        context: "sidebar",
                        source: TSpan {
                            data: "*****\nblock2\n*****\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
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
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "sidebar");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },),
                    source: TSpan {
                        data: "block2\n",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title: None,
                    attrlist: None,
                },),),
                context: "sidebar",
                source: TSpan {
                    data: "*****\nblock2\n*****\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());
    }
}

mod table {
    use crate::blocks::{preamble::Preamble, CompoundDelimitedBlock};

    #[test]
    fn empty() {
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("|===\n|===")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new(",===\n,===")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new(":===\n:===")).is_none());
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("!===\n!===")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new("|===\nline1  \nline2\n|===")).is_none()
        );
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new(",===\nline1  \nline2\n,===")).is_none()
        );
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new(":===\nline1  \nline2\n:===")).is_none()
        );
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new("!===\nline1  \nline2\n!===")).is_none()
        );
    }
}

mod pass {
    use crate::blocks::{preamble::Preamble, CompoundDelimitedBlock};

    #[test]
    fn empty() {
        assert!(CompoundDelimitedBlock::parse(&Preamble::new("++++\n++++")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(
            CompoundDelimitedBlock::parse(&Preamble::new("++++\nline1  \nline2\n++++")).is_none()
        );
    }
}

mod quote {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, CompoundDelimitedBlock, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TCompoundDelimitedBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
    };

    #[test]
    fn empty() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new("____\n____")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(),
                context: "quote",
                source: TSpan {
                    data: "____\n____",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "quote");
        assert!(mi.item.nested_blocks().next().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn multiple_blocks() {
        let maw =
            CompoundDelimitedBlock::parse(&Preamble::new("____\nblock1\n\nblock2\n____")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block2",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },),
                        source: TSpan {
                            data: "block2\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
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
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "quote");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block2",
                    line: 4,
                    col: 1,
                    offset: 13,
                },),
                source: TSpan {
                    data: "block2\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert!(blocks.next().is_none());
    }

    #[test]
    fn nested_blocks() {
        let maw = CompoundDelimitedBlock::parse(&Preamble::new(
            "____\nblock1\n\n_____\nblock2\n_____\n____",
        ))
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TCompoundDelimitedBlock {
                blocks: vec!(
                    TBlock::Simple(TSimpleBlock {
                        inline: TInline::Uninterpreted(TSpan {
                            data: "block1",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },),
                        source: TSpan {
                            data: "block1\n",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },
                        title: None,
                        attrlist: None,
                    },),
                    TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                        blocks: vec!(TBlock::Simple(TSimpleBlock {
                            inline: TInline::Uninterpreted(TSpan {
                                data: "block2",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },),
                            source: TSpan {
                                data: "block2\n",
                                line: 5,
                                col: 1,
                                offset: 19,
                            },
                            title: None,
                            attrlist: None,
                        },),),
                        context: "quote",
                        source: TSpan {
                            data: "_____\nblock2\n_____\n",
                            line: 4,
                            col: 1,
                            offset: 13,
                        },
                        title: None,
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
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.raw_context().as_ref(), "quote");
        assert!(mi.item.title().is_none());
        assert!(mi.item.attrlist().is_none());

        let mut blocks = mi.item.nested_blocks();
        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "block1",
                    line: 2,
                    col: 1,
                    offset: 5,
                },),
                source: TSpan {
                    data: "block1\n",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                attrlist: None,
            },)
        );

        assert_eq!(
            blocks.next().unwrap(),
            &TBlock::CompoundDelimited(TCompoundDelimitedBlock {
                blocks: vec!(TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "block2",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },),
                    source: TSpan {
                        data: "block2\n",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title: None,
                    attrlist: None,
                },),),
                context: "quote",
                source: TSpan {
                    data: "_____\nblock2\n_____\n",
                    line: 4,
                    col: 1,
                    offset: 13,
                },
                title: None,
                attrlist: None,
            })
        );

        assert!(blocks.next().is_none());
    }
}
