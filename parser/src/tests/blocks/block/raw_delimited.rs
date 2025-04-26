mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::Block,
        tests::fixtures::{
            blocks::{TBlock, TSimpleBlock},
            warnings::TWarning,
            TContent, TSpan,
        },
        warnings::WarningType,
        Parser, Span,
    };

    #[test]
    fn err_invalid_delimiter() {
        let mut parser = Parser::default();

        assert!(Block::parse(Span::new(""), &mut parser)
            .unwrap_if_no_warnings()
            .is_none());

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("..."), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent::Passthrough(TSpan {
                    data: "...",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                source: TSpan {
                    data: "...",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("++++x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent::Passthrough(TSpan {
                    data: "++++x",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                source: TSpan {
                    data: "++++x",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
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
                content: TContent::Passthrough(TSpan {
                    data: "____x",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
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

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("====x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent::Passthrough(TSpan {
                    data: "====x",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
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
    }

    #[test]
    fn err_unterminated() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("....\nblah blah blah"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent::Passthrough(TSpan {
                    data: "....\nblah blah blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                source: TSpan {
                    data: "....\nblah blah blah",
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
                    data: "....",
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
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        tests::fixtures::{
            blocks::{TBlock, TRawDelimitedBlock},
            TSpan,
        },
        Parser, Span,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("////\n////"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(),
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: "////\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn title() {
        let mut parser = Parser::default();

        let mi = Block::parse(
            Span::new(".comment\n////\nline1  \nline2\n////"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 3,
                        col: 1,
                        offset: 14,
                    },
                    TSpan {
                        data: "line2",
                        line: 4,
                        col: 1,
                        offset: 22,
                    }
                ),
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: ".comment\n////\nline1  \nline2\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: Some(TSpan {
                    data: "comment",
                    line: 1,
                    col: 2,
                    offset: 1,
                },),
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.title().unwrap(),
            TSpan {
                data: "comment",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("////\nline1  \nline2\n////"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "line2",
                        line: 3,
                        col: 1,
                        offset: 13,
                    }
                ),
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: "////\nline1  \nline2\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("////\nline1  \n/////\nline2\n////"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "/////",
                        line: 3,
                        col: 1,
                        offset: 13,
                    },
                    TSpan {
                        data: "line2",
                        line: 4,
                        col: 1,
                        offset: 19,
                    }
                ),
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: "////\nline1  \n/////\nline2\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
    }
}

mod listing {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        span::HasSpan,
        tests::fixtures::{
            blocks::{TBlock, TRawDelimitedBlock},
            TSpan,
        },
        Parser, Span,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("----\n----"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(),
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.raw_context().as_ref(), "listing");
        assert_eq!(mi.item.resolved_context().as_ref(), "listing");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("----\nline1  \nline2\n----"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "line2",
                        line: 3,
                        col: 1,
                        offset: 13,
                    }
                ),
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\nline1  \nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.raw_context().as_ref(), "listing");
        assert_eq!(mi.item.resolved_context().as_ref(), "listing");
        assert!(mi.item.declared_style().is_none());
        assert_eq!(mi.item.nested_blocks().next(), None);
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "----\nline1  \nline2\n----",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn title() {
        let mut parser = Parser::default();

        let mi = Block::parse(
            Span::new(".listing title\n----\nline1  \nline2\n----"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 3,
                        col: 1,
                        offset: 20,
                    },
                    TSpan {
                        data: "line2",
                        line: 4,
                        col: 1,
                        offset: 28,
                    }
                ),
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: ".listing title\n----\nline1  \nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: Some(TSpan {
                    data: "listing title",
                    line: 1,
                    col: 2,
                    offset: 1,
                },),
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.raw_context().as_ref(), "listing");
        assert_eq!(mi.item.resolved_context().as_ref(), "listing");
        assert!(mi.item.declared_style().is_none());
        assert_eq!(mi.item.nested_blocks().next(), None);
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.title().unwrap(),
            TSpan {
                data: "listing title",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ".listing title\n----\nline1  \nline2\n----",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("----\nline1  \n----/\nline2\n----"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "----/",
                        line: 3,
                        col: 1,
                        offset: 13,
                    },
                    TSpan {
                        data: "line2",
                        line: 4,
                        col: 1,
                        offset: 19,
                    }
                ),
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\nline1  \n----/\nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.raw_context().as_ref(), "listing");
        assert_eq!(mi.item.resolved_context().as_ref(), "listing");
        assert!(mi.item.declared_style().is_none());
        assert_eq!(mi.item.nested_blocks().next(), None);
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "----\nline1  \n----/\nline2\n----",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod pass {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        span::HasSpan,
        tests::fixtures::{
            blocks::{TBlock, TRawDelimitedBlock},
            TSpan,
        },
        Parser, Span,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("++++\n++++"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(),
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert_eq!(mi.item.nested_blocks().next(), None);
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "++++\n++++",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("++++\nline1  \nline2\n++++"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "line2",
                        line: 3,
                        col: 1,
                        offset: 13,
                    }
                ),
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\nline1  \nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert_eq!(mi.item.nested_blocks().next(), None);
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "++++\nline1  \nline2\n++++",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn title() {
        let mut parser = Parser::default();

        let mi = Block::parse(
            Span::new(".pass title\n++++\nline1  \nline2\n++++"),
            &mut parser,
        )
        .unwrap_if_no_warnings()
        .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 3,
                        col: 1,
                        offset: 17,
                    },
                    TSpan {
                        data: "line2",
                        line: 4,
                        col: 1,
                        offset: 25,
                    }
                ),
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: ".pass title\n++++\nline1  \nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: Some(TSpan {
                    data: "pass title",
                    line: 1,
                    col: 2,
                    offset: 1,
                },),
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert_eq!(mi.item.nested_blocks().next(), None);
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());

        assert_eq!(
            mi.item.title().unwrap(),
            TSpan {
                data: "pass title",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: ".pass title\n++++\nline1  \nline2\n++++",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("++++\nline1  \n++++/\nline2\n++++"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::RawDelimited(TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "++++/",
                        line: 3,
                        col: 1,
                        offset: 13,
                    },
                    TSpan {
                        data: "line2",
                        line: 4,
                        col: 1,
                        offset: 19,
                    }
                ),
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\nline1  \n++++/\nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert_eq!(mi.item.nested_blocks().next(), None);
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());

        assert_eq!(
            mi.item.span(),
            TSpan {
                data: "++++\nline1  \n++++/\nline2\n++++",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
