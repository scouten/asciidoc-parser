mod take_ident {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::default();
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn starts_with_non_word() {
        let span = crate::Span::new("#not-a-proper-name");
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn starts_with_hyphen() {
        let span = crate::Span::new("-not-a-proper-name");
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn starts_with_number() {
        let span = crate::Span::new("9not-a-proper-name");
        assert!(span.take_ident().is_none());
    }

    #[test]
    fn stops_at_non_ident() {
        let span = crate::Span::new("x#");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "x",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "#",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn alpha_numeric() {
        let span = crate::Span::new("i94!");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "i94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "!",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }

    #[test]
    fn starts_with_underscore() {
        let span = crate::Span::new("_i94!");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "_i94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "!",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn contains_underscores() {
        let span = crate::Span::new("blah_blah_94 = foo");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "blah_blah_94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: " = foo",
                line: 1,
                col: 13,
                offset: 12
            }
        );
    }

    #[test]
    fn contains_hyphens() {
        let span = crate::Span::new("blah-blah-94 = foo");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "blah",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "-blah-94 = foo",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn stops_at_eof() {
        let span = crate::Span::new("xyz");
        let mi = span.take_ident().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "xyz",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }
}

mod take_attr_name {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::default();
        assert!(span.take_attr_name().is_none());
    }

    #[test]
    fn starts_with_non_word() {
        let span = crate::Span::new("#not-a-proper-name");
        assert!(span.take_attr_name().is_none());
    }

    #[test]
    fn starts_with_hyphen() {
        let span = crate::Span::new("-not-a-proper-name");
        assert!(span.take_attr_name().is_none());
    }

    #[test]
    fn stops_at_non_ident() {
        let span = crate::Span::new("x#");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "x",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "#",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn numeric() {
        let span = crate::Span::new("94!");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "!",
                line: 1,
                col: 3,
                offset: 2
            }
        );
    }

    #[test]
    fn contains_hyphens() {
        let span = crate::Span::new("blah-blah-94 = foo");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "blah-blah-94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: " = foo",
                line: 1,
                col: 13,
                offset: 12
            }
        );
    }

    #[test]
    fn stops_at_eof() {
        let span = crate::Span::new("xyz");
        let mi = span.take_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "xyz",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }
}

mod take_user_attr_name {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::default();
        assert!(span.take_user_attr_name().is_none());
    }

    #[test]
    fn starts_with_non_word() {
        let span = crate::Span::new("#not-a-proper-name");
        assert!(span.take_user_attr_name().is_none());
    }

    #[test]
    fn starts_with_hyphen() {
        let span = crate::Span::new("-not-a-proper-name");
        assert!(span.take_user_attr_name().is_none());
    }

    #[test]
    fn stops_at_non_ident() {
        let span = crate::Span::new("x#");
        let mi = span.take_user_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "x",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "#",
                line: 1,
                col: 2,
                offset: 1
            }
        );
    }

    #[test]
    fn numeric() {
        let span = crate::Span::new("94!");
        let mi = span.take_user_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "!",
                line: 1,
                col: 3,
                offset: 2
            }
        );
    }

    #[test]
    fn contains_hyphens() {
        let span = crate::Span::new("blah-blah-94 = foo");
        let mi = span.take_user_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "blah-blah-94",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: " = foo",
                line: 1,
                col: 13,
                offset: 12
            }
        );
    }

    #[test]
    fn stops_at_eof() {
        let span = crate::Span::new("xyz");
        let mi = span.take_user_attr_name().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "xyz",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }
}

#[test]
fn is_xml_name() {
    assert!(!crate::Span::default().is_xml_name());
    assert!(crate::Span::new("a").is_xml_name());
    assert!(crate::Span::new("a9").is_xml_name());

    assert!(crate::Span::new("install").is_xml_name());
    assert!(crate::Span::new("data-structures").is_xml_name());
    assert!(crate::Span::new("error-handling").is_xml_name());
    assert!(crate::Span::new("subject-and-body").is_xml_name());
    assert!(crate::Span::new("unset_an_attribute").is_xml_name());
    assert!(crate::Span::new(":a").is_xml_name());
    assert!(crate::Span::new("_a").is_xml_name());

    assert!(!crate::Span::new("install the gem").is_xml_name());
    assert!(!crate::Span::new("3 blind mice").is_xml_name());
    assert!(!crate::Span::new("-about-the-author").is_xml_name());
    assert!(!crate::Span::new("\u{037e}abc").is_xml_name());
    assert!(!crate::Span::new("ab\u{037e}c").is_xml_name());
}

mod take_quoted_string {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    #[test]
    fn empty_source() {
        let span = crate::Span::default();
        assert!(span.take_quoted_string().is_none());
    }

    #[test]
    fn unterminated_double_quote() {
        let span = crate::Span::new("\"xxx");
        assert!(span.take_quoted_string().is_none());
    }

    #[test]
    fn double_quoted_string() {
        let span = crate::Span::new("\"abc\"def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );
    }

    #[test]
    fn double_quoted_with_escape() {
        let span = crate::Span::new("\"a\\\"bc\"def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "a\\\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn double_quoted_with_single_quote() {
        let span = crate::Span::new("\"a'bc\"def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "a'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn unterminated_single_quote() {
        let span = crate::Span::new("'xxx");
        assert!(span.take_quoted_string().is_none());
    }

    #[test]
    fn single_quoted_string() {
        let span = crate::Span::new("'abc'def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "abc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 1,
                col: 6,
                offset: 5
            }
        );
    }

    #[test]
    fn single_quoted_with_escape() {
        let span = crate::Span::new("'a\\'bc'def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "a\\'bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 1,
                col: 8,
                offset: 7
            }
        );
    }

    #[test]
    fn single_quoted_with_double_quote() {
        let span = crate::Span::new("'a\"bc'def");
        let mi = span.take_quoted_string().unwrap();

        assert_eq!(
            mi.item,
            Span {
                data: "a\"bc",
                line: 1,
                col: 2,
                offset: 1
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }
}

mod trim_remainder {
    use pretty_assertions_sorted::assert_eq;

    use crate::tests::prelude::*;

    fn advanced_span(source: &'static str, skip: usize) -> crate::Span<'static> {
        let span = crate::Span::new(source);
        span.slice_from(skip..)
    }

    #[test]
    fn empty_spans() {
        let source = advanced_span("abcdef", 6);
        let after = crate::Span::default();

        assert_eq!(
            source.trim_remainder(after),
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_equals_source() {
        let source = advanced_span("abcdef", 6);
        let after = crate::Span::new("abcdef");

        assert_eq!(
            source.trim_remainder(after),
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_too_long() {
        // This is nonsense input, but we should at least not panic in this case.

        let source = advanced_span("abcdef", 6);
        let after = crate::Span::new("abcdef_bogus_bogus");

        assert_eq!(
            source.trim_remainder(after),
            Span {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_is_subset_of_source() {
        let source = advanced_span("abcdef", 2);
        let after = advanced_span("abcdef", 4);

        assert_eq!(
            source.trim_remainder(after),
            Span {
                data: "cd",
                line: 1,
                col: 3,
                offset: 2
            }
        );
    }

    #[test]
    fn rem_is_incomplete_subset_of_source() {
        let source = crate::Span::new("abc\ndef\n");
        let line1 = source.take_normalized_line();
        let line2 = line1.after.take_line();

        assert_eq!(
            source.trim_remainder(line2.item),
            Span {
                data: "abc\n",
                line: 1,
                col: 1,
                offset: 0
            }
        );
    }
}
