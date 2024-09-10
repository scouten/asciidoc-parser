mod into_parse_result {
    use crate::{tests::fixtures::TSpan, Span};

    #[test]
    fn base_case() {
        let s = Span::new("abc");
        let mi = s.into_parse_result(1);

        assert_eq!(
            mi.item,
            TSpan {
                data: "a",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "bc",
                line: 1,
                col: 2,
                offset: 1,
            }
        );
    }

    #[test]
    fn index_out_of_range() {
        let s = Span::new("abc");
        let mi = s.into_parse_result(4);

        assert_eq!(
            mi.item,
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3,
            }
        );
    }
}

mod split_at_match_non_empty {
    use crate::Span;

    #[test]
    fn empty_source() {
        let s = Span::new("");
        assert!(s.split_at_match_non_empty(|c| c == ':').is_none());
    }

    #[test]
    fn empty_subspan() {
        let s = Span::new(":abc");
        assert!(s.split_at_match_non_empty(|c| c == ':').is_none());
    }

    #[test]
    fn match_after_first() {
        let s = Span::new("ab:cd");
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
        let s = Span::new("abcd");
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
