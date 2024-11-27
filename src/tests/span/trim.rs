mod trim_trailing_whitespace {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_source() {
        let s = Span::new("").trim_trailing_whitespace();

        assert_eq!(
            s,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn nothing_to_trim() {
        let s = Span::new("foo").trim_trailing_whitespace();

        assert_eq!(
            s,
            TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn space_in_middle() {
        let s = Span::new("foo bar").trim_trailing_whitespace();

        assert_eq!(
            s,
            TSpan {
                data: "foo bar",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn trailing_space() {
        let s = Span::new("foo ").trim_trailing_whitespace();

        assert_eq!(
            s,
            TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn trailing_newlines() {
        let s = Span::new("foo\n\n").trim_trailing_whitespace();

        assert_eq!(
            s,
            TSpan {
                data: "foo",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}