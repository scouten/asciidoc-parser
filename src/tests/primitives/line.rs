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
