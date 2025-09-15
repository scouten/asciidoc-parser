mod trim_trailing_whitespace {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let s = crate::Span::default().trim_trailing_whitespace();

        assert_eq!(
            s,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn nothing_to_trim() {
        let s = crate::Span::new("foo").trim_trailing_whitespace();

        assert_eq!(
            s,
            Span {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn space_in_middle() {
        let s = crate::Span::new("foo bar").trim_trailing_whitespace();

        assert_eq!(
            s,
            Span {
                data: "foo bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn trailing_space() {
        let s = crate::Span::new("foo ").trim_trailing_whitespace();

        assert_eq!(
            s,
            Span {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn trailing_newlines() {
        let s = crate::Span::new("foo\n\n").trim_trailing_whitespace();

        assert_eq!(
            s,
            Span {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod trim_trailing_line_end {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let s = crate::Span::default().trim_trailing_line_end();

        assert_eq!(
            s,
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn nothing_to_trim() {
        let s = crate::Span::new("foo").trim_trailing_line_end();

        assert_eq!(
            s,
            Span {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn space_in_middle() {
        let s = crate::Span::new("foo bar").trim_trailing_line_end();

        assert_eq!(
            s,
            Span {
                data: "foo bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn trailing_space() {
        let s = crate::Span::new("foo ").trim_trailing_line_end();

        assert_eq!(
            s,
            Span {
                data: "foo ",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn trailing_newlines() {
        let s = crate::Span::new("foo\n\n").trim_trailing_line_end();

        assert_eq!(
            s,
            Span {
                data: "foo\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn trailing_windows_newlines() {
        let s = crate::Span::new("foo\n\r\n").trim_trailing_line_end();

        assert_eq!(
            s,
            Span {
                data: "foo\n",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
