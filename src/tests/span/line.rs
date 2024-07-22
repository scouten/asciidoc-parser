mod take_empty_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        let line = span.take_empty_line().unwrap();

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
        let span = Span::new("abc");
        assert!(span.take_empty_line().is_none());
    }

    #[test]
    fn leading_space() {
        let span = Span::new("  abc");
        assert!(span.take_empty_line().is_none());
    }

    #[test]
    fn consumes_spaces() {
        // Should consume a source containing only spaces.

        let span = Span::new("     ");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            line.t,
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
        // Should consume a source containing spaces and tabs.

        let span = Span::new("  \t  ");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            line.t,
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

        let span = Span::new("   \ndef");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            line.t,
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

        let span = Span::new("   \r\ndef");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 5
            }
        );

        assert_eq!(
            line.t,
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

        let span = Span::new("   \n\rdef");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.rem,
            TSpan {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            line.t,
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

        let span = Span::new("   \rdef");
        assert!(span.take_empty_line().is_none());
    }
}

mod discard_empty_lines {
    use pretty_assertions_sorted::assert_eq;

    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let span = Span::new("");
        let rem = span.discard_empty_lines();
        assert_eq!(rem, Span::new("",));
    }

    #[test]
    fn consumes_empty_line() {
        let span = Span::new("\nabc");
        let rem = span.discard_empty_lines();

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
        let span = Span::new("abc");
        let rem = span.discard_empty_lines();

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
        let span = Span::new("   abc");
        let rem = span.discard_empty_lines();

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
        let span = Span::new("   \nabc");
        let rem = span.discard_empty_lines();

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
        let span = Span::new(" \t \nabc");
        let rem = span.discard_empty_lines();

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
        let span = Span::new("\n  \t \n\nabc");
        let rem = span.discard_empty_lines();

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

        let span = Span::new("  \r\ndef");
        let rem = span.discard_empty_lines();

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

        let span = Span::new("   \n\rdef");
        let rem = span.discard_empty_lines();

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

        let span = Span::new("   \rdef");
        let rem = span.discard_empty_lines();

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
