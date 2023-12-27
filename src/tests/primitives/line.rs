mod fn_line {
    use crate::{primitives::line, Span};

    #[test]
    fn empty_source() {
        let (rem, line) = line(Span::new("", true)).unwrap();

        assert_eq!(rem, Span::new("", true));
        assert_eq!(line, Span::new("", true));
    }

    #[test]
    fn simple_line() {
        let (rem, line) = line(Span::new("abc", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 4);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn trailing_space() {
        let (rem, line) = line(Span::new("abc ", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 5);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc ");
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let (rem, line) = line(Span::new("abc\ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let (rem, line) = line(Span::new("abc\r\ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let (rem, line) = line(Span::new("abc\n\rdef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "\rdef");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // Shouldn't terminate line at \r without \n.

        let (rem, line) = line(Span::new("abc\rdef", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 8);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc\rdef");
    }
}

mod normalized_line {
    use crate::{primitives::normalized_line, Span};

    #[test]
    fn empty_source() {
        let (rem, line) = normalized_line(Span::new("", true)).unwrap();

        assert_eq!(rem, Span::new("", true));
        assert_eq!(line, Span::new("", true));
    }

    #[test]
    fn simple_line() {
        let (rem, line) = normalized_line(Span::new("abc", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 4);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn discards_trailing_space() {
        let (rem, line) = normalized_line(Span::new("abc ", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 5);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn discards_multiple_trailing_spaces() {
        let (rem, line) = normalized_line(Span::new("abc   ", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 7);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let (rem, line) = normalized_line(Span::new("abc  \ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let (rem, line) = normalized_line(Span::new("abc  \r\ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let (rem, line) = normalized_line(Span::new("abc  \n\rdef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "\rdef");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // Shouldn't terminate normalized line at \r without \n.

        let (rem, line) = normalized_line(Span::new("abc   \rdef", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 11);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc   \rdef");
    }
}

mod non_empty_line {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };

    use crate::{primitives::non_empty_line, Span};

    #[test]
    fn empty_source() {
        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("", true), ErrorKind::TakeTill1));

        let actual_err = non_empty_line(Span::new("", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn only_spaces() {
        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("   ", true), ErrorKind::TakeTill1));

        let actual_err = non_empty_line(Span::new("   ", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn simple_line() {
        let (rem, line) = non_empty_line(Span::new("abc", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 4);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn discards_trailing_space() {
        let (rem, line) = non_empty_line(Span::new("abc ", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 5);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let (rem, line) = non_empty_line(Span::new("abc  \ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let (rem, line) = non_empty_line(Span::new("abc  \r\ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let (rem, line) = non_empty_line(Span::new("abc  \n\rdef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "\rdef");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // Shouldn't terminate line at \r without \n.

        let (rem, line) = non_empty_line(Span::new("abc   \rdef", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 11);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc   \rdef");
    }
}

mod empty_line {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };

    use crate::{primitives::empty_line, Span};

    #[test]
    fn empty_source() {
        let (rem, line) = empty_line(Span::new("", true)).unwrap();

        assert_eq!(rem, Span::new("", true));
        assert_eq!(line, Span::new("", true));
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

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 6);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "     ");
    }

    #[test]
    fn consumes_spaces_and_tabs() {
        // Should consume a source containing only spaces.

        let (rem, line) = empty_line(Span::new("  \t  ", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 6);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "  \t  ");
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let (rem, line) = empty_line(Span::new("   \ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "   ");
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let (rem, line) = empty_line(Span::new("   \r\ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "   ");
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let (rem, line) = empty_line(Span::new("   \n\rdef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "\rdef");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "   ");
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
    use crate::{primitives::consume_empty_lines, Span};

    #[test]
    fn empty_source() {
        let rem = consume_empty_lines(Span::new("", true));
        assert_eq!(rem, Span::new("", true));
    }

    #[test]
    fn consumes_empty_line() {
        let rem = consume_empty_lines(Span::new("\nabc", true));

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "abc");
    }

    #[test]
    fn doesnt_consume_non_empty_line() {
        let rem = consume_empty_lines(Span::new("abc", true));
        assert_eq!(rem, Span::new("abc", true));
    }

    #[test]
    fn doesnt_consume_leading_space() {
        let rem = consume_empty_lines(Span::new("   abc", true));
        assert_eq!(rem, Span::new("   abc", true));
    }

    #[test]
    fn consumes_line_with_only_spaces() {
        let rem = consume_empty_lines(Span::new("   \nabc", true));

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "abc");
    }

    #[test]
    fn consumes_spaces_and_tabs() {
        let rem = consume_empty_lines(Span::new(" \t \nabc", true));

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "abc");
    }

    #[test]
    fn consumes_multiple_lines() {
        let rem = consume_empty_lines(Span::new("\n  \t \n\nabc", true));

        assert_eq!(rem.line(), 4);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "abc");
    }

    #[test]
    fn consumes_crlf() {
        // Should consume \r\n sequence.

        let rem = consume_empty_lines(Span::new("  \r\ndef", true));

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.

        let rem = consume_empty_lines(Span::new("   \n\rdef", true));

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "\rdef");
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // A "line" with \r and no immediate \n is not considered empty.

        let rem = consume_empty_lines(Span::new("   \rdef", true));
        assert_eq!(rem, Span::new("   \rdef", true));
    }
}
