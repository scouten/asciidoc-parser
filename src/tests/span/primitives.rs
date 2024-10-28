mod take_ident {
    use pretty_assertions_sorted::assert_eq;

    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn starts_with_non_word() {
        let span = Span::new("#not-a-proper-name");
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn starts_with_hyphen() {
        let span = Span::new("-not-a-proper-name");
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn starts_with_number() {
        let span = Span::new("9not-a-proper-name");
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn stops_at_non_ident() {
        let span = Span::new("x#");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "x",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "#",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn alpha_numeric() {
        let span = Span::new("i94!");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "i94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "!",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }

    #[test]
    fn starts_with_underscore() {
        let span = Span::new("_i94!");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "_i94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "!",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn contains_underscores() {
        let span = Span::new("blah_blah_94 = foo");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "blah_blah_94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: " = foo",
                line: 1,
                col: 13,
                offset: 12
            }
        );
    }

    #[test]
    fn contains_hyphens() {
        let span = Span::new("blah-blah-94 = foo");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "blah",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "-blah-94 = foo",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn stops_at_eof() {
        let span = Span::new("xyz");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "xyz",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }
}

mod take_attr_name {
    use pretty_assertions_sorted::assert_eq;

    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        assert!(span.take_attr_name().is_none());
    }

    #[test]
    fn starts_with_non_word() {
        let span = Span::new("#not-a-proper-name");
        assert!(span.take_attr_name().is_none());
    }

    #[test]
    fn starts_with_hyphen() {
        let span = Span::new("-not-a-proper-name");
        assert!(span.take_attr_name().is_none());
    }

    #[test]
    fn stops_at_non_ident() {
        let span = Span::new("x#");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "x",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "#",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn numeric() {
        let span = Span::new("94!");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "!",
                line: 1,
                col: 3,
                offset: 2
            }
        );
    }

    #[test]
    fn contains_hyphens() {
        let span = Span::new("blah-blah-94 = foo");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "blah-blah-94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: " = foo",
                line: 1,
                col: 13,
                offset: 12
            }
        );
    }

    #[test]
    fn stops_at_eof() {
        let span = Span::new("xyz");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "xyz",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }
}

mod take_quoted_string {
    use pretty_assertions_sorted::assert_eq;

    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        assert!(span.take_quoted_string().is_none());
    }

    #[test]
    fn unterminated_double_quote() {
        let span = Span::new("\"xxx");
        assert!(span.take_quoted_string().is_none());
    }

    #[test]
    fn double_quoted_string() {
        let span = Span::new("\"abc\"def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );
    }

    #[test]
    fn double_quoted_with_escape() {
        let span = Span::new("\"a\\\"bc\"def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "a\\\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn double_quoted_with_single_quote() {
        let span = Span::new("\"a'bc\"def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "a'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn unterminated_single_quote() {
        let span = Span::new("'xxx");
        assert!(span.take_quoted_string().is_none());
    }

    #[test]
    fn single_quoted_string() {
        let span = Span::new("'abc'def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );
    }

    #[test]
    fn single_quoted_with_escape() {
        let span = Span::new("'a\\'bc'def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "a\\'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn single_quoted_with_double_quote() {
        let span = Span::new("'a\"bc'def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            TSpan {
                data: "a\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }
}

mod trim_remainder {
    use pretty_assertions_sorted::assert_eq;

    use crate::{tests::fixtures::TSpan, Span};

    fn advanced_span(source: &'static str, skip: usize) -> Span<'static> {
        let span = Span::new(source);
        span.slice_from(skip..)
    }

    #[test]
    fn empty_spans() {
        let source = advanced_span("abcdef", 6);
        let after = Span::new("");

        assert_eq!(
            source.trim_remainder(after),
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_equals_source() {
        let source = advanced_span("abcdef", 6);
        let after = Span::new("abcdef");

        assert_eq!(
            source.trim_remainder(after),
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

        let source = advanced_span("abcdef", 6);
        let after = Span::new("abcdef_bogus_bogus");

        assert_eq!(
            source.trim_remainder(after),
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_is_subset_of_source() {
        let source = advanced_span("abcdef", 2);
        let after = advanced_span("abcdef", 4);

        assert_eq!(
            source.trim_remainder(after),
            TSpan {
                data: "cd",
                line: 1,
                col: 3,
                offset: 2
            }
        );
    }

    #[test]
    fn rem_is_incomplete_subset_of_source() {
        let source = Span::new("abc\ndef\n");
        let line1 = source.take_normalized_line();
        let line2 = line1.after.take_line();

        assert_eq!(
            source.trim_remainder(line2.item),
            TSpan {
                data: "abc\n",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}
