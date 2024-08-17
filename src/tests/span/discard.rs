mod discard {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_input() {
        let span = Span::new("");
        assert_eq!(
            span.discard(4),
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn n_gt_len() {
        let span = Span::new("abc");
        assert_eq!(
            span.discard(6),
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }

    #[test]
    fn n_eq_len() {
        let span = Span::new("abc");
        assert_eq!(
            span.discard(3),
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }

    #[test]
    fn n_lt_len() {
        let span = Span::new("abc");
        assert_eq!(
            span.discard(2),
            TSpan {
                data: "c",
                line: 1,
                col: 3,
                offset: 2,
            }
        );
    }

    #[test]
    fn zero() {
        let span = Span::new("abc");
        assert_eq!(
            span.discard(0),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod discard_all {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn empty_input() {
        let span = Span::new("");
        assert_eq!(
            span.discard_all(),
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn non_empty() {
        let span = Span::new("abc");
        assert_eq!(
            span.discard(3),
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }
}
