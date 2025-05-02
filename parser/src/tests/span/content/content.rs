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

mod replace_str {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        tests::fixtures::{content::TContent, TSpan},
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
            }
        );
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
            }
        );
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
            }
        );
    }
}
