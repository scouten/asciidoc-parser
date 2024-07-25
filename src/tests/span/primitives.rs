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
