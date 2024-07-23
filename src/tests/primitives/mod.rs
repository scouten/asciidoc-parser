mod trim_input_for_rem {
    use nom::Slice;
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::trim_input_for_rem, tests::fixtures::TSpan, Span};

    fn advanced_span(source: &'static str, skip: usize) -> Span<'static> {
        let span = Span::new(source);
        span.slice(skip..)
    }

    #[test]
    fn empty_spans() {
        let inp = advanced_span("abcdef", 6);
        let rem = Span::new("");

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
        let rem = Span::new("abcdef");

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
        let rem = Span::new("abcdef_bogus_bogus");

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

mod attr_name {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::attr_name, tests::fixtures::TSpan, Span};

    #[test]
    fn err_empty_source() {
        let expected_err: Err<Error<Span>> =
            Err::Error(Error::new(Span::new(""), ErrorKind::AlphaNumeric));

        let actual_err = attr_name(Span::new("")).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn err_starts_with_non_word() {
        let expected_err: Err<Error<Span>> = Err::Error(Error::new(
            Span::new("#not-a-proper-name"),
            ErrorKind::AlphaNumeric,
        ));

        let actual_err = attr_name(Span::new("#not-a-proper-name")).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn err_starts_with_hyphen() {
        let expected_err: Err<Error<Span>> = Err::Error(Error::new(
            Span::new("-not-a-proper-name"),
            ErrorKind::AlphaNumeric,
        ));

        let actual_err = attr_name(Span::new("-not-a-proper-name")).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn stops_at_non_ident() {
        let (rem, qstr) = attr_name(Span::new("x#")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "#",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "x",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn numeric() {
        let (rem, qstr) = attr_name(Span::new("94!")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "!",
                line: 1,
                col: 3,
                offset: 2
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "94",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn contains_hyphens() {
        let (rem, qstr) = attr_name(Span::new("blah-blah-94 = foo")).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: " = foo",
                line: 1,
                col: 13,
                offset: 12
            }
        );

        assert_eq!(
            qstr,
            TSpan {
                data: "blah-blah-94",
                line: 1,
                col: 1,
                offset: 0
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
        let expected_err: Err<Error<Span>> = Err::Error(Error::new(Span::new(""), ErrorKind::Char));

        let actual_err = quoted_string(Span::new("")).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn err_unterminated_double_quote() {
        let err = quoted_string(Span::new("\"xxx")).unwrap_err();

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
        let (rem, qstr) = quoted_string(Span::new("\"abc\"def")).unwrap();

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
        let (rem, qstr) = quoted_string(Span::new("\"a\\\"bc\"def")).unwrap();

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
        let (rem, qstr) = quoted_string(Span::new("\"a'bc\"def")).unwrap();

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
        let err = quoted_string(Span::new("'xxx")).unwrap_err();

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
        let (rem, qstr) = quoted_string(Span::new("'abc'def")).unwrap();

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
        let (rem, qstr) = quoted_string(Span::new("'a\\'bc'def")).unwrap();

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
        let (rem, qstr) = quoted_string(Span::new("'a\"bc'def")).unwrap();

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
