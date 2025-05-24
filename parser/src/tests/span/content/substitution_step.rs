mod special_characters {
    use crate::{span::content::SubstitutionStep, strings::CowStr, Content, Parser, Span};

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionStep::SpecialCharacters.apply(&mut content, &p);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        let p = Parser::default();
        SubstitutionStep::SpecialCharacters.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(Span::new("bl<ah>"));
        let p = Parser::default();
        SubstitutionStep::SpecialCharacters.apply(&mut content, &p);
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
        SubstitutionStep::SpecialCharacters.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;a&amp;h&gt;".to_string().into_boxed_str())
        );
    }
}

mod quotes {
    use crate::{span::content::SubstitutionStep, strings::CowStr, Content, Parser, Span};

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionStep::Quotes.apply(&mut content, &p);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        let p = Parser::default();
        SubstitutionStep::Quotes.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn ignore_lt_and_gt() {
        let mut content = Content::from(Span::new("bl<ah>"));
        let p = Parser::default();
        SubstitutionStep::Quotes.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl<ah>".to_string().into_boxed_str())
        );
    }

    #[test]
    fn strong_word() {
        let mut content = Content::from(Span::new("One *word* is strong."));
        let p = Parser::default();
        SubstitutionStep::Quotes.apply(&mut content, &p);
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
    fn marked_string_with_id() {
        let mut content = Content::from(Span::new(r#"[#id]#a few words#"#));
        let p = Parser::default();
        SubstitutionStep::Quotes.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed(r#"<span id="id">a few words</span>"#.to_string().into_boxed_str())
        );
    }
}

mod attribute_references {
    use crate::{span::content::SubstitutionStep, strings::CowStr, Content, Parser, Span};

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionStep::AttributeReferences.apply(&mut content, &p);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        let p = Parser::default();
        SubstitutionStep::AttributeReferences.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn ignore_non_match() {
        let mut content = Content::from(Span::new("bl{ah}"));
        let p = Parser::default();
        SubstitutionStep::AttributeReferences.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl{ah}".to_string().into_boxed_str())
        );
    }

    #[test]
    fn ignore_escaped_non_match() {
        let mut content = Content::from(Span::new("bl\\{ah}"));
        let p = Parser::default();
        SubstitutionStep::AttributeReferences.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl\\{ah}".to_string().into_boxed_str())
        );
    }

    #[test]
    fn replace_sp_match() {
        let mut content = Content::from(Span::new("bl{sp}ah"));
        let p = Parser::default();
        SubstitutionStep::AttributeReferences.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl ah".to_string().into_boxed_str())
        );
    }

    #[test]
    fn ignore_escaped_sp_match() {
        let mut content = Content::from(Span::new("bl\\{sp}ah"));
        let p = Parser::default();
        SubstitutionStep::AttributeReferences.apply(&mut content, &p);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl{sp}ah".to_string().into_boxed_str())
        );
    }
}

mod callouts {
    use crate::{span::content::SubstitutionStep, strings::CowStr, Content, Parser, Span};

    #[test]
    #[should_panic]
    fn not_yet_implemented() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionStep::Callouts.apply(&mut content, &p);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }
}
