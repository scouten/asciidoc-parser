mod is_empty {
    use crate::content::Content;

    #[test]
    fn basic_empty_span() {
        let content = Content::from(crate::Span::new(""));
        assert!(content.is_empty());
    }

    #[test]
    fn basic_non_empty_span() {
        let content = Content::from(crate::Span::new("blah"));
        assert!(!content.is_empty());
    }
}

mod replace_str {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        content::Content,
        tests::fixtures::{Span, content::TContent},
    };

    #[test]
    fn basic_empty_span() {
        let mut content = Content::from(crate::Span::new(""));
        content.replace_str("<", "&lt;");

        assert_eq!(
            content,
            TContent {
                original: Span {
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
        let mut content = Content::from(crate::Span::new("no matching text"));
        content.replace_str("<", "&lt;");

        assert_eq!(
            content,
            TContent {
                original: Span {
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
        let mut content = Content::from(crate::Span::new("some matching < text"));
        content.replace_str("<", "&lt;");

        assert_eq!(
            content,
            TContent {
                original: Span {
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
