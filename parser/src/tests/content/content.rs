mod is_empty {
    use crate::{Content, Span};

    #[test]
    fn basic_empty_span() {
        let content = Content::from(Span::new(""));
        assert!(content.is_empty());
    }

    #[test]
    fn basic_non_empty_span() {
        let content = Content::from(Span::new("blah"));
        assert!(!content.is_empty());
    }
}

mod spans_and_substitutions {
    use crate::{
        tests::fixtures::{content::TSpanOrSubstitution, TSpan},
        Content, Span,
    };

    #[test]
    fn basic_empty_span() {
        let content = Content::from(Span::new(""));
        let mut sns = content.spans_and_substitions();
        assert!(sns.next().is_none());
    }

    #[test]
    fn basic_non_empty_span() {
        let content = Content::from(Span::new("blah"));
        let mut sns = content.spans_and_substitions();

        let item = sns.next().unwrap();
        assert_eq!(
            &item,
            TSpanOrSubstitution::Span(TSpan {
                data: "blah",
                line: 1,
                col: 1,
                offset: 0,
            })
        );

        assert!(sns.next().is_none());
    }
}
