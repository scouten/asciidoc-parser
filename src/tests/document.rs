use crate::{
    blocks::{Block, SimpleBlock},
    Document, Span,
};

#[test]
fn empty_source() {
    let doc = Document::parse("").unwrap();

    let span = doc.span();
    assert_eq!(span.data(), &"");
    assert_eq!(span.line(), 1);
    assert_eq!(span.col(), 1);
    assert_eq!(span.byte_offset(), 0);

    let mut blocks = doc.blocks();
    assert!(blocks.next().is_none());
}

#[test]
fn only_spaces() {
    let doc = Document::parse("    ").unwrap();

    let span = doc.span();
    assert_eq!(span.data(), &"    ");
    assert_eq!(span.line(), 1);
    assert_eq!(span.col(), 1);
    assert_eq!(span.byte_offset(), 0);

    let mut blocks: std::slice::Iter<'_, Block<'_>> = doc.blocks();
    assert!(blocks.next().is_none());
}

#[test]
fn one_simple_block() {
    let doc = Document::parse("abc").unwrap();

    let span = doc.span();
    assert_eq!(span.data(), &"abc");
    assert_eq!(span.line(), 1);
    assert_eq!(span.col(), 1);
    assert_eq!(span.byte_offset(), 0);

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

    let span = doc.span();
    assert_eq!(span.data(), &"abc\n\ndef");
    assert_eq!(span.line(), 1);
    assert_eq!(span.col(), 1);
    assert_eq!(span.byte_offset(), 0);

    let mut blocks: std::slice::Iter<'_, Block<'_>> = doc.blocks();

    let expected = Block::Simple(SimpleBlock {
        inlines: vec![Span::new("abc", true)],
    });
    assert_eq!(blocks.next(), Some(&expected));

    let Block::Simple(def_block) = blocks.next().unwrap();
    // else ... error

    let span0 = def_block.inlines.first().unwrap();
    assert_eq!(span0.data(), &"def");
    assert_eq!(span0.line(), 3);
    assert_eq!(span0.col(), 1);
    assert_eq!(span0.byte_offset(), 5);
    assert_eq!(def_block.inlines.len(), 1);

    assert!(blocks.next().is_none());
}
