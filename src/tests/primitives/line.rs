mod fn_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::line, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let l = line(Span::new("", true));

        assert_eq!(
            l.rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            l.t,
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
        let l = line(Span::new("abc", true));

        assert_eq!(
            l.rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            l.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn trailing_space() {
        let l = line(Span::new("abc ", true));

        assert_eq!(
            l.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            l.t,
            TSpan {
                data: "abc ",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let l = line(Span::new("abc\ndef", true));

        assert_eq!(
            l.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            l.t,
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

        let l = line(Span::new("abc\r\ndef", true));

        assert_eq!(
            l.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 5
            }
        );

        assert_eq!(
            l.t,
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

        let l = line(Span::new("abc\n\rdef", true));

        assert_eq!(
            l.rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            l.t,
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

        let l = line(Span::new("abc\rdef", true));

        assert_eq!(
            l.rem,
            TSpan {
                data: "",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            l.t,
            TSpan {
                data: "abc\rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod normalized_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::normalized_line, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let (rem, line) = normalized_line(Span::new("", true));

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = normalized_line(Span::new("abc", true));

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = normalized_line(Span::new("abc ", true));

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = normalized_line(Span::new("abc   ", true));

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = normalized_line(Span::new("abc  \ndef", true));

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = normalized_line(Span::new("abc  \r\ndef", true));

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = normalized_line(Span::new("abc  \n\rdef", true));

        assert_eq!(
            rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = normalized_line(Span::new("abc   \rdef", true));

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line,
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
        assert!(non_empty_line(Span::new("", true)).is_none());
    }

    #[test]
    fn only_spaces() {
        assert!(non_empty_line(Span::new("   ", true)).is_none());
    }

    #[test]
    fn simple_line() {
        let (rem, line) = non_empty_line(Span::new("abc", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = non_empty_line(Span::new("abc ", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = non_empty_line(Span::new("abc  \ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = non_empty_line(Span::new("abc  \r\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = non_empty_line(Span::new("abc  \n\rdef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = non_empty_line(Span::new("abc   \rdef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line,
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
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::line_with_continuation, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("", true), ErrorKind::TakeTill1));

        let actual_err = line_with_continuation(Span::new("", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn only_spaces() {
        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("   ", true), ErrorKind::TakeTill1));

        let actual_err = line_with_continuation(Span::new("   ", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn simple_line() {
        let (rem, line) = line_with_continuation(Span::new("abc", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = line_with_continuation(Span::new("abc ", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = line_with_continuation(Span::new("abc  \ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = line_with_continuation(Span::new("abc  \r\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = line_with_continuation(Span::new("abc  \n\rdef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line,
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

        let (rem, line) = line_with_continuation(Span::new("abc   \rdef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = line_with_continuation(Span::new("abc \\\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 9
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = line_with_continuation(Span::new("abc \\\r\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 10
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = line_with_continuation(Span::new("abc \\   \ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 12
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = line_with_continuation(Span::new("abc \\\ndef\\\nghi", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 3,
                col: 4,
                offset: 14
            }
        );

        assert_eq!(
            line,
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
        let (rem, line) = line_with_continuation(Span::new("abc \\\ndef  \nghi", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "ghi",
                line: 3,
                col: 1,
                offset: 12
            }
        );

        assert_eq!(
            line,
            TSpan {
                data: "abc \\\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod empty_line {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::empty_line, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let (rem, line) = empty_line(Span::new("", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            line,
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
        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("abc", true), ErrorKind::NonEmpty));

        let actual_err = empty_line(Span::new("abc", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn leading_space() {
        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("  abc", true), ErrorKind::NonEmpty));

        let actual_err = empty_line(Span::new("  abc", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn consumes_spaces() {
        // Should consume a source containing only spaces.

        let (rem, line) = empty_line(Span::new("     ", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            line,
            TSpan {
                data: "     ",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_spaces_and_tabs() {
        // Should consume a source containing only spaces.

        let (rem, line) = empty_line(Span::new("  \t  ", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            line,
            TSpan {
                data: "  \t  ",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let (rem, line) = empty_line(Span::new("   \ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            line,
            TSpan {
                data: "   ",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let (rem, line) = empty_line(Span::new("   \r\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 5
            }
        );

        assert_eq!(
            line,
            TSpan {
                data: "   ",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let (rem, line) = empty_line(Span::new("   \n\rdef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            line,
            TSpan {
                data: "   ",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // A "line" with \r and no immediate \n is not considered empty.

        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("   \rdef", true), ErrorKind::NonEmpty));

        let actual_err = empty_line(Span::new("   \rdef", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }
}

mod consume_empty_lines {
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::consume_empty_lines, tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let rem = consume_empty_lines(Span::new("", true));
        assert_eq!(rem, Span::new("", true));
    }

    #[test]
    fn consumes_empty_line() {
        let rem = consume_empty_lines(Span::new("\nabc", true));

        assert_eq!(
            rem,
            TSpan {
                data: "abc",
                line: 2,
                col: 1,
                offset: 1
            }
        );
    }

    #[test]
    fn doesnt_consume_non_empty_line() {
        let rem = consume_empty_lines(Span::new("abc", true));

        assert_eq!(
            rem,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn doesnt_consume_leading_space() {
        let rem = consume_empty_lines(Span::new("   abc", true));

        assert_eq!(
            rem,
            TSpan {
                data: "   abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_line_with_only_spaces() {
        let rem = consume_empty_lines(Span::new("   \nabc", true));

        assert_eq!(
            rem,
            TSpan {
                data: "abc",
                line: 2,
                col: 1,
                offset: 4
            }
        );
    }

    #[test]
    fn consumes_spaces_and_tabs() {
        let rem = consume_empty_lines(Span::new(" \t \nabc", true));

        assert_eq!(
            rem,
            TSpan {
                data: "abc",
                line: 2,
                col: 1,
                offset: 4
            }
        );
    }

    #[test]
    fn consumes_multiple_lines() {
        let rem = consume_empty_lines(Span::new("\n  \t \n\nabc", true));

        assert_eq!(
            rem,
            TSpan {
                data: "abc",
                line: 4,
                col: 1,
                offset: 7
            }
        );
    }

    #[test]
    fn consumes_crlf() {
        // Should consume \r\n sequence.

        let rem = consume_empty_lines(Span::new("  \r\ndef", true));

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 4
            }
        );
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let rem = consume_empty_lines(Span::new("   \n\rdef", true));

        assert_eq!(
            rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 4
            }
        );
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // A "line" with \r and no immediate \n is not considered empty.

        let rem = consume_empty_lines(Span::new("   \rdef", true));

        assert_eq!(
            rem,
            TSpan {
                data: "   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}
