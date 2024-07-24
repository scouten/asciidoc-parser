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
    let pr = s.into_parse_result(1);

    assert_eq!(pr.t.data(), "a");
    assert_eq!(pr.t.line(), 1);
    assert_eq!(pr.t.col(), 1);
    assert_eq!(pr.t.byte_offset(), 0);

    assert_eq!(pr.rem.data(), "bc");
    assert_eq!(pr.rem.line(), 1);
    assert_eq!(pr.rem.col(), 2);
    assert_eq!(pr.rem.byte_offset(), 1);
}

mod split_at_match_non_empty {
    use crate::Span;

    #[test]
    fn empty_input() {
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
        let pr = s.split_at_match_non_empty(|c| c == ':').unwrap();

        assert_eq!(pr.t.data(), "ab");
        assert_eq!(pr.t.line(), 1);
        assert_eq!(pr.t.col(), 1);
        assert_eq!(pr.t.byte_offset(), 0);

        assert_eq!(pr.rem.data(), ":cd");
        assert_eq!(pr.rem.line(), 1);
        assert_eq!(pr.rem.col(), 3);
        assert_eq!(pr.rem.byte_offset(), 2);
    }
}

mod line;
mod nom_traits;
mod parse_result;
mod primitives;
mod split;
mod take;
