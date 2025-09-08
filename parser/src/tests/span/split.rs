mod into_parse_result {
    use crate::tests::fixtures::Span;

    #[test]
    fn base_case() {
        let s = crate::Span::new("abc");
        let mi = s.into_parse_result(1);

        assert_eq!(
            mi.item,
            Span {
                data: "a",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "bc",
                line: 1,
                col: 2,
                offset: 1,
            }
        );
    }

    #[test]
    fn index_out_of_range() {
        let s = crate::Span::new("abc");
        let mi = s.into_parse_result(4);

        assert_eq!(
            mi.item,
            Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }
}

mod split_at_match_non_empty {
    #[test]
    fn empty_source() {
        let s = crate::Span::new("");
        assert!(s.split_at_match_non_empty(|c| c == ':').is_none());
    }

    #[test]
    fn empty_subspan() {
        let s = crate::Span::new(":abc");
        assert!(s.split_at_match_non_empty(|c| c == ':').is_none());
    }

    #[test]
    fn match_after_first() {
        let s = crate::Span::new("ab:cd");
        let mi = s.split_at_match_non_empty(|c| c == ':').unwrap();

        assert_eq!(mi.item.data(), "ab");
        assert_eq!(mi.item.line(), 1);
        assert_eq!(mi.item.col(), 1);
        assert_eq!(mi.item.byte_offset(), 0);

        assert_eq!(mi.after.data(), ":cd");
        assert_eq!(mi.after.line(), 1);
        assert_eq!(mi.after.col(), 3);
        assert_eq!(mi.after.byte_offset(), 2);
    }

    #[test]
    fn non_empty_no_match() {
        let s = crate::Span::new("abcd");
        let mi = s.split_at_match_non_empty(|c| c == ':').unwrap();

        assert_eq!(mi.item.data(), "abcd");
        assert_eq!(mi.item.line(), 1);
        assert_eq!(mi.item.col(), 1);
        assert_eq!(mi.item.byte_offset(), 0);

        assert_eq!(mi.after.data(), "");
        assert_eq!(mi.after.line(), 1);
        assert_eq!(mi.after.col(), 5);
        assert_eq!(mi.after.byte_offset(), 4);
    }
}
