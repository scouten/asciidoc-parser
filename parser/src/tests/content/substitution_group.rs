mod from_custom_string {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser, Span,
        content::{Content, SubstitutionGroup, SubstitutionStep},
        strings::CowStr,
    };

    #[test]
    fn empty() {
        assert_eq!(SubstitutionGroup::from_custom_string(""), None);
    }

    #[test]
    fn normal() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("n"),
            Some(SubstitutionGroup::Normal)
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("normal"),
            Some(SubstitutionGroup::Normal)
        );

        assert_eq!(SubstitutionGroup::from_custom_string("nermal"), None);
    }

    #[test]
    fn verbatim() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("v"),
            Some(SubstitutionGroup::Verbatim)
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("verbatim"),
            Some(SubstitutionGroup::Verbatim)
        );

        assert_eq!(SubstitutionGroup::from_custom_string("verboten"), None);
    }

    #[test]
    fn special_chars() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("c"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("specialchars"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters
            ]))
        );
    }

    #[test]
    fn quotes() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("q"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Quotes]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("quotes"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Quotes]))
        );
    }

    #[test]
    fn attributes() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("a"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("attributes"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::AttributeReferences
            ]))
        );
    }

    #[test]
    fn replacements() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("r"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::CharacterReplacements
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("replacements"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::CharacterReplacements
            ]))
        );
    }

    #[test]
    fn macros() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("m"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Macros]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("macros"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Macros]))
        );
    }

    #[test]
    fn post_replacements() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("p"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::PostReplacement
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("post replacements"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::PostReplacement
            ]))
        );
    }

    #[test]
    fn multiple() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("q,a"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("q, a"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("quotes,attributes"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("x,bogus,no such step"),
            None
        );
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("n,-r"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences,
                SubstitutionStep::Macros,
                SubstitutionStep::PostReplacement,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("n,-r,-r,-m"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences,
                SubstitutionStep::PostReplacement,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("v,-r"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("v,-c"),
            Some(SubstitutionGroup::Custom(vec![]))
        );
    }

    #[test]
    fn addition() {
        assert_eq!(
            SubstitutionGroup::from_custom_string("n,r"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences,
                SubstitutionStep::CharacterReplacements,
                SubstitutionStep::Macros,
                SubstitutionStep::PostReplacement,
                SubstitutionStep::CharacterReplacements,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string("v,m"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Macros,
            ]))
        );
    }

    #[test]
    fn custon_group_with_macros_preserves_passthroughs() {
        let custom_group = SubstitutionGroup::from_custom_string("q,m").unwrap();

        let mut content = Content::from(Span::new(
            "Text with +++pass<through>+++ icon:github[] content.",
        ));
        let p = Parser::default();
        custom_group.apply(&mut content, &p, None);

        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed(
                "Text with pass<through> <span class=\"icon\">[github&#93;</span> content."
                    .to_string()
                    .into_boxed_str()
            )
        );
    }
}

mod normal {
    use crate::{
        Parser, Span,
        content::{Content, SubstitutionGroup},
        strings::CowStr,
    };

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

mod attribute_entry_value {
    use crate::{
        Parser, Span,
        content::{Content, SubstitutionGroup},
        parser::ModificationContext,
        strings::CowStr,
    };

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionGroup::AttributeEntryValue.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        let p = Parser::default();
        SubstitutionGroup::AttributeEntryValue.apply(&mut content, &p, None);
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
        SubstitutionGroup::AttributeEntryValue.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;a&amp;h&gt;".to_string().into_boxed_str())
        );
    }

    #[test]
    fn ignores_strong_word() {
        let mut content = Content::from(Span::new("One *word* is strong."));
        let p = Parser::default();
        SubstitutionGroup::AttributeEntryValue.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("One *word* is strong.".to_string().into_boxed_str())
        );
    }

    #[test]
    fn special_chars_and_attributes() {
        let mut content = Content::from(Span::new("bl<ah> {color}"));

        let p = Parser::default().with_intrinsic_attribute(
            "color",
            "red",
            ModificationContext::Anywhere,
        );

        SubstitutionGroup::AttributeEntryValue.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;ah&gt; red".to_string().into_boxed_str())
        );
    }
}

mod header {
    use crate::{
        Parser, Span,
        content::{Content, SubstitutionGroup},
        strings::CowStr,
    };

    #[test]
    fn empty() {
        let mut content = Content::from(Span::new(""));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(Span::new("blah"));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(Span::new("bl<ah>"));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
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
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;a&amp;h&gt;".to_string().into_boxed_str())
        );
    }

    #[test]
    fn ignores_strong_word() {
        let mut content = Content::from(Span::new("One *word* is strong."));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("One *word* is strong."));
    }

    #[test]
    fn ignores_strong_word_with_special_chars() {
        let mut content = Content::from(Span::new("One *wo<r>d* is strong."));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("One *wo&lt;r&gt;d* is strong.".to_string().into_boxed_str())
        );
    }

    #[test]
    fn ignores_marked_string_with_id() {
        let mut content = Content::from(Span::new(r#"[#id]#a few words#"#));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("[#id]#a few words#"));
    }
}
