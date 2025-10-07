mod from_custom_string {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        content::{Content, SubstitutionGroup, SubstitutionStep},
        strings::CowStr,
    };

    #[test]
    fn empty() {
        assert_eq!(SubstitutionGroup::from_custom_string(None, ""), None);
    }

    #[test]
    fn none() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "none"),
            Some(SubstitutionGroup::None)
        );

        assert_eq!(SubstitutionGroup::from_custom_string(None, "nermal"), None);
    }

    #[test]
    fn normal() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "n"),
            Some(SubstitutionGroup::Normal)
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "normal"),
            Some(SubstitutionGroup::Normal)
        );

        assert_eq!(SubstitutionGroup::from_custom_string(None, "nermal"), None);
    }

    #[test]
    fn verbatim() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "v"),
            Some(SubstitutionGroup::Verbatim)
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "verbatim"),
            Some(SubstitutionGroup::Verbatim)
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "verboten"),
            None
        );
    }

    #[test]
    fn special_chars() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "c"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "specialchars"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters
            ]))
        );
    }

    #[test]
    fn quotes() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "q"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Quotes]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "quotes"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Quotes]))
        );
    }

    #[test]
    fn attributes() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "a"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "attributes"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::AttributeReferences
            ]))
        );
    }

    #[test]
    fn replacements() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "r"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::CharacterReplacements
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "replacements"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::CharacterReplacements
            ]))
        );
    }

    #[test]
    fn macros() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "m"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Macros]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "macros"),
            Some(SubstitutionGroup::Custom(vec![SubstitutionStep::Macros]))
        );
    }

    #[test]
    fn post_replacements() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "p"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::PostReplacement
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "post_replacements"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::PostReplacement
            ]))
        );
    }

    #[test]
    fn multiple() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "q,a"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "q, a"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "quotes,attributes"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "x,bogus,no such step"),
            None
        );
    }

    #[test]
    fn subtraction() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "n,-r"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences,
                SubstitutionStep::Macros,
                SubstitutionStep::PostReplacement,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "n,-r,-r,-m"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Quotes,
                SubstitutionStep::AttributeReferences,
                SubstitutionStep::PostReplacement,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "v,-r"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "v,-c"),
            Some(SubstitutionGroup::Custom(vec![]))
        );
    }

    #[test]
    fn addition() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "n,r"),
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
            SubstitutionGroup::from_custom_string(None, "v,m"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Macros,
            ]))
        );
    }

    #[test]
    fn incremental() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "n,r"),
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
            SubstitutionGroup::from_custom_string(None, "v,m"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Macros,
            ]))
        );
    }

    #[test]
    fn prepend() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(
                Some(&SubstitutionGroup::Verbatim),
                "attributes+"
            ),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::AttributeReferences,
                SubstitutionStep::SpecialCharacters,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "attributes+"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::AttributeReferences,
            ]))
        );
    }

    #[test]
    fn append() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(
                Some(&SubstitutionGroup::Verbatim),
                "+attributes"
            ),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::AttributeReferences,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "attributes+"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::AttributeReferences,
            ]))
        );
    }

    #[test]
    fn subtract() {
        assert_eq!(
            SubstitutionGroup::from_custom_string(Some(&SubstitutionGroup::Normal), "-attributes"),
            Some(SubstitutionGroup::Custom(vec![
                SubstitutionStep::SpecialCharacters,
                SubstitutionStep::Quotes,
                SubstitutionStep::CharacterReplacements,
                SubstitutionStep::Macros,
                SubstitutionStep::PostReplacement,
            ]))
        );

        assert_eq!(
            SubstitutionGroup::from_custom_string(None, "-attributes"),
            Some(SubstitutionGroup::Custom(vec![]))
        );
    }

    #[test]
    fn custom_group_with_macros_preserves_passthroughs() {
        let custom_group = SubstitutionGroup::from_custom_string(None, "q,m").unwrap();

        let mut content = Content::from(crate::Span::new(
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
        Parser,
        content::{Content, SubstitutionGroup},
        strings::CowStr,
    };

    #[test]
    fn empty() {
        let mut content = Content::from(crate::Span::default());
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(crate::Span::new("blah"));
        let p = Parser::default();
        SubstitutionGroup::Normal.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(crate::Span::new("bl<ah>"));
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
        let mut content = Content::from(crate::Span::new("bl<a&h>"));
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
        let mut content = Content::from(crate::Span::new("One *word* is strong."));
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
        let mut content = Content::from(crate::Span::new("One *wo<r>d* is strong."));
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
        let mut content = Content::from(crate::Span::new(r#"[#id]#a few words#"#));
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
        Parser,
        content::{Content, SubstitutionGroup},
        parser::ModificationContext,
        strings::CowStr,
    };

    #[test]
    fn empty() {
        let mut content = Content::from(crate::Span::default());
        let p = Parser::default();
        SubstitutionGroup::AttributeEntryValue.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(crate::Span::new("blah"));
        let p = Parser::default();
        SubstitutionGroup::AttributeEntryValue.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(crate::Span::new("bl<ah>"));
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
        let mut content = Content::from(crate::Span::new("bl<a&h>"));
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
        let mut content = Content::from(crate::Span::new("One *word* is strong."));
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
        let mut content = Content::from(crate::Span::new("bl<ah> {color}"));

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
        Parser,
        content::{Content, SubstitutionGroup},
        strings::CowStr,
    };

    #[test]
    fn empty() {
        let mut content = Content::from(crate::Span::default());
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(crate::Span::new("blah"));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(crate::Span::new("bl<ah>"));
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
        let mut content = Content::from(crate::Span::new("bl<a&h>"));
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
        let mut content = Content::from(crate::Span::new("One *word* is strong."));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("One *word* is strong."));
    }

    #[test]
    fn ignores_strong_word_with_special_chars() {
        let mut content = Content::from(crate::Span::new("One *wo<r>d* is strong."));
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
        let mut content = Content::from(crate::Span::new(r#"[#id]#a few words#"#));
        let p = Parser::default();
        SubstitutionGroup::Header.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("[#id]#a few words#"));
    }
}

mod title {
    use crate::{
        Parser,
        content::{Content, SubstitutionGroup},
        strings::CowStr,
    };

    #[test]
    fn empty() {
        let mut content = Content::from(crate::Span::default());
        let p = Parser::default();
        SubstitutionGroup::Title.apply(&mut content, &p, None);
        assert!(content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed(""));
    }

    #[test]
    fn basic_non_empty_span() {
        let mut content = Content::from(crate::Span::new("blah"));
        let p = Parser::default();
        SubstitutionGroup::Title.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(content.rendered, CowStr::Borrowed("blah"));
    }

    #[test]
    fn match_lt_and_gt() {
        let mut content = Content::from(crate::Span::new("bl<ah>"));
        let p = Parser::default();
        SubstitutionGroup::Title.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;ah&gt;".to_string().into_boxed_str())
        );
    }

    #[test]
    fn match_amp() {
        let mut content = Content::from(crate::Span::new("bl<a&h>"));
        let p = Parser::default();
        SubstitutionGroup::Title.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed("bl&lt;a&amp;h&gt;".to_string().into_boxed_str())
        );
    }

    #[test]
    fn strong_word() {
        let mut content = Content::from(crate::Span::new("One *word* is strong."));
        let p = Parser::default();
        SubstitutionGroup::Title.apply(&mut content, &p, None);
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
        let mut content = Content::from(crate::Span::new("One *wo<r>d* is strong."));
        let p = Parser::default();
        SubstitutionGroup::Title.apply(&mut content, &p, None);
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
        let mut content = Content::from(crate::Span::new(r#"[#id]#a few words#"#));
        let p = Parser::default();
        SubstitutionGroup::Title.apply(&mut content, &p, None);
        assert!(!content.is_empty());
        assert_eq!(
            content.rendered,
            CowStr::Boxed(r#"<span id="id">a few words</span>"#.to_string().into_boxed_str())
        );
    }

    #[test]
    fn title_behaves_same_as_normal() {
        let test_input = "One *wo<r>d* is strong with [#id]#marked text#.";

        let mut title_content = Content::from(crate::Span::new(test_input));
        let mut normal_content = Content::from(crate::Span::new(test_input));
        let p = Parser::default();

        SubstitutionGroup::Title.apply(&mut title_content, &p, None);
        SubstitutionGroup::Normal.apply(&mut normal_content, &p, None);

        // Title should produce exactly the same result as Normal
        assert_eq!(title_content.rendered, normal_content.rendered);
    }
}
