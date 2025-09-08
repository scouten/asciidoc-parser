mod take_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        let l = span.take_line();

        assert_eq!(
            l.after,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            l.item,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_line() {
        let span = crate::Span::new("abc");
        let l = span.take_line();

        assert_eq!(
            l.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            l.item,
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn trailing_space() {
        let span = crate::Span::new("abc ");
        let l = span.take_line();

        assert_eq!(
            l.after,
            Span {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            l.item,
            Span {
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

        let span = crate::Span::new("abc\ndef");
        let l = span.take_line();

        assert_eq!(
            l.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            l.item,
            Span {
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

        let span = crate::Span::new("abc\r\ndef");
        let l = span.take_line();

        assert_eq!(
            l.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 5
            }
        );

        assert_eq!(
            l.item,
            Span {
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

        let span = crate::Span::new("abc\n\rdef");
        let l = span.take_line();

        assert_eq!(
            l.after,
            Span {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            l.item,
            Span {
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

        let span = crate::Span::new("abc\rdef");
        let l = span.take_line();

        assert_eq!(
            l.after,
            Span {
                data: "",
                line: 1,
                col: 8,
                offset: 7
            }
        );

        assert_eq!(
            l.item,
            Span {
                data: "abc\rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod take_normalized_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_line() {
        let span = crate::Span::new("abc");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_trailing_space() {
        let span = crate::Span::new("abc ");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_multiple_trailing_spaces() {
        let span = crate::Span::new("abc   ");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \ndef");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \r\ndef");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \n\rdef");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc   \rdef");
        let line = span.take_normalized_line();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod take_non_empty_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        assert!(span.take_non_empty_line().is_none());
    }

    #[test]
    fn only_spaces() {
        let span = crate::Span::new("   ");
        assert!(span.take_non_empty_line().is_none());
    }

    #[test]
    fn simple_line() {
        let span = crate::Span::new("abc");
        let line = span.take_non_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_trailing_space() {
        let span = crate::Span::new("abc ");
        let line = span.take_non_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \ndef");
        let line = span.take_non_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \r\ndef");
        let line = span.take_non_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \n\rdef");
        let line = span.take_non_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc   \rdef");
        let line = span.take_non_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod take_empty_line {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_line() {
        let span = crate::Span::new("abc");
        assert!(span.take_empty_line().is_none());
    }

    #[test]
    fn leading_space() {
        let span = crate::Span::new("  abc");
        assert!(span.take_empty_line().is_none());
    }

    #[test]
    fn consumes_spaces() {
        // Should consume a source containing only spaces.

        let span = crate::Span::new("     ");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("  \t  ");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 6,
                offset: 5
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("   \ndef");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("   \r\ndef");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 5
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("   \n\rdef");
        let line = span.take_empty_line().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 4
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("   \rdef");
        assert!(span.take_empty_line().is_none());
    }
}

mod discard_empty_lines {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        let after = span.discard_empty_lines();
        assert_eq!(after, crate::Span::new("",));
    }

    #[test]
    fn consumes_empty_line() {
        let span = crate::Span::new("\nabc");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
                data: "abc",
                line: 2,
                col: 1,
                offset: 1
            }
        );
    }

    #[test]
    fn doesnt_consume_non_empty_line() {
        let span = crate::Span::new("abc");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn doesnt_consume_leading_space() {
        let span = crate::Span::new("   abc");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
                data: "   abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn consumes_line_with_only_spaces() {
        let span = crate::Span::new("   \nabc");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
                data: "abc",
                line: 2,
                col: 1,
                offset: 4
            }
        );
    }

    #[test]
    fn consumes_spaces_and_tabs() {
        let span = crate::Span::new(" \t \nabc");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
                data: "abc",
                line: 2,
                col: 1,
                offset: 4
            }
        );
    }

    #[test]
    fn consumes_multiple_lines() {
        let span = crate::Span::new("\n  \t \n\nabc");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
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

        let span = crate::Span::new("  \r\ndef");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
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

        let span = crate::Span::new("   \n\rdef");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
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

        let span = crate::Span::new("   \rdef");
        let after = span.discard_empty_lines();

        assert_eq!(
            after,
            Span {
                data: "   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}

mod take_line_with_continuation {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        assert!(span.take_line_with_continuation().is_none());
    }

    #[test]
    fn only_spaces() {
        let span = crate::Span::new("   ");
        assert!(span.take_line_with_continuation().is_none());
    }

    #[test]
    fn simple_line() {
        let span = crate::Span::new("abc");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn discards_trailing_space() {
        let span = crate::Span::new("abc ");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \ndef");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \r\ndef");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "def",
                line: 2,
                col: 1,
                offset: 7
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc  \n\rdef");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "\rdef",
                line: 2,
                col: 1,
                offset: 6
            }
        );

        assert_eq!(
            line.item,
            Span {
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

        let span = crate::Span::new("abc   \rdef");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 1,
                col: 11,
                offset: 10
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc   \rdef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_continuation() {
        let span = crate::Span::new("abc \\\ndef");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 2,
                col: 4,
                offset: 9
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc \\\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn simple_continuation_with_crlf() {
        let span = crate::Span::new("abc \\\r\ndef");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 2,
                col: 4,
                offset: 10
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc \\\r\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn continuation_with_trailing_space() {
        let span = crate::Span::new("abc \\   \ndef");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 2,
                col: 4,
                offset: 12
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc \\   \ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn multiple_continuations() {
        let span = crate::Span::new("abc \\\ndef\\\nghi");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "",
                line: 3,
                col: 4,
                offset: 14
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc \\\ndef\\\nghi",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn terminates_on_line_without_trailing_slash() {
        let span = crate::Span::new("abc \\\ndef  \nghi");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "ghi",
                line: 3,
                col: 1,
                offset: 12
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc \\\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }

    #[test]
    fn doesnt_consume_empty_line() {
        let span = crate::Span::new("abc \\\ndef\n\nghi");
        let line = span.take_line_with_continuation().unwrap();

        assert_eq!(
            line.after,
            Span {
                data: "\nghi",
                line: 3,
                col: 1,
                offset: 10
            }
        );

        assert_eq!(
            line.item,
            Span {
                data: "abc \\\ndef",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}
