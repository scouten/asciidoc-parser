mod compound_delimited;
mod r#macro;
mod raw_delimited;
mod section;
mod simple;

mod content_model {
    use crate::blocks::ContentModel;

    #[test]
    fn impl_copy() {
        // Silly test to mark the #[derive(...)] line as covered.
        let c1 = ContentModel::Simple;
        let c2 = c1;
        assert_eq!(c1, c2);
    }
}

mod error_cases {
    use std::ops::Deref;

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::{preamble::Preamble, ContentModel, IsBlock, SectionBlock},
        tests::fixtures::{
            blocks::{TBlock, TSectionBlock, TSimpleBlock},
            inlines::TInline,
            TSpan,
        },
        warnings::MatchAndWarnings,
    };

    #[test]
    fn missing_block_after_title_line() {
        let MatchAndWarnings { item: mi, warnings } = SectionBlock::parse(&Preamble::new(
            "=== Section Title\n\nabc\n\n== Section 2\n\ndef",
        ))
        .unwrap();

        assert_eq!(mi.item.content_model(), ContentModel::Compound);
        assert_eq!(mi.item.context().deref(), "section");

        assert_eq!(
            mi.item,
            TSectionBlock {
                level: 2,
                section_title: TSpan {
                    data: "Section Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                },
                blocks: vec![TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 19,
                    }),
                    title: None
                })],
                source: TSpan {
                    // TO DO: Fix bug that includes blank lines.
                    data: "=== Section Title\n\nabc\n\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
            }
        );

        assert_eq!(
            mi.after,
            TSpan {
                data: "== Section 2\n\ndef",
                line: 5,
                col: 1,
                offset: 24
            }
        );

        dbg!(&warnings);
        assert!(warnings.is_empty());
    }
}
