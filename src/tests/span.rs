use crate::Span;

#[test]
fn test_happy_case() {
    let span = Span::new(r#"{"hello": "world 🙌"}"#);

    assert_eq!(span.line(), 1);
    assert_eq!(span.col(), 1);
    assert_eq!(span.byte_offset(), 0);
}

#[test]
fn impl_as_ref() {
    let span = Span::new("abcdef");
    assert_eq!(span.as_ref(), "abcdef");
}

#[test]
fn impl_nom_as_bytes() {
    use nom::AsBytes;

    let span = Span::new("abcdef");
    assert_eq!(span.as_bytes(), b"abcdef");
}

mod impl_nom_compare_no_case {
    use nom::{Compare, CompareResult};

    use crate::Span;

    #[test]
    fn eq_different_case() {
        let span = Span::new("BCD");
        assert_eq!(span.compare_no_case("bcd"), CompareResult::Ok);
    }

    #[test]
    fn eq_smae_case() {
        let span = Span::new("BCD");
        assert_eq!(span.compare_no_case("BCD"), CompareResult::Ok);
    }

    #[test]
    fn error_not_same() {
        let span = Span::new("BCD");
        assert_eq!(span.compare_no_case("ABCE"), CompareResult::Error);
    }
}

mod impl_nom_input_iter {
    use nom::InputIter;

    use crate::Span;

    #[test]
    fn iter_indices() {
        let span = Span::new("abc");
        let mut i = span.iter_indices();
        assert_eq!(i.next(), Some((0, 'a')));
        assert_eq!(i.next(), Some((1, 'b')));
        assert_eq!(i.next(), Some((2, 'c')));
        assert_eq!(i.next(), None);
    }

    mod position {
        use nom::InputIter;

        use crate::Span;

        #[test]
        fn success() {
            let span = Span::new("abc");
            assert_eq!(span.position(|c| c == 'c'), Some(2));
        }

        #[test]
        fn no_match() {
            let span = Span::new("abc");
            assert_eq!(span.position(|c| c == 'x'), None);
        }
    }
}

mod impl_nom_input_take {
    use nom::InputTake;

    use crate::Span;

    #[test]
    fn take() {
        let span = Span::new("abc").take(2);
        assert_eq!(span.data(), "ab");
        assert_eq!(span.line(), 1);
        assert_eq!(span.col(), 1);
        assert_eq!(span.byte_offset(), 0);
    }
}
