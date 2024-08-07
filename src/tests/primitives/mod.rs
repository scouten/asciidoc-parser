mod trim_input_for_rem {
    use pretty_assertions_sorted::assert_eq;

    use crate::{primitives::trim_input_for_rem, tests::fixtures::TSpan, Span};

    fn advanced_span(source: &'static str, skip: usize) -> Span<'static> {
        let span = Span::new(source);
        span.slice_from(skip..)
    }

    #[test]
    fn empty_spans() {
        let inp = advanced_span("abcdef", 6);
        let rem = Span::new("");

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_equals_inp() {
        let inp = advanced_span("abcdef", 6);
        let rem = Span::new("abcdef");

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
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

        let inp = advanced_span("abcdef", 6);
        let rem = Span::new("abcdef_bogus_bogus");

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
                data: "",
                line: 1,
                col: 7,
                offset: 6
            }
        );
    }

    #[test]
    fn rem_is_subset_of_inp() {
        let inp = advanced_span("abcdef", 2);
        let rem = advanced_span("abcdef", 4);

        assert_eq!(
            trim_input_for_rem(inp, rem),
            TSpan {
                data: "cd",
                line: 1,
                col: 3,
                offset: 2
            }
        );
    }
}
