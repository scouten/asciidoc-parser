use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{ContentModel, IsBlock},
    document::Document,
    tests::fixtures::{
        attributes::{TAttrlist, TElementAttribute},
        blocks::{TBlock, TMacroBlock, TSectionBlock, TSimpleBlock},
        document::{TDocument, THeader},
        inlines::TInline,
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let doc1 = Document::parse("");
    let doc2 = doc1.clone();
    assert_eq!(doc1, doc2);
}

#[test]
fn empty_source() {
    let doc = Document::parse("");

    assert_eq!(doc.content_model(), ContentModel::Compound);
    assert_eq!(doc.context().deref(), "document");
    assert!(doc.title().is_none());

    assert_eq!(
        doc,
        TDocument {
            header: THeader {
                title: None,
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: TSpan {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![],
            warnings: vec![],
        }
    );
}

#[test]
fn only_spaces() {
    assert_eq!(
        Document::parse("    "),
        TDocument {
            header: THeader {
                title: None,
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: TSpan {
                data: "    ",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![],
            warnings: vec![],
        }
    );
}

#[test]
fn one_simple_block() {
    assert_eq!(
        Document::parse("abc"),
        TDocument {
            header: THeader {
                title: None,
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }),
                source: TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                attrlist: None,
            })],
            warnings: vec![],
        }
    );
}

#[test]
fn two_simple_blocks() {
    assert_eq!(
        Document::parse("abc\n\ndef"),
        TDocument {
            header: THeader {
                title: None,
                attributes: vec!(),
                source: TSpan {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: TSpan {
                data: "abc\n\ndef",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: vec![
                TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    }),
                    source: TSpan {
                        data: "abc\n",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title: None,
                    attrlist: None,
                }),
                TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 3,
                        col: 1,
                        offset: 5,
                    }),
                    source: TSpan {
                        data: "def",
                        line: 3,
                        col: 1,
                        offset: 5,
                    },
                    title: None,
                    attrlist: None,
                })
            ],
            warnings: vec![],
        }
    );
}

#[test]
fn two_blocks_and_title() {
    assert_eq!(
        Document::parse("= Example Title\n\nabc\n\ndef"),
        TDocument {
            header: THeader {
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
            },
            blocks: vec![
                TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 17,
                    }),
                    source: TSpan {
                        data: "abc\n",
                        line: 3,
                        col: 1,
                        offset: 17,
                    },
                    title: None,
                    attrlist: None,
                }),
                TBlock::Simple(TSimpleBlock {
                    inline: TInline::Uninterpreted(TSpan {
                        data: "def",
                        line: 5,
                        col: 1,
                        offset: 22,
                    }),
                    source: TSpan {
                        data: "def",
                        line: 5,
                        col: 1,
                        offset: 22,
                    },
                    title: None,
                    attrlist: None,
                })
            ],
            source: TSpan {
                data: "= Example Title\n\nabc\n\ndef",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: vec![],
        }
    );
}

#[test]
fn extra_space_before_title() {
    assert_eq!(
        Document::parse("=   Example Title\n\nabc"),
        TDocument {
            header: THeader {
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
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 19,
                }),
                source: TSpan {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 19,
                },
                title: None,
                attrlist: None,
            })],
            source: TSpan {
                data: "=   Example Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: vec!(),
        }
    );
}

#[test]
fn err_bad_header() {
    assert_eq!(
        Document::parse("= Title\nnot an attribute\n"),
        TDocument {
            header: THeader {
                title: Some(TSpan {
                    data: "Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                attributes: vec![],
                source: TSpan {
                    data: "= Title\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inline: TInline::Uninterpreted(TSpan {
                    data: "not an attribute",
                    line: 2,
                    col: 1,
                    offset: 8,
                }),
                source: TSpan {
                    data: "not an attribute\n",
                    line: 2,
                    col: 1,
                    offset: 8,
                },
                title: None,
                attrlist: None,
            })],
            source: TSpan {
                data: "= Title\nnot an attribute\n",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: vec!(TWarning {
                source: TSpan {
                    data: "not an attribute",
                    line: 2,
                    col: 1,
                    offset: 8,
                },
                warning: WarningType::DocumentHeaderNotTerminated,
            },),
        }
    );
}

