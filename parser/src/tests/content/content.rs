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
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        tests::fixtures::{content::TSpanOrSubstitution, TSpan},
        Content, Span,
    };

    #[test]
    fn basic_empty_span() {
        let content = Content::from(Span::new(""));
        let mut sns = content.spans_and_substitutions();
        assert!(sns.next().is_none());
    }

    #[test]
    fn basic_non_empty_span() {
        let content = Content::from(Span::new("blah"));
        let mut sns = content.spans_and_substitutions();

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

mod replace_str {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        tests::fixtures::{
            content::{TContent, TSpanOrSubstitution, TSubstitution},
            TSpan,
        },
        Content, Span,
    };

    #[test]
    fn basic_empty_span() {
        let mut content = Content::from(Span::new(""));
        content.replace_str("<", "&lt;");

        assert_eq!(
            content,
            TContent {
                original: TSpan {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "",
                substitutions: vec![],
            }
        );

        let mut sns = content.spans_and_substitutions();
        assert!(sns.next().is_none());
    }

    #[test]
    fn no_match() {
        let mut content = Content::from(Span::new("no matching text"));
        content.replace_str("<", "&lt;");

        assert_eq!(
            content,
            TContent {
                original: TSpan {
                    data: "no matching text",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "no matching text",
                substitutions: vec![],
            }
        );

        let mut sns = content.spans_and_substitutions();
        let s = sns.next().unwrap();

        assert_eq!(
            s,
            TSpanOrSubstitution::Span(TSpan {
                data: "no matching text",
                line: 1,
                col: 1,
                offset: 0,
            })
        );

        assert!(sns.next().is_none());
    }

    #[test]
    fn one_match() {
        let mut content = Content::from(Span::new("some matching < text"));
        content.replace_str("<", "&lt;");

        dbg!(&content);

        assert_eq!(
            content,
            TContent {
                original: TSpan {
                    data: "some matching < text",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "some matching &lt; text",
                substitutions: vec![TSubstitution {
                    original: TSpan {
                        data: "<",
                        line: 1,
                        col: 15,
                        offset: 14,
                    },
                    replacement: "&lt;",
                },],
            }
        );

        let mut sns = content.spans_and_substitutions();

        let s = sns.next().unwrap();
        assert_eq!(
            s,
            TSpanOrSubstitution::Span(TSpan {
                data: "some matching ",
                line: 1,
                col: 1,
                offset: 0,
            })
        );

        let s = sns.next().unwrap();
        assert_eq!(
            s,
            TSpanOrSubstitution::Substitution(TSubstitution {
                original: TSpan {
                    data: "<",
                    line: 1,
                    col: 15,
                    offset: 14,
                },
                replacement: "&lt;",
            })
        );

        let s = sns.next().unwrap();
        assert_eq!(
            s,
            TSpanOrSubstitution::Span(TSpan {
                data: " text",
                line: 1,
                col: 16,
                offset: 15st,
            })
        );

        assert!(sns.next().is_none());
    }
}
