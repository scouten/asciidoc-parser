use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{ContentModel, IsBlock},
    document::Document,
    tests::fixtures::{
        blocks::{TBlock, TSimpleBlock},
        document::{TDocument, THeader},
        inlines::TInline,
        TSpan,
    },
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let doc1 = Document::parse("").unwrap();
    let doc2 = doc1.clone();
    assert_eq!(doc1, doc2);
}

#[test]
fn empty_source() {
    let doc = Document::parse("").unwrap();

    assert_eq!(doc.content_model(), ContentModel::Compound);
    assert_eq!(doc.context().deref(), "document");

    assert_eq!(
        doc,
        TDocument {
            header: None,
            source: TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![],
        }
    );
}

#[test]
fn only_spaces() {
    assert_eq!(
        Document::parse("    ").unwrap(),
        TDocument {
            header: None,
            source: TSpan {
                data: "    ",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![],
        }
    );
}

#[test]
fn one_simple_block() {
    assert_eq!(
        Document::parse("abc").unwrap(),
        TDocument {
            header: None,
            source: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            )))]
        }
    );
}

#[test]
fn two_simple_blocks() {
    assert_eq!(
        Document::parse("abc\n\ndef").unwrap(),
        TDocument {
            header: None,
            source: TSpan {
                data: "abc\n\ndef",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![
                TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }))),
                TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                    data: "def",
                    line: 3,
                    col: 1,
                    offset: 5,
                })))
            ],
        }
    );
}

#[test]
fn two_blocks_and_title() {
    assert_eq!(
        Document::parse("= Example Title\n\nabc\n\ndef").unwrap(),
        TDocument {
            header: Some(THeader {
                title: Some(TSpan {
                    data: "Example Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                attributes: vec![],
                source: TSpan {
                    data: "= Example Title\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            }),
            blocks: vec![
                TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 17,
                }))),
                TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(TSpan {
                    data: "def",
                    line: 5,
                    col: 1,
                    offset: 22,
                })))
            ],
            source: TSpan {
                data: "= Example Title\n\nabc\n\ndef",
                line: 1,
                col: 1,
                offset: 0
            },
        }
    );
}

#[test]
fn extra_space_before_title() {
    assert_eq!(
        Document::parse("=   Example Title\n\nabc").unwrap(),
        TDocument {
            header: Some(THeader {
                title: Some(TSpan {
                    data: "Example Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                }),
                attributes: vec![],
                source: TSpan {
                    data: "=   Example Title\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            }),
            blocks: vec![TBlock::Simple(TSimpleBlock(TInline::Uninterpreted(
                TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 19,
                }
            )))],
            source: TSpan {
                data: "=   Example Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0
            },
        }
    );
}

#[test]
fn bad_header() {
    assert!(Document::parse("= Title\nnot an attribute\n").is_none());
}
