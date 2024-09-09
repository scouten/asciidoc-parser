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
fn into_parse_result() {
    let s = Span::new("abc");
    let mi = s.into_parse_result(1);

    assert_eq!(mi.item.data(), "a");
    assert_eq!(mi.item.line(), 1);
    assert_eq!(mi.item.col(), 1);
    assert_eq!(mi.item.byte_offset(), 0);

    assert_eq!(mi.after.data(), "bc");
    assert_eq!(mi.after.line(), 1);
    assert_eq!(mi.after.col(), 2);
    assert_eq!(mi.after.byte_offset(), 1);
}

mod split_at_match_non_empty {
    use crate::Span;

    #[test]
    fn empty_source() {
        let s = Span::new("");
        assert!(s.split_at_match_non_empty(|c| c == ':').is_none());
    }

    #[test]
    fn empty_subspan() {
        let s = Span::new(":abc");
        assert!(s.split_at_match_non_empty(|c| c == ':').is_none());
    }

    #[test]
    fn match_after_first() {
        let s = Span::new("ab:cd");
        let mi = s.split_at_match_non_empty(|c| c == ':').unwrap();

        assert_eq!(mi.item.data(), "ab");
        assert_eq!(mi.item.line(), 1);
        assert_eq!(mi.item.col(), 1);
        assert_eq!(mi.item.byte_offset(), 0);

        assert_eq!(mi.after.data(), ":cd");
        assert_eq!(mi.after.line(), 1);
        assert_eq!(mi.after.col(), 3);
        assert_eq!(mi.after.byte_offset(), 2);
    }
}

mod discard;
mod line;
mod parse_result;
mod primitives;
mod split;
mod take;
