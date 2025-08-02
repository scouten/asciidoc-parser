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

        assert!(
            Block::parse(Span::new(""), &mut parser)
                .unwrap_if_no_warnings()
                .is_none()
        );

        let mut parser = Parser::default();

        let mi = Block::parse(Span::new("..."), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "...",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "&#8230;&#8203;",
                },
                source: TSpan {
                    data: "...",
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

        let mi = Block::parse(Span::new("++++x"), &mut parser)
            .unwrap_if_no_warnings()
            .unwrap();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "++++x",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "x",
                },
                source: TSpan {
                    data: "++++x",
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
    }

    #[test]
    fn err_unterminated() {
        let mut parser = Parser::default();
        let maw = Block::parse(Span::new("....\nblah blah blah"), &mut parser);

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: "....\nblah blah blah",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "&#8230;&#8203;.\nblah blah blah",
                },
                source: TSpan {
                    data: "....\nblah blah blah",
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
        Parser, Span,
        blocks::{Block, ContentModel, IsBlock},
        content::SubstitutionGroup,
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TRawDelimitedBlock},
            content::TContent,
        },
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
                content: TContent {
                    original: TSpan {
                        data: "",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "",
                },
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: "////\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::None);
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
                content: TContent {
                    original: TSpan {
                        data: "line1  \nline2",
                        line: 3,
                        col: 1,
                        offset: 14,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: ".comment\n////\nline1  \nline2\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: Some(TSpan {
                    data: "comment",
                    line: 1,
                    col: 2,
                    offset: 1,
                },),
                title: Some("comment"),
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::None);

        assert_eq!(
            mi.item.title_source().unwrap(),
            TSpan {
                data: "comment",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(mi.item.title_source().unwrap().data(), "comment");
        assert_eq!(mi.item.title().unwrap(), "comment");

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
                content: TContent {
                    original: TSpan {
                        data: "line1  \nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: "////\nline1  \nline2\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::None);
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
                content: TContent {
                    original: TSpan {
                        data: "line1  \n/////\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n/////\nline2",
                },
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: "////\nline1  \n/////\nline2\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::None,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::None);
    }
}

mod listing {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser, Span,
        blocks::{Block, ContentModel, IsBlock},
        content::SubstitutionGroup,
        span::HasSpan,
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TRawDelimitedBlock},
            content::TContent,
        },
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
                content: TContent {
                    original: TSpan {
                        data: "",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Verbatim,
            })
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.raw_context().as_ref(), "listing");
        assert_eq!(mi.item.resolved_context().as_ref(), "listing");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Verbatim);
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
                content: TContent {
                    original: TSpan {
                        data: "line1  \nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\nline1  \nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Verbatim,
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Verbatim);

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
                content: TContent {
                    original: TSpan {
                        data: "line1  \nline2",
                        line: 3,
                        col: 1,
                        offset: 20,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: ".listing title\n----\nline1  \nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: Some(TSpan {
                    data: "listing title",
                    line: 1,
                    col: 2,
                    offset: 1,
                },),
                title: Some("listing title"),
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Verbatim,
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
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Verbatim);

        assert_eq!(
            mi.item.title_source().unwrap(),
            TSpan {
                data: "listing title",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(mi.item.title_source().unwrap().data(), "listing title");
        assert_eq!(mi.item.title().unwrap(), "listing title");

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
                content: TContent {
                    original: TSpan {
                        data: "line1  \n----/\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n----/\nline2",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\nline1  \n----/\nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Verbatim,
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Verbatim);

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
        Parser, Span,
        blocks::{Block, ContentModel, IsBlock},
        content::SubstitutionGroup,
        span::HasSpan,
        tests::fixtures::{
            TSpan,
            blocks::{TBlock, TRawDelimitedBlock},
            content::TContent,
        },
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
                content: TContent {
                    original: TSpan {
                        data: "",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Pass,
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Pass);

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
                content: TContent {
                    original: TSpan {
                        data: "line1  \nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\nline1  \nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Pass,
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Pass);

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
                content: TContent {
                    original: TSpan {
                        data: "line1  \nline2",
                        line: 3,
                        col: 1,
                        offset: 17,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: ".pass title\n++++\nline1  \nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: Some(TSpan {
                    data: "pass title",
                    line: 1,
                    col: 2,
                    offset: 1,
                },),
                title: Some("pass title"),
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Pass,
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
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Pass);

        assert_eq!(
            mi.item.title_source().unwrap(),
            TSpan {
                data: "pass title",
                line: 1,
                col: 2,
                offset: 1,
            }
        );

        assert_eq!(mi.item.title_source().unwrap().data(), "pass title");
        assert_eq!(mi.item.title().unwrap(), "pass title");

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
                content: TContent {
                    original: TSpan {
                        data: "line1  \n++++/\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n++++/\nline2",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\nline1  \n++++/\nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Pass,
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Pass);

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
