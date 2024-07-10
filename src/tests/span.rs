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
