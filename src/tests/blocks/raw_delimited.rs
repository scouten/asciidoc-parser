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
    use crate::{
        blocks::RawDelimitedBlock,
        tests::fixtures::{warnings::TWarning, TSpan},
        warnings::WarningType,
        Span,
    };

    #[test]
    fn err_invalid_delimiter() {
        assert!(RawDelimitedBlock::parse(Span::new("")).is_none());
        assert!(RawDelimitedBlock::parse(Span::new("...")).is_none());
        assert!(RawDelimitedBlock::parse(Span::new("____x")).is_none());
        assert!(RawDelimitedBlock::parse(Span::new("====x")).is_none());
        assert!(RawDelimitedBlock::parse(Span::new("==\n==")).is_none());
    }

    #[test]
    fn err_unterminated() {
        let maw = RawDelimitedBlock::parse(Span::new("....\nblah blah blah")).unwrap();

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
