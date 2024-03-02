mod line;

mod trim_input_for_rem {
    use nom::Slice;
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::trim_input_for_rem, tests::fixtures::TSpan, Span};

    fn advanced_span(source: &'static str, skip: usize) -> Span<'static> {
        let span = Span::new(source, true);
        span.slice(skip..)
    }

    #[test]
    fn empty_spans() {
        let inp = advanced_span("abcdef", 6);
        let rem = Span::new("", true);

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_equals_inp() {
        let inp = advanced_span("abcdef", 6);
        let rem = Span::new("abcdef", true);

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_too_long() {
        // This is nonsense input, but we should at least not panic in this case.

        let inp = advanced_span("abcdef", 6);
        let rem = Span::new("abcdef_bogus_bogus", true);

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_is_subset_of_inp() {
        let inp = advanced_span("abcdef", 2);
        let rem = advanced_span("abcdef", 4);

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
                data: "cd",
                line: 1,
                col: 3,
                offset: 2
            }
        );
    }
}

mod quoted_string {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::quoted_string, tests::fixtures::TSpan, Span};

    #[test]
    fn err_empty_source() {
        let expected_err: Err<Error<nom_span::Spanned<&str>>> =
            Err::Error(Error::new(Span::new("", true), ErrorKind::Char));

        let actual_err = quoted_string(Span::new("", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn err_unterminated_double_quote() {
        let err = quoted_string(Span::new("\"xxx", true)).unwrap_err();

        let Err::Error(e) = err else {
            panic!("Expected Err::Error: {err:#?}");
        };

        assert_eq!(e.code, ErrorKind::Char);

        assert_eq!(
            e.input,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn double_quoted_string() {
        let (rem, qstr) = quoted_string(Span::new("\"abc\"def", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn double_quoted_with_escape() {
        let (rem, qstr) = quoted_string(Span::new("\"a\\\"bc\"def", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "a\\\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn double_quoted_with_single_quote() {
        let (rem, qstr) = quoted_string(Span::new("\"a'bc\"def", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "a'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn err_unterminated_single_quote() {
        let err = quoted_string(Span::new("'xxx", true)).unwrap_err();

        let Err::Error(e) = err else {
            panic!("Expected Err::Error: {err:#?}");
        };

        assert_eq!(e.code, ErrorKind::Char);

        assert_eq!(
            e.input,
            TSpan {
                data: "\'xxx",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn single_quoted_string() {
        let (rem, qstr) = quoted_string(Span::new("'abc'def", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn single_quoted_with_escape() {
        let (rem, qstr) = quoted_string(Span::new("'a\\'bc'def", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "a\\'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn single_quoted_with_double_quote() {
        let (rem, qstr) = quoted_string(Span::new("'a\"bc'def", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "a\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }
}
