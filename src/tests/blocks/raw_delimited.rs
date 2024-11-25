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
    };

    #[test]
    fn err_invalid_delimiter() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new("...")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new("++++x")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new("____x")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new("====x")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new("==\n==")).is_none());
    }

    #[test]
    fn err_unterminated() {
        let maw = RawDelimitedBlock::parse(&Preamble::new("....\nblah blah blah")).unwrap();

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
        tests::fixtures::{blocks::TRawDelimitedBlock, TSpan},
    };

    #[test]
    fn empty() {
        let maw = RawDelimitedBlock::parse(&Preamble::new("////\n////")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
                lines: vec!(),
                content_model: ContentModel::Raw,
                context: "comment",
                source: TSpan {
                    data: "////\n////",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "comment");
        assert!(mi.item.lines().next().is_none());
    }

    #[test]
    fn multiple_lines() {
        let maw = RawDelimitedBlock::parse(&Preamble::new("////\nline1  \nline2\n////")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "comment");

        let mut lines = mi.item.lines();
        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line1",
                line: 2,
                col: 1,
                offset: 5,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line2",
                line: 3,
                col: 1,
                offset: 13,
            }
        );

        assert!(lines.next().is_none());
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let maw =
            RawDelimitedBlock::parse(&Preamble::new("////\nline1  \n/////\nline2\n////")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "comment");

        let mut lines = mi.item.lines();
        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line1",
                line: 2,
                col: 1,
                offset: 5,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "/////",
                line: 3,
                col: 1,
                offset: 13,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line2",
                line: 4,
                col: 1,
                offset: 19,
            }
        );

        assert!(lines.next().is_none());
    }
}

mod example {
    use crate::blocks::{preamble::Preamble, RawDelimitedBlock};

    #[test]
    fn empty() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("====\n====")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("====\nline1  \nline2\n====")).is_none());
    }
}

mod listing {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, ContentModel, IsBlock, RawDelimitedBlock},
        tests::fixtures::{blocks::TRawDelimitedBlock, TSpan},
    };

    #[test]
    fn empty() {
        let maw = RawDelimitedBlock::parse(&Preamble::new("----\n----")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
                lines: vec!(),
                content_model: ContentModel::Verbatim,
                context: "listing",
                source: TSpan {
                    data: "----\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.context().as_ref(), "listing");
        assert!(mi.item.lines().next().is_none());
    }

    #[test]
    fn multiple_lines() {
        let maw = RawDelimitedBlock::parse(&Preamble::new("----\nline1  \nline2\n----")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.context().as_ref(), "listing");

        let mut lines = mi.item.lines();
        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line1",
                line: 2,
                col: 1,
                offset: 5,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line2",
                line: 3,
                col: 1,
                offset: 13,
            }
        );

        assert!(lines.next().is_none());
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let maw =
            RawDelimitedBlock::parse(&Preamble::new("----\nline1  \n-----\nline2\n----")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "-----",
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
                    data: "----\nline1  \n-----\nline2\n----",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
        assert_eq!(mi.item.context().as_ref(), "listing");

        let mut lines = mi.item.lines();
        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line1",
                line: 2,
                col: 1,
                offset: 5,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "-----",
                line: 3,
                col: 1,
                offset: 13,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line2",
                line: 4,
                col: 1,
                offset: 19,
            }
        );

        assert!(lines.next().is_none());
    }
}

mod sidebar {
    use crate::blocks::{preamble::Preamble, RawDelimitedBlock};

    #[test]
    fn empty() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("****\n****")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("****\nline1  \nline2\n****")).is_none());
    }
}

mod table {
    use crate::blocks::{preamble::Preamble, RawDelimitedBlock};

    #[test]
    fn empty() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("|===\n|===")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new(",===\n,===")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new(":===\n:===")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new("!===\n!===")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("|===\nline1  \nline2\n|===")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new(",===\nline1  \nline2\n,===")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new(":===\nline1  \nline2\n:===")).is_none());
        assert!(RawDelimitedBlock::parse(&Preamble::new("!===\nline1  \nline2\n!===")).is_none());
    }
}

mod pass {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, ContentModel, IsBlock, RawDelimitedBlock},
        tests::fixtures::{blocks::TRawDelimitedBlock, TSpan},
    };

    #[test]
    fn empty() {
        let maw = RawDelimitedBlock::parse(&Preamble::new("++++\n++++")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
                lines: vec!(),
                content_model: ContentModel::Raw,
                context: "pass",
                source: TSpan {
                    data: "++++\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "pass");
        assert!(mi.item.lines().next().is_none());
    }

    #[test]
    fn multiple_lines() {
        let maw = RawDelimitedBlock::parse(&Preamble::new("++++\nline1  \nline2\n++++")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
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
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "pass");

        let mut lines = mi.item.lines();
        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line1",
                line: 2,
                col: 1,
                offset: 5,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line2",
                line: 3,
                col: 1,
                offset: 13,
            }
        );

        assert!(lines.next().is_none());
    }

    #[test]
    fn ignores_delimiter_prefix() {
        let maw =
            RawDelimitedBlock::parse(&Preamble::new("++++\nline1  \n+++++\nline2\n++++")).unwrap();

        let mi = maw.item.unwrap().clone();

        assert_eq!(
            mi.item,
            TRawDelimitedBlock {
                lines: vec!(
                    TSpan {
                        data: "line1",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                    TSpan {
                        data: "+++++",
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
                    data: "++++\nline1  \n+++++\nline2\n++++",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None
            }
        );

        assert_eq!(mi.item.content_model(), ContentModel::Raw);
        assert_eq!(mi.item.context().as_ref(), "pass");

        let mut lines = mi.item.lines();
        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line1",
                line: 2,
                col: 1,
                offset: 5,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "+++++",
                line: 3,
                col: 1,
                offset: 13,
            }
        );

        assert_eq!(
            lines.next().unwrap(),
            TSpan {
                data: "line2",
                line: 4,
                col: 1,
                offset: 19,
            }
        );

        assert!(lines.next().is_none());
    }
}

mod quote {
    use crate::blocks::{preamble::Preamble, RawDelimitedBlock};

    #[test]
    fn empty() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("____\n____")).is_none());
    }

    #[test]
    fn multiple_lines() {
        assert!(RawDelimitedBlock::parse(&Preamble::new("____\nline1  \nline2\n____")).is_none());
    }
}
