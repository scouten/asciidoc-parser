mod take_prefix {
    use crate::{Span, tests::fixtures::TSpan};

    #[test]
    fn empty_source() {
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
        let mi = s.take_prefix("ab").unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "ab",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_prefix("ab:cd").unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "ab:cd",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
    use crate::{Span, tests::fixtures::TSpan};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        let mi = span.take_whitespace();

        assert_eq!(
            mi.item,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_whitespace();

        assert_eq!(
            mi.item,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_whitespace();

        assert_eq!(
            mi.item,
            TSpan {
                data: " \t",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_whitespace();

        assert_eq!(
            mi.item,
            TSpan {
                data: "  \t ",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
    use crate::{Span, tests::fixtures::TSpan};

    #[test]
    fn empty_source() {
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
        let mi = s.take_required_whitespace().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: " \t",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_required_whitespace().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "  \t ",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
    use crate::{Span, tests::fixtures::TSpan};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        let mi = span.take_while(|c| c != ':');

        assert_eq!(
            mi.item,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_while(|c| c != ':');

        assert_eq!(
            mi.item,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_while(|c| c != ':');

        assert_eq!(
            mi.item,
            TSpan {
                data: "ab",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
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
        let mi = s.take_while(|c| c != ':');

        assert_eq!(
            mi.item,
            TSpan {
                data: "abcd",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4,
            }
        );
    }
}

mod take_non_empty_lines {
    use crate::{Span, tests::fixtures::TSpan};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        assert!(span.take_non_empty_lines().is_none());
    }

    #[test]
    fn immediate_false() {
        let span = Span::new("\nabc");
        assert!(span.take_non_empty_lines().is_none());
    }

    #[test]
    fn match_after_first() {
        let span = Span::new("abc\n\ndef");
        let mi = span.take_non_empty_lines().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "\ndef",
                line: 2,
                col: 1,
                offset: 4,
            }
        );
    }

    #[test]
    fn several_lines() {
        let span = Span::new("abc\ndef\nline3\nline4\n\ndef");
        let mi = span.take_non_empty_lines().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "abc\ndef\nline3\nline4",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "\ndef",
                line: 5,
                col: 1,
                offset: 20,
            }
        );
    }
}
