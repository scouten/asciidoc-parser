use crate::{
    tests::fixtures::{
        blocks::{TBlock, TSimpleBlock},
        TSpan,
    },
    Document,
};

#[test]
fn empty_source() {
    let doc = Document::parse("").unwrap();

    assert_eq!(
        doc.span(),
        TSpan {
            data: "",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    let mut blocks = doc.blocks();
    assert!(blocks.next().is_none());
}

#[test]
fn only_spaces() {
    let doc = Document::parse("    ").unwrap();

    assert_eq!(
        doc.span(),
        TSpan {
            data: "    ",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    let mut blocks = doc.blocks();
    assert!(blocks.next().is_none());
}

#[test]
fn one_simple_block() {
    let doc = Document::parse("abc").unwrap();

    assert_eq!(
        doc.span(),
        TSpan {
            data: "abc",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    let mut blocks = doc.blocks();

    assert_eq!(
        blocks.next().unwrap(),
        &TBlock::Simple(TSimpleBlock {
            inlines: vec![TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },],
        })
    );

    assert!(blocks.next().is_none());
}

#[test]
fn two_simple_blocks() {
    let doc = Document::parse("abc\n\ndef").unwrap();

    assert_eq!(
        doc.span(),
        TSpan {
            data: "abc\n\ndef",
            line: 1,
            col: 1,
            offset: 0
        }
    );

    let mut blocks = doc.blocks();

    assert_eq!(
        blocks.next().unwrap(),
        &TBlock::Simple(TSimpleBlock {
            inlines: vec![TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0,
            },],
        })
    );

    assert_eq!(
        blocks.next().unwrap(),
        &TBlock::Simple(TSimpleBlock {
            inlines: vec![TSpan {
                data: "def",
                line: 3,
                col: 1,
                offset: 5,
            },],
        })
    );

    assert!(blocks.next().is_none());
}
