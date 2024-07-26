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
        let pr = span.take_attr_name().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "x",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_attr_name().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_attr_name().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "blah-blah-94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_attr_name().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "xyz",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_quoted_string().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_quoted_string().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "a\\\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_quoted_string().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "a'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_quoted_string().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_quoted_string().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "a\\'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            pr.rem,
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
        let pr = span.take_quoted_string().unwrap();

        assert_eq!(
            pr.t,
            TSpan {
                data: "a\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }
}
