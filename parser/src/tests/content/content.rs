mod is_empty {
    #[test]
    fn basic_empty_span() {
        let content = crate::content::Content::from(crate::Span::default());
        assert!(content.is_empty());
    }

    #[test]
    fn basic_non_empty_span() {
        let content = crate::content::Content::from(crate::Span::new("blah"));
        assert!(!content.is_empty());
    }
}
