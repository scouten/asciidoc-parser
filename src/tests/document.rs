use crate::{
    blocks::{Block, SimpleBlock},
    tests::fixtures::TSpan,
    Document, Span,
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

    let mut blocks: std::slice::Iter<'_, Block<'_>> = doc.blocks();
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

    let mut blocks: std::slice::Iter<'_, Block<'_>> = doc.blocks();

    let expected = Block::Simple(SimpleBlock {
        inlines: vec![Span::new("abc", true)],
    });
    assert_eq!(blocks.next(), Some(&expected));

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

    let mut blocks: std::slice::Iter<'_, Block<'_>> = doc.blocks();

    let expected = Block::Simple(SimpleBlock {
        inlines: vec![Span::new("abc", true)],
    });
    assert_eq!(blocks.next(), Some(&expected));

    let Block::Simple(def_block) = blocks.next().unwrap();
    // else ... error

    assert_eq!(
        def_block.inlines.first().unwrap(),
        TSpan {
            data: "def",
            line: 3,
            col: 1,
            offset: 5
        }
    );

    assert!(blocks.next().is_none());
}
