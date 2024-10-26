mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::Block,
        tests::fixtures::{
            blocks::{TBlock, TSimpleBlock},
            inlines::TInline,
            warnings::TWarning,
            TSpan,
        },
        warnings::WarningType,
        Span,
    };

    #[test]
    fn err_invalid_delimiter() {
        assert!(Block::parse(Span::new(""))
            .unwrap_if_no_warnings()
            .is_none());

        let mi = Block::parse(Span::new("..."))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "...",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        let mi = Block::parse(Span::new("++++x"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "++++x",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        let mi = Block::parse(Span::new("____x"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "____x",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );

        let mi = Block::parse(Span::new("====x"))
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                data: "====x",
                line: 1,
                col: 1,
                offset: 0,
            })))
        );
    }

    #[test]
    fn err_unterminated() {
        let maw = Block::parse(Span::new("....\nblah blah blah"));

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock(TInline::Sequence(
                vec!(
                    TInline::Uninterpreted(TSpan {
                        data: "....",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }),
                    TInline::Uninterpreted(TSpan {
                        data: "blah blah blah",
                        line: 2,
                        col: 1,
                        offset: 5,
                    })
                ),
                TSpan {
                    data: "....\nblah blah blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            )))
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
        Span,
    };

    #[test]
    fn empty() {
        let mi = Block::parse(Span::new("////\n////"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "comment");
    }

    #[test]
    fn multiple_lines() {
        let mi = Block::parse(Span::new("////\nline1  \nline2\n////"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "comment");
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mi = Block::parse(Span::new("////\nline1  \n/////\nline2\n////"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "comment");
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
        Span,
    };

    #[test]
    fn empty() {
        let mi = Block::parse(Span::new("----\n----"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.context().as_ref(), "listing");
    }

    #[test]
    fn multiple_lines() {
        let mi = Block::parse(Span::new("----\nline1  \nline2\n----"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.context().as_ref(), "listing");
        assert_eq!(mi.item.nested_blocks().next(), None);

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
    fn ignores_delimiter_prefix() {
        let mi = Block::parse(Span::new("----\nline1  \n----/\nline2\n----"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.context().as_ref(), "listing");
        assert_eq!(mi.item.nested_blocks().next(), None);

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
        Span,
    };

    #[test]
    fn empty() {
        let mi = Block::parse(Span::new("++++\n++++"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "pass");
        assert_eq!(mi.item.nested_blocks().next(), None);

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
        let mi = Block::parse(Span::new("++++\nline1  \nline2\n++++"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "pass");
        assert_eq!(mi.item.nested_blocks().next(), None);

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
    fn ignores_delimiter_prefix() {
        let mi = Block::parse(Span::new("++++\nline1  \n++++/\nline2\n++++"))
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
                }
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "pass");
        assert_eq!(mi.item.nested_blocks().next(), None);

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
