mod is_valid_delimiter {
    use crate::{blocks::RawDelimitedBlock, Span};

    #[test]
    fn comment() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new("////")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new("/////")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "/////////"
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("///")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("//-/")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("////-")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "//////////x"
        )));
    }

    #[test]
    fn example() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("====")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("=====")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("===")));
    }

    #[test]
    fn listing() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new("----")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new("-----")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "---------"
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("---")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("--/-")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("----/")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "----------x"
        )));
    }

    #[test]
    fn literal() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new("....")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new(".....")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "........."
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("...")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("../.")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("..../")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "..........x"
        )));
    }

    #[test]
    fn sidebar() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("****")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("*****")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("***")));
    }

    #[test]
    fn table() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("|===")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new(",===")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new(":===")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("!===")));
    }

    #[test]
    fn pass() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new("++++")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new("+++++")));
        assert!(RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "+++++++++"
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("+++")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("++/+")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("++++/")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new(
            "++++++++++x"
        )));
    }

    #[test]
    fn quote() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("____")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("_____")));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&Span::new("___")));
    }
}

mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, RawDelimitedBlock},
        tests::fixtures::{warnings::TWarning, TSpan},
        warnings::WarningType,
        Parser,
    };

    #[test]
    fn err_invalid_delimiter() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new(""), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("..."), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("++++x"), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("____x"), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("====x"), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("==\n=="), &mut parser).is_none());
    }

    #[test]
    fn err_unterminated() {
        let mut parser = Parser::default();

        let maw =
            RawDelimitedBlock::parse(&Preamble::new("....\nblah blah blah"), &mut parser).unwrap();

        assert!(maw.item.is_none());

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
        blocks::{preamble::Preamble, ContentModel, IsBlock, RawDelimitedBlock},
        span::content::SubstitutionGroup,
        tests::fixtures::{blocks::TRawDelimitedBlock, content::TContent, TSpan},
        Parser,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = RawDelimitedBlock::parse(&Preamble::new("////\n////"), &mut parser).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None,
                anchor: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "comment");
        assert_eq!(mi.item.resolved_context().as_ref(), "comment");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.content().is_empty());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let maw =
            RawDelimitedBlock::parse(&Preamble::new("////\nline1  \nline2\n////"), &mut parser)
                .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None,
                anchor: None,
                attrlist: None,
            }
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
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.content(),
            TContent {
                original: TSpan {
                    data: "line1  \nline2",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "line1  \nline2",
            }
        );
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mut parser = Parser::default();

        let maw = RawDelimitedBlock::parse(
            &Preamble::new("////\nline1  \n/////\nline2\n////"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None,
                anchor: None,
                attrlist: None,
            }
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
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.content(),
            TContent {
                original: TSpan {
                    data: "line1  \n/////\nline2",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "line1  \n/////\nline2",
            }
        );
    }
}

mod example {
    use crate::{
        blocks::{preamble::Preamble, RawDelimitedBlock},
        Parser,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("====\n===="), &mut parser).is_none());
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        assert!(RawDelimitedBlock::parse(
            &Preamble::new("====\nline1  \nline2\n===="),
            &mut parser
        )
        .is_none());
    }
}

mod listing {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, ContentModel, IsBlock, RawDelimitedBlock},
        span::content::SubstitutionGroup,
        tests::fixtures::{blocks::TRawDelimitedBlock, content::TContent, TSpan},
        Parser,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = RawDelimitedBlock::parse(&Preamble::new("----\n----"), &mut parser).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None,
                anchor: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.raw_context().as_ref(), "listing");
        assert_eq!(mi.item.resolved_context().as_ref(), "listing");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.content().is_empty());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let maw =
            RawDelimitedBlock::parse(&Preamble::new("----\nline1  \nline2\n----"), &mut parser)
                .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None,
                anchor: None,
                attrlist: None,
            }
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
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.content(),
            TContent {
                original: TSpan {
                    data: "line1  \nline2",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "line1  \nline2",
            }
        );
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mut parser = Parser::default();

        let maw = RawDelimitedBlock::parse(
            &Preamble::new("----\nline1  \n-----\nline2\n----"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
                content: TContent {
                    original: TSpan {
                        data: "line1  \n-----\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n-----\nline2",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\nline1  \n-----\nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            }
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
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.content(),
            TContent {
                original: TSpan {
                    data: "line1  \n-----\nline2",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "line1  \n-----\nline2",
            }
        );

        assert_eq!(
            mi.item.content(),
            TContent {
                original: TSpan {
                    data: "line1  \n-----\nline2",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "line1  \n-----\nline2",
            }
        );
    }
}

mod sidebar {
    use crate::{
        blocks::{preamble::Preamble, RawDelimitedBlock},
        Parser,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("****\n****"), &mut parser).is_none());
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(
            &Preamble::new("****\nline1  \nline2\n****"),
            &mut parser
        )
        .is_none());
    }
}

mod table {
    use crate::{
        blocks::{preamble::Preamble, RawDelimitedBlock},
        Parser,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("|===\n|==="), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new(",===\n,==="), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new(":===\n:==="), &mut parser).is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("!===\n!==="), &mut parser).is_none());
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(
            &Preamble::new("|===\nline1  \nline2\n|==="),
            &mut parser
        )
        .is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(
            &Preamble::new(",===\nline1  \nline2\n,==="),
            &mut parser
        )
        .is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(
            &Preamble::new(":===\nline1  \nline2\n:==="),
            &mut parser
        )
        .is_none());

        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(
            &Preamble::new("!===\nline1  \nline2\n!==="),
            &mut parser
        )
        .is_none());
    }
}

mod pass {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, ContentModel, IsBlock, RawDelimitedBlock},
        span::content::SubstitutionGroup,
        tests::fixtures::{blocks::TRawDelimitedBlock, content::TContent, TSpan},
        Parser,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw = RawDelimitedBlock::parse(&Preamble::new("++++\n++++"), &mut parser).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None,
                anchor: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.content().is_empty());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let maw =
            RawDelimitedBlock::parse(&Preamble::new("++++\nline1  \nline2\n++++"), &mut parser)
                .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None,
                anchor: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.content(),
            TContent {
                original: TSpan {
                    data: "line1  \nline2",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "line1  \nline2",
            }
        );
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mut parser = Parser::default();

        let maw = RawDelimitedBlock::parse(
            &Preamble::new("++++\nline1  \n+++++\nline2\n++++"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
                content: TContent {
                    original: TSpan {
                        data: "line1  \n+++++\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n+++++\nline2",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\nline1  \n+++++\nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.item.content(),
            TContent {
                original: TSpan {
                    data: "line1  \n+++++\nline2",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                rendered: "line1  \n+++++\nline2",
            }
        );
    }
}

mod quote {
    use crate::{
        blocks::{preamble::Preamble, RawDelimitedBlock},
        Parser,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(&Preamble::new("____\n____"), &mut parser).is_none());
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        assert!(RawDelimitedBlock::parse(
            &Preamble::new("____\nline1  \nline2\n____"),
            &mut parser
        )
        .is_none());
    }
}
