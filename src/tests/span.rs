use crate::Span;

#[test]
fn test_happy_case() {
    let span = Span::new(r#"{"hello": "world 🙌"}"#);

    assert_eq!(span.line(), 1);
    assert_eq!(span.col(), 1);
    assert_eq!(span.byte_offset(), 0);
}

#[test]
fn impl_as_bytes() {
    let span = Span::new("abcdef");
    assert_eq!(span.as_ref(), "abcdef");
}

#[test]
fn impl_as_ref() {
    let span = Span::new("abcdef");
    assert_eq!(span.as_bytes(), b"abcdef");
}
