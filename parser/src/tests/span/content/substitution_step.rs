mod special_characters {
    use crate::{span::content::SubstitutionStep, strings::CowStr, Content, Span};

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        SubstitutionStep::SpecialCharacters.apply(&mut content);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        SubstitutionStep::SpecialCharacters.apply(&mut content);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(Span::new("bl<ah>"));
        SubstitutionStep::SpecialCharacters.apply(&mut content);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;ah&gt;".to_string().into_boxed_str())
        );
    }
}

mod quotes {
    use crate::{span::content::SubstitutionStep, strings::CowStr, Content, Span};

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        SubstitutionStep::Quotes.apply(&mut content);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        SubstitutionStep::Quotes.apply(&mut content);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn ignore_lt_and_gt() {
        let mut content = Content::from(Span::new("bl<ah>"));
        SubstitutionStep::Quotes.apply(&mut content);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl<ah>".to_string().into_boxed_str())
        );
    }

    #[test]
    fn strong_word() {
        let mut content = Content::from(Span::new("One *word* is strong."));
        SubstitutionStep::Quotes.apply(&mut content);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("One <em>word</em> is strong.".to_string().into_boxed_str())
        );
    }
}

mod callouts {
    use crate::{span::content::SubstitutionStep, strings::CowStr, Content, Span};

    #[test]
    #[should_panic]
    fn not_yet_implemented() {
        let mut content = Content::from(Span::new(""));
        SubstitutionStep::Callouts.apply(&mut content);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }
}
