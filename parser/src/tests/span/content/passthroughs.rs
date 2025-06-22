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

    #[test]
    fn adds_warning_text_for_unresolved_passthrough_id() {
        let mut content = Content::from(Span::new("pass:q,a[*<{backend}>*]"));
        let pt = Passthroughs::extract_from(&mut content);

        assert_eq!(
            content,
            TContent {
                original: TSpan {
                    data: "pass:q,a[*<{backend}>*]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "\u{96}0\u{97}",
            }
        );

        assert_eq!(
            pt,
            Passthroughs(vec![Passthrough {
                text: "*<{backend}>*".to_owned(),
                subs: SubstitutionGroup::Custom(vec![
                    SubstitutionStep::Quotes,
                    SubstitutionStep::AttributeReferences,
                ]),
                type_: None,
                attrlist: None,
            },],)
        );

        let parser = Parser::default().with_intrinsic_attribute(
            "backend",
            "html5",
            ModificationContext::ApiOnly,
        );

        pt.0[0].subs.apply(&mut content, &parser, None);

        content.rendered = "\u{96}99\u{97}".into();

        pt.restore_to(&mut content, &parser);

        assert_eq!(
            content,
            TContent {
                original: TSpan {
                    data: "pass:q,a[*<{backend}>*]",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "(INTERNAL ERROR: Unresolved passthrough index 99)",
            }
        );
    }
}
