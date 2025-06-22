#![allow(unused)] // TEMPORARY: DO NOT MERGE with this line.

mod passthroughs {
    use pretty_assertions_sorted::assert_eq;

    use crate::{
        blocks::Block,
        parser::ModificationContext,
        span::content::{Passthrough, Passthroughs, SubstitutionGroup, SubstitutionStep},
        tests::fixtures::{
            blocks::{TBlock, TSimpleBlock},
            content::TContent,
            TSpan,
        },
        Content, Parser, Span,
    };

    #[test]
    fn inline_double_plus_with_escaped_attrlist() {
        let mut p = Parser::default();
        let maw = Block::parse(Span::new(r#"abc \[attrs]++text++"#), &mut p);

        let block = maw.item.unwrap().item;

        assert_eq!(
            block,
            TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: TSpan {
                        data: r#"abc \[attrs]++text++"#,
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "abc [attrs]text",
                },
                source: TSpan {
                    data: r#"abc \[attrs]++text++"#,
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                anchor: None,
                attrlist: None,
            },)
        );
    }
}