#[test]
fn err_bad_header_and_bad_macro() {
    assert_eq!(
        Document::parse("= Title\nnot an attribute\n\n== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]"),
        TDocument {
            header: THeader {
                title: Some(TSpan {
                    data: "Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                attributes: vec![],
                source: TSpan {
                    data: "= Title\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: vec![TBlock::Simple(TSimpleBlock { inline: TInline::Uninterpreted(
                TSpan {
                    data: "not an attribute",
                    line: 2,
                    col: 1,
                    offset: 8,
                }
            ),
            source: TSpan {
                data: "not an attribute\n",
                line: 2,
                col: 1,
                offset: 8,
            },
            title: None,
            attrlist: None, }),
            TBlock::Section(
                TSectionBlock {
                    level: 1,
                    section_title: TSpan {
                        data: "Section Title",
                        line: 4,
                        col: 4,
                        offset: 29,
                    },
                    blocks: vec![
                        TBlock::Macro(
                            TMacroBlock {
                                name: TSpan {
                                    data: "foo",
                                    line: 6,
                                    col: 1,
                                    offset: 44,
                                },
                                target: Some(
                                    TSpan {
                                        data: "bar",
                                        line: 6,
                                        col: 6,
                                        offset: 49,
                                    },
                                ),
                                macro_attrlist: TAttrlist {
                                    attributes: vec![
                                        TElementAttribute {
                                            name: Some(
                                                TSpan {
                                                    data: "alt",
                                                    line: 6,
                                                    col: 10,
                                                    offset: 53,
                                                },
                                            ),
                                            shorthand_items: vec![],
                                            value: TSpan {
                                                data: "Sunset",
                                                line: 6,
                                                col: 14,
                                                offset: 57,
                                            },
                                            source: TSpan {
                                                data: "alt=Sunset",
                                                line: 6,
                                                col: 10,
                                                offset: 53,
                                            },
                                        },
                                        TElementAttribute {
                                            name: Some(
                                                TSpan {
                                                    data: "width",
                                                    line: 6,
                                                    col: 21,
                                                    offset: 64,
                                                },
                                            ),
                                            shorthand_items: vec![],
                                            value: TSpan {
                                                data: "300",
                                                line: 6,
                                                col: 27,
                                                offset: 70,
                                            },
                                            source: TSpan {
                                                data: "width=300",
                                                line: 6,
                                                col: 21,
                                                offset: 64,
                                            },
                                        },
                                        TElementAttribute {
                                            name: Some(
                                                TSpan {
                                                    data: "height",
                                                    line: 6,
                                                    col: 32,
                                                    offset: 75,
                                                },
                                            ),
                                            shorthand_items: vec![],
                                            value: TSpan {
                                                data: "400",
                                                line: 6,
                                                col: 39,
                                                offset: 82,
                                            },
                                            source: TSpan {
                                                data: "height=400",
                                                line: 6,
                                                col: 32,
                                                offset: 75,
                                            },
                                        },
                                    ],
                                    source: TSpan {
                                        data: "alt=Sunset,width=300,,height=400",
                                        line: 6,
                                        col: 10,
                                        offset: 53,
                                    },
                                },
                                source: TSpan {
                                    data: "foo::bar[alt=Sunset,width=300,,height=400]",
                                    line: 6,
                                    col: 1,
                                    offset: 44,
                                },
                                title: None,
                                attrlist: None,
                            },
                        ),
                    ],
                    source: TSpan {
                        data: "== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]",
                        line: 4,
                        col: 1,
                        offset: 26,
                    },
                    title: None,
                    attrlist: None,
                },
            )],
            source: TSpan {
                data: "= Title\nnot an attribute\n\n== Section Title\n\nfoo::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: vec![TWarning {
                source: TSpan {
                    data: "not an attribute",
                    line: 2,
                    col: 1,
                    offset: 8,
                },
                warning: WarningType::DocumentHeaderNotTerminated,
            },
            TWarning {
                source: TSpan {
                    data: ",",
                    line: 6,
                    col: 30,
                    offset: 73,
                },
                warning: WarningType::EmptyAttributeValue,
                },
            ],
        }
    );
}
