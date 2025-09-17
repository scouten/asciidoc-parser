mod is_valid_delimiter {
    use crate::blocks::RawDelimitedBlock;

    #[test]
    fn comment() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "////"
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "/////"
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "/////////"
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "///"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "//-/"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "////-"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "//////////x"
        )));
    }

    #[test]
    fn example() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "===="
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "====="
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "==="
        )));
    }

    #[test]
    fn listing() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "----"
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "-----"
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "---------"
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "---"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "--/-"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "----/"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "----------x"
        )));
    }

    #[test]
    fn literal() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "...."
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "....."
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "........."
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "..."
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "../."
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "..../"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "..........x"
        )));
    }

    #[test]
    fn sidebar() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "****"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "*****"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "***"
        )));
    }

    #[test]
    fn table() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "|==="
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            ",==="
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            ":==="
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "!==="
        )));
    }

    #[test]
    fn pass() {
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "++++"
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "+++++"
        )));
        assert!(RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "+++++++++"
        )));

        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "+++"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "++/+"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "++++/"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "++++++++++x"
        )));
    }

    #[test]
    fn quote() {
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "____"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "_____"
        )));
        assert!(!RawDelimitedBlock::is_valid_delimiter(&crate::Span::new(
            "___"
        )));
    }
}

mod parse {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser, blocks::metadata::BlockMetadata, tests::prelude::*, warnings::WarningType,
    };

    #[test]
    fn err_invalid_delimiter() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new(""), &mut parser).is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("..."), &mut parser)
                .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("++++x"), &mut parser)
                .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("____x"), &mut parser)
                .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("====x"), &mut parser)
                .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("==\n=="), &mut parser)
                .is_none()
        );
    }

    #[test]
    fn err_unterminated() {
        let mut parser = Parser::default();

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("....\nblah blah blah"),
            &mut parser,
        )
        .unwrap();

        assert_eq!(
            maw.warnings,
            vec![Warning {
                source: Span {
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
        Parser,
        blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
        content::SubstitutionGroup,
        tests::prelude::*,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw =
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("////\n////"), &mut parser)
                .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "",
                },
                content_model: ContentModel::Raw,
                context: "comment",
                source: Span {
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::None);
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("////\nline1  \nline2\n////"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "line1  \nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Raw,
                context: "comment",
                source: Span {
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
            }
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

        assert_eq!(
            mi.item.content(),
            Content {
                original: Span {
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

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("////\nline1  \n/////\nline2\n////"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "line1  \n/////\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n/////\nline2",
                },
                content_model: ContentModel::Raw,
                context: "comment",
                source: Span {
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
            }
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

        assert_eq!(
            mi.item.content(),
            Content {
                original: Span {
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
    use crate::{Parser, blocks::metadata::BlockMetadata};

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("====\n===="), &mut parser)
                .is_none()
        );
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        assert!(
            crate::blocks::RawDelimitedBlock::parse(
                &BlockMetadata::new("====\nline1  \nline2\n===="),
                &mut parser
            )
            .is_none()
        );
    }
}

mod listing {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
        content::{SubstitutionGroup, SubstitutionStep},
        tests::prelude::*,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw =
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("----\n----"), &mut parser)
                .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: Span {
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Verbatim);
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("----\nline1  \nline2\n----"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "line1  \nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: Span {
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
            }
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

        assert_eq!(
            mi.item.content(),
            Content {
                original: Span {
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
    fn overrides_sub_group_via_subs_attribute() {
        let mut parser = Parser::default();

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("[subs=quotes]\n----\nline1 < *line2*\n----"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "line1 < *line2*",
                        line: 3,
                        col: 1,
                        offset: 19,
                    },
                    rendered: "line1 < <strong>line2</strong>",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: Span {
                    data: "[subs=quotes]\n----\nline1 < *line2*\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: Some("subs"),
                        value: "quotes",
                        shorthand_items: &[],
                    },],
                    anchor: None,
                    source: Span {
                        data: "subs=quotes",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
                substitution_group: SubstitutionGroup::Custom(vec![SubstitutionStep::Quotes]),
            }
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

        assert_eq!(
            mi.item.attrlist().unwrap(),
            Attrlist {
                attributes: &[ElementAttribute {
                    name: Some("subs"),
                    value: "quotes",
                    shorthand_items: &[],
                },],
                anchor: None,
                source: Span {
                    data: "subs=quotes",
                    line: 1,
                    col: 2,
                    offset: 1,
                },
            }
        );

        assert_eq!(
            mi.item.substitution_group(),
            SubstitutionGroup::Custom(vec![SubstitutionStep::Quotes])
        );

        assert_eq!(
            mi.item.content(),
            Content {
                original: Span {
                    data: "line1 < *line2*",
                    line: 3,
                    col: 1,
                    offset: 19,
                },
                rendered: "line1 < <strong>line2</strong>",
            }
        );
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let mut parser = Parser::default();

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("----\nline1  \n-----\nline2\n----"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "line1  \n-----\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n-----\nline2",
                },
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: Span {
                    data: "----\nline1  \n-----\nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Verbatim,
            }
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

        assert_eq!(
            mi.item.content(),
            Content {
                original: Span {
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
            Content {
                original: Span {
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
    use crate::{Parser, blocks::metadata::BlockMetadata};

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("****\n****"), &mut parser)
                .is_none()
        );
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(
                &BlockMetadata::new("****\nline1  \nline2\n****"),
                &mut parser
            )
            .is_none()
        );
    }
}

mod table {
    use crate::{Parser, blocks::metadata::BlockMetadata};

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("|===\n|==="), &mut parser)
                .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new(",===\n,==="), &mut parser)
                .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new(":===\n:==="), &mut parser)
                .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("!===\n!==="), &mut parser)
                .is_none()
        );
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(
                &BlockMetadata::new("|===\nline1  \nline2\n|==="),
                &mut parser
            )
            .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(
                &BlockMetadata::new(",===\nline1  \nline2\n,==="),
                &mut parser
            )
            .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(
                &BlockMetadata::new(":===\nline1  \nline2\n:==="),
                &mut parser
            )
            .is_none()
        );

        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(
                &BlockMetadata::new("!===\nline1  \nline2\n!==="),
                &mut parser
            )
            .is_none()
        );
    }
}

