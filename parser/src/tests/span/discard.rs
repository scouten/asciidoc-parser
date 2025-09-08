mod discard {
    use crate::tests::fixtures::Span;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        assert_eq!(
            span.discard(4),
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn n_gt_len() {
        let span = crate::Span::new("abc");
        assert_eq!(
            span.discard(6),
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }

    #[test]
    fn n_eq_len() {
        let span = crate::Span::new("abc");
        assert_eq!(
            span.discard(3),
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }

    #[test]
    fn n_lt_len() {
        let span = crate::Span::new("abc");
        assert_eq!(
            span.discard(2),
            Span {
                data: "c",
                line: 1,
                col: 3,
                offset: 2,
            }
        );
    }

    #[test]
    fn zero() {
        let span = crate::Span::new("abc");
        assert_eq!(
            span.discard(0),
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod discard_whitespace {
    use crate::tests::fixtures::Span;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        assert_eq!(
            span.discard_whitespace(),
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn all_whitespace() {
        let span = crate::Span::new("   \t  ");
        assert_eq!(
            span.discard_whitespace(),
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6,
            }
        );
    }

    #[test]
    fn doesnt_consume_newline() {
        let span = crate::Span::new("   \nabc");
        assert_eq!(
            span.discard_whitespace(),
            Span {
                data: "\nabc",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }

    #[test]
    fn some_whitespace() {
        let span = crate::Span::new("   abc");
        assert_eq!(
            span.discard_whitespace(),
            Span {
                data: "abc",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }

    #[test]
    fn no_whitespace() {
        let span = crate::Span::new("abc");
        assert_eq!(
            span.discard_whitespace(),
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}

mod discard_all {
    use crate::tests::fixtures::Span;

    #[test]
    fn empty_source() {
        let span = crate::Span::new("");
        assert_eq!(
            span.discard_all(),
            Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn non_empty() {
        let span = crate::Span::new("abc");
        assert_eq!(
            span.discard(3),
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }
}
