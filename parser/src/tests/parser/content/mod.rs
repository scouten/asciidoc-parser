use crate::{
    blocks::Block,
    tests::fixtures::{
        blocks::{TBlock, TSimpleBlock},
        content::TContent,
        TSpan,
    },
    Parser, Span,
};

// Most of the content API is tested via
// tests/asciidoctor_rb/substitutions_test.rs. This covers a few coverage gaps.
#[test]
fn preserves_character_references() {
    let mut p = Parser::default();
    let maw = Block::parse(
        Span::new("Something something &#167; &amp; blah blah"),
        &mut p,
    );

    let block = maw.item.unwrap().item;

    assert_eq!(
        block,
        TBlock::Simple(TSimpleBlock {
            content: TContent {
                original: TSpan {
                    data: "Something something &#167; &amp; blah blah",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                rendered: "Something something &#167; &amp; blah blah",
            },
            source: TSpan {
                data: "Something something &#167; &amp; blah blah",
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