mod pass {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
        content::SubstitutionGroup,
        tests::prelude::*,
    };

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        let maw =
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("++++\n++++"), &mut parser)
                .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: Span {
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
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Pass);
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("++++\nline1  \nline2\n++++"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "line1  \nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \nline2",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: Span {
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
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Pass);

        assert_eq!(
            mi.item.content(),
            Content {
                original: Span {
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

        let maw = crate::blocks::RawDelimitedBlock::parse(
            &BlockMetadata::new("++++\nline1  \n+++++\nline2\n++++"),
            &mut parser,
        )
        .unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            RawDelimitedBlock {
                content: Content {
                    original: Span {
                        data: "line1  \n+++++\nline2",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    rendered: "line1  \n+++++\nline2",
                },
                content_model: ContentModel::Raw,
                context: "pass",
                source: Span {
                    data: "++++\nline1  \n+++++\nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
                substitution_group: SubstitutionGroup::Pass,
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.raw_context().as_ref(), "pass");
        assert_eq!(mi.item.resolved_context().as_ref(), "pass");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Pass);

        assert_eq!(
            mi.item.content(),
            Content {
                original: Span {
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
    use crate::{Parser, blocks::metadata::BlockMetadata};

    #[test]
    fn empty() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(&BlockMetadata::new("____\n____"), &mut parser)
                .is_none()
        );
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::RawDelimitedBlock::parse(
                &BlockMetadata::new("____\nline1  \nline2\n____"),
                &mut parser
            )
            .is_none()
        );
    }
}
