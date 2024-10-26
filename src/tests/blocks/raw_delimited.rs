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
    use crate::{blocks::RawDelimitedBlock, Span};

    #[test]
    fn err_invalid_delimiter() {
        assert!(RawDelimitedBlock::parse(Span::new("")).is_none());
        assert!(RawDelimitedBlock::parse(Span::new("...")).is_none());
        assert!(RawDelimitedBlock::parse(Span::new("====x")).is_none());
        assert!(RawDelimitedBlock::parse(Span::new("==\n==")).is_none());
    }
}
