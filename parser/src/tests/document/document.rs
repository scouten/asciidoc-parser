use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    blocks::{ContentModel, IsBlock, MediaType},
    content::SubstitutionGroup,
    tests::fixtures::{
        Span,
        attributes::{Attrlist, TElementAttribute},
        blocks::{TBlock, TMediaBlock, TSectionBlock, TSimpleBlock},
        content::TContent,
        document::{TDocument, THeader},
        warnings::TWarning,
    },
    warnings::WarningType,
};

#[test]
fn impl_clone() {
    // Silly test to mark the #[derive(...)] line as covered.
    let doc1 = Parser::default().parse("");
    let doc2 = doc1.clone();
    assert_eq!(doc1, doc2);
}

#[test]
fn empty_source() {
    let doc = Parser::default().parse("");

    assert_eq!(doc.content_model(), ContentModel::Compound);
    assert_eq!(doc.raw_context().deref(), "document");
    assert_eq!(doc.resolved_context().deref(), "document");
    assert!(doc.declared_style().is_none());
    assert!(doc.id().is_none());
    assert!(doc.roles().is_empty());
    assert!(doc.title_source().is_none());
    assert!(doc.title().is_none());
    assert!(doc.anchor().is_none());
    assert!(doc.attrlist().is_none());
    assert_eq!(doc.substitution_group(), SubstitutionGroup::Normal);

    assert_eq!(
        doc,
        TDocument {
            header: THeader {
                title_source: None,
                title: None,
                attributes: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: &[],
            warnings: &[],
        }
    );
}

#[test]
fn only_spaces() {
    assert_eq!(
        Parser::default().parse("    "),
        TDocument {
            header: THeader {
                title_source: None,
                title: None,
                attributes: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: Span {
                data: "",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: &[],
            warnings: &[],
        }
    );
}

#[test]
fn one_simple_block() {
    let doc = Parser::default().parse("abc");
    assert_eq!(
        doc,
        TDocument {
            header: THeader {
                title_source: None,
                title: None,
                attributes: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: Span {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: Span {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "abc",
                },
                source: Span {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            warnings: &[],
        }
    );

    assert!(doc.anchor().is_none());
}

#[test]
fn two_simple_blocks() {
    assert_eq!(
        Parser::default().parse("abc\n\ndef"),
        TDocument {
            header: THeader {
                title_source: None,
                title: None,
                attributes: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 1,
                    offset: 0
                },
            },
            source: Span {
                data: "abc\n\ndef",
                line: 1,
                col: 1,
                offset: 0
            },
            blocks: &[
                TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "abc",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "abc",
                    },
                    source: Span {
                        data: "abc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                }),
                TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "def",
                            line: 3,
                            col: 1,
                            offset: 5,
                        },
                        rendered: "def",
                    },
                    source: Span {
                        data: "def",
                        line: 3,
                        col: 1,
                        offset: 5,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                })
            ],
            warnings: &[],
        }
    );
}

#[test]
fn two_blocks_and_title() {
    assert_eq!(
        Parser::default().parse("= Example Title\n\nabc\n\ndef"),
        TDocument {
            header: THeader {
                title_source: Some(Span {
                    data: "Example Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                title: Some("Example Title"),
                attributes: &[],
                source: Span {
                    data: "= Example Title",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[
                TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 17,
                        },
                        rendered: "abc",
                    },
                    source: Span {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 17,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                }),
                TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "def",
                            line: 5,
                            col: 1,
                            offset: 22,
                        },
                        rendered: "def",
                    },
                    source: Span {
                        data: "def",
                        line: 5,
                        col: 1,
                        offset: 22,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                })
            ],
            source: Span {
                data: "= Example Title\n\nabc\n\ndef",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: &[],
        }
    );
}

#[test]
fn extra_space_before_title() {
    assert_eq!(
        Parser::default().parse("=   Example Title\n\nabc"),
        TDocument {
            header: THeader {
                title_source: Some(Span {
                    data: "Example Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                }),
                title: Some("Example Title"),
                attributes: &[],
                source: Span {
                    data: "=   Example Title",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: Span {
                        data: "abc",
                        line: 3,
                        col: 1,
                        offset: 19,
                    },
                    rendered: "abc",
                },
                source: Span {
                    data: "abc",
                    line: 3,
                    col: 1,
                    offset: 19,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "=   Example Title\n\nabc",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: &[],
        }
    );
}

#[test]
fn err_bad_header() {
    assert_eq!(
        Parser::default().parse("= Title\nnot an attribute\n"),
        TDocument {
            header: THeader {
                title_source: Some(Span {
                    data: "Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                title: Some("Title"),
                attributes: &[],
                source: Span {
                    data: "= Title",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[TBlock::Simple(TSimpleBlock {
                content: TContent {
                    original: Span {
                        data: "not an attribute",
                        line: 2,
                        col: 1,
                        offset: 8,
                    },
                    rendered: "not an attribute",
                },
                source: Span {
                    data: "not an attribute",
                    line: 2,
                    col: 1,
                    offset: 8,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "= Title\nnot an attribute",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: &[TWarning {
                source: Span {
                    data: "not an attribute",
                    line: 2,
                    col: 1,
                    offset: 8,
                },
                warning: WarningType::DocumentHeaderNotTerminated,
            },],
        }
    );
}

#[test]
fn err_bad_header_and_bad_macro() {
    assert_eq!(
        Parser::default().parse("= Title\nnot an attribute\n\n== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]"),
        TDocument {
            header: THeader {
                title_source: Some(Span {
                    data: "Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                title: Some("Title"),
                attributes: &[],
                source: Span {
                    data: "= Title",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[
                TBlock::Simple(TSimpleBlock {
                    content: TContent {
                        original: Span {
                            data: "not an attribute",
                            line: 2,
                            col: 1,
                            offset: 8,
                        },
                        rendered: "not an attribute",
                    },
                    source: Span {
                        data: "not an attribute",
                        line: 2,
                        col: 1,
                        offset: 8,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                }
            ),
            TBlock::Section(
                TSectionBlock {
                    level: 1,
                    section_title: Span {
                        data: "Section Title",
                        line: 4,
                        col: 4,
                        offset: 29,
                    },
                    blocks: &[
                        TBlock::Media(
                            TMediaBlock {
                                type_: MediaType::Image,
                                target: Span {
                                    data: "bar",
                                    line: 6,
                                    col: 8,
                                    offset: 51,
                                },
                                macro_attrlist: Attrlist {
                                    attributes: &[
                                        TElementAttribute {
                                            name: Some("alt"),
                                            shorthand_items: &[],
                                            value: "Sunset"
                                        },
                                        TElementAttribute {
                                            name: Some("width"),
                                            shorthand_items: &[],
                                            value: "300"
                                        },
                                        TElementAttribute {
                                            name: Some("height"),
                                            shorthand_items: &[],
                                            value: "400"
                                        },
                                    ],
                                    source: Span {
                                        data: "alt=Sunset,width=300,,height=400",
                                        line: 6,
                                        col: 12,
                                        offset: 55,
                                    },
                                },
                                source: Span {
                                    data: "image::bar[alt=Sunset,width=300,,height=400]",
                                    line: 6,
                                    col: 1,
                                    offset: 44,
                                },
                                title_source: None,
                                title: None,
                                anchor: None,
                                attrlist: None,
                            },
                        ),
                    ],
                    source: Span {
                        data: "== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                        line: 4,
                        col: 1,
                        offset: 26,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },
            )],
            source: Span {
                data: "= Title\nnot an attribute\n\n== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: &[TWarning {
                source: Span {
                    data: "not an attribute",
                    line: 2,
                    col: 1,
                    offset: 8,
                },
                warning: WarningType::DocumentHeaderNotTerminated,
            },
            TWarning {
                source: Span {
                    data: "alt=Sunset,width=300,,height=400",
                    line: 6,
                    col: 12,
                    offset: 55,
                },
                warning: WarningType::EmptyAttributeValue,
                },
            ],
        }
    );
}
