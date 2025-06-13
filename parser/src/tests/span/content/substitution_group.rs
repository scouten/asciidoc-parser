mod normal {
    use crate::{span::content::SubstitutionGroup, strings::CowStr, Content, Parser, Span};

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(Span::new("bl<ah>"));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;ah&gt;".to_string().into_boxed_str())
        );
    }

    #[test]
    fn match_amp() {
        let mut content = Content::from(Span::new("bl<a&h>"));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;a&amp;h&gt;".to_string().into_boxed_str())
        );
    }

    #[test]
    fn strong_word() {
        let mut content = Content::from(Span::new("One *word* is strong."));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed(
                "One <strong>word</strong> is strong."
                    .to_string()
                    .into_boxed_str()
            )
        );
    }

    #[test]
    fn strong_word_with_special_chars() {
        let mut content = Content::from(Span::new("One *wo<r>d* is strong."));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed(
                "One <strong>wo&lt;r&gt;d</strong> is strong."
                    .to_string()
                    .into_boxed_str()
            )
        );
    }

    #[test]
    fn marked_string_with_id() {
        let mut content = Content::from(Span::new(r#"[#id]#a few words#"#));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed(r#"<span id="id">a few words</span>"#.to_string().into_boxed_str())
        );
    }
}

mod header {
    use crate::{span::content::SubstitutionGroup, strings::CowStr, Content, Parser, Span};

    #[test]
    #[should_panic]
    fn not_yet_implemented() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }
}
