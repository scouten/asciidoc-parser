mod simple {
    // use nom::{
    //     error::{Error, ErrorKind},
    //     Err,
    // };
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        inlines::Inline,
        tests::fixtures::{inlines::TInline, TSpan},
        HasSpan, Span,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let (_, b1) = Inline::parse(Span::new("abc", true)).unwrap();
        let b2 = b1.clone();
        assert_eq!(b1, b2);
    }

    #[test]
    fn empty_source() {
        let (rem, inline) = Inline::parse(Span::new("", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            inline,
            TInline::Uninterpreted(TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            })
        );

        assert_eq!(
            inline.span(),
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn only_spaces() {
        let (rem, inline) = Inline::parse(Span::new("   ", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            inline,
            TInline::Uninterpreted(TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            })
        );

        assert_eq!(
            inline.span(),
            TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }

    #[test]
    fn simple_line() {
        let (rem, inline) = Inline::parse(Span::new("abc", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            inline,
            TInline::Uninterpreted(TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            })
        );

        assert_eq!(
            inline.span(),
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );
    }
}
