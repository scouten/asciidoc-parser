mod take_while {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_input() {
        let span = Span::new("");
        let pr = span.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn immediate_false() {
        let s = Span::new(":abc");
        let pr = s.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn match_after_first() {
        let s = Span::new("ab:cd");
        let pr = s.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "ab",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: ":cd",
                line: 1,
                col: 3,
                offset: 2,
            }
        );
    }

    #[test]
    fn non_empty_no_match() {
        let s = Span::new("abcd");
        let pr = s.take_while(|c| c != ':');

        assert_eq!(
            pr.t,
            TSpan {
                data: "abcd",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            pr.rem,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4,
            }
        );
    }
}
