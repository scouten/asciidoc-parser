use pretty_assertions_sorted::assert_eq;

use crate::{
    document::Document,
    tests::fixtures::{
        blocks::{TBlock, TSimpleBlock},
        document::TDocument,
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
    assert_eq!(
        Document::parse("").unwrap(),
        TDocument {
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
            source: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inlines: vec![TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },],
                source: TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            })],
        }
    );
}

#[test]
fn two_simple_blocks() {
    assert_eq!(
        Document::parse("abc\n\ndef").unwrap(),
        TDocument {
            source: TSpan {
                data: "abc\n\ndef",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![
                TBlock::Simple(TSimpleBlock {
                    inlines: vec![TSpan {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },],
                    source: TSpan {
                        data: "abc\n",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }
                }),
                TBlock::Simple(TSimpleBlock {
                    inlines: vec![TSpan {
                        data: "def",
                        line: 3,
                        col: 1,
                        offset: 5,
                    },],
                    source: TSpan {
                        data: "def",
                        line: 3,
                        col: 1,
                        offset: 5,
                    }
                }),
            ],
        }
    );
}
