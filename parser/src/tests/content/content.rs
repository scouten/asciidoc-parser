mod is_empty {
    use crate::{Content, Span};

    #[test]
    fn basic_empty_span() {
        let content = Content::Basic(Span::new(""));
        assert!(content.is_empty());
    }

    #[test]
    fn basic_non_empty_span() {
        let content = Content::Basic(Span::new("blah"));
        assert!(!content.is_empty());
    }

    #[test]
    fn other_type() {
        let content = Content::NamedCharacterAmp(Span::new("&amp;"));
        assert!(!content.is_empty());
    }
}
