mod normalized_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::normalized_line, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let line = normalized_line(Span::new(""));

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_line() {
        let line = normalized_line(Span::new("abc"));

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_trailing_space() {
        let line = normalized_line(Span::new("abc "));

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_multiple_trailing_spaces() {
        let line = normalized_line(Span::new("abc   "));

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let line = normalized_line(Span::new("abc  \ndef"));

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let line = normalized_line(Span::new("abc  \r\ndef"));

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let line = normalized_line(Span::new("abc  \n\rdef"));

        assert_eq!(
            line.rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // Shouldn't terminate normalized line at \r without \n.

        let line = normalized_line(Span::new("abc   \rdef"));

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod non_empty_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::non_empty_line, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        assert!(non_empty_line(Span::new("",)).is_none());
    }

    #[test]
    fn only_spaces() {
        assert!(non_empty_line(Span::new("   ",)).is_none());
    }

    #[test]
    fn simple_line() {
        let line = non_empty_line(Span::new("abc")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_trailing_space() {
        let line = non_empty_line(Span::new("abc ")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let line = non_empty_line(Span::new("abc  \ndef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let line = non_empty_line(Span::new("abc  \r\ndef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let line = non_empty_line(Span::new("abc  \n\rdef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // Shouldn't terminate line at \r without \n.

        let line = non_empty_line(Span::new("abc   \rdef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod line_with_continuation {
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::line_with_continuation, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        assert!(line_with_continuation(Span::new("",)).is_none());
    }

    #[test]
    fn only_spaces() {
        assert!(line_with_continuation(Span::new("   ",)).is_none());
    }

    #[test]
    fn simple_line() {
        let line = line_with_continuation(Span::new("abc")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_trailing_space() {
        let line = line_with_continuation(Span::new("abc ")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let line = line_with_continuation(Span::new("abc  \ndef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let line = line_with_continuation(Span::new("abc  \r\ndef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let line = line_with_continuation(Span::new("abc  \n\rdef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // Shouldn't terminate line at \r without \n.

        let line = line_with_continuation(Span::new("abc   \rdef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_continuation() {
        let line = line_with_continuation(Span::new("abc \\\ndef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 9
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc \\\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_continuation_with_crlf() {
        let line = line_with_continuation(Span::new("abc \\\r\ndef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 10
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc \\\r\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn continuation_with_trailing_space() {
        let line = line_with_continuation(Span::new("abc \\   \ndef")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 12
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc \\   \ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn multiple_continuations() {
        let line = line_with_continuation(Span::new("abc \\\ndef\\\nghi")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 3,
                col: 4,
                offset: 14
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc \\\ndef\\\nghi",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn terminates_on_line_without_plus() {
        let line = line_with_continuation(Span::new("abc \\\ndef  \nghi")).unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "ghi",
                line: 3,
                col: 1,
                offset: 12
            }
        );

        assert_eq!(
            line.t,
            TSpan {
                data: "abc \\\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}
