mod take_prefix {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_input() {
        let span = Span::new("");
        assert!(span.take_prefix("foo").is_none());
    }

    #[test]
    fn mismatch() {
        let span = Span::new(":abc");
        assert!(span.take_prefix("abc").is_none());
    }

    #[test]
    fn partial_match() {
        let span = Span::new("abc");
        assert!(span.take_prefix("abcd").is_none());
    }

    #[test]
    fn match_with_remainder() {
        let s = Span::new("ab:cd");
        let pr = s.take_prefix("ab").unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "ab",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":cd",
                line: 1,
                col: 3,
                offset: 2,
            }
        );
    }

    #[test]
    fn exact_match() {
        let s = Span::new("ab:cd");
        let pr = s.take_prefix("ab:cd").unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "ab:cd",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 6,
                offset: 5,
            }
        );
    }
}

mod take_whitespace {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_input() {
        let span = Span::new("");
        let pr = span.take_whitespace();

        assert_eq!(
            pr.t,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn immediate_false() {
        let s = Span::new(":abc");
        let pr = s.take_whitespace();

        assert_eq!(
            pr.t,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn match_after_first() {
        let s = Span::new(" \t:cd");
        let pr = s.take_whitespace();

        assert_eq!(
            pr.t,
            TSpan {
                data: " \t",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":cd",
                line: 1,
                col: 3,
                offset: 2,
            }
        );
    }

    #[test]
    fn all_whitespace() {
        let s = Span::new("  \t ");
        let pr = s.take_whitespace();

        assert_eq!(
            pr.t,
            TSpan {
                data: "  \t ",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4,
            }
        );
    }
}

mod take_required_whitespace {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_input() {
        let span = Span::new("");
        assert!(span.take_required_whitespace().is_none());
    }

    #[test]
    fn starts_with_non_whitespace() {
        let s = Span::new(":abc");
        assert!(s.take_required_whitespace().is_none());
    }

    #[test]
    fn match_after_first() {
        let s = Span::new(" \t:cd");
        let pr = s.take_required_whitespace().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: " \t",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":cd",
                line: 1,
                col: 3,
                offset: 2,
            }
        );
    }

    #[test]
    fn all_whitespace() {
        let s = Span::new("  \t ");
        let pr = s.take_required_whitespace().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "  \t ",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4,
            }
        );
    }
}

mod take_while {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_input() {
        let span = Span::new("");
        let pr = span.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn immediate_false() {
        let s = Span::new(":abc");
        let pr = s.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn match_after_first() {
        let s = Span::new("ab:cd");
        let pr = s.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "ab",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":cd",
                line: 1,
                col: 3,
                offset: 2,
            }
        );
    }

    #[test]
    fn non_empty_no_match() {
        let s = Span::new("abcd");
        let pr = s.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "abcd",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4,
            }
        );
    }
}
