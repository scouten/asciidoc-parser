use std::ops::Deref;

use pretty_assertions_sorted::assert_eq;

use crate::{
    Parser,
    blocks::{ContentModel, IsBlock, MediaType},
    content::SubstitutionGroup,
    tests::prelude::*,
    warnings::WarningType,
};

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
        Document {
            header: Header {
                title_source: None,
                title: None,
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
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
        Document {
            header: Header {
                title_source: None,
                title: None,
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
                source: Span {
                    data: "",
                    line: 1,
                    col: 5,
                    offset: 4
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
        Document {
            header: Header {
                title_source: None,
                title: None,
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
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
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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
        Document {
            header: Header {
                title_source: None,
                title: None,
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
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
                Block::Simple(SimpleBlock {
                    content: Content {
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
                Block::Simple(SimpleBlock {
                    content: Content {
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
        Document {
            header: Header {
                title_source: Some(Span {
                    data: "Example Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                title: Some("Example Title"),
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
                source: Span {
                    data: "= Example Title",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[
                Block::Simple(SimpleBlock {
                    content: Content {
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
                Block::Simple(SimpleBlock {
                    content: Content {
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
fn blank_lines_before_header() {
    let doc = Parser::default().parse("\n\n= Example Title\n\nabc\n\ndef");

    assert_eq!(
        doc,
        Document {
            header: Header {
                title_source: Some(Span {
                    data: "Example Title",
                    line: 3,
                    col: 3,
                    offset: 4,
                },),
                title: Some("Example Title",),
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
                source: Span {
                    data: "= Example Title",
                    line: 3,
                    col: 1,
                    offset: 2,
                },
            },
            blocks: &[
                Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "abc",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        rendered: "abc",
                    },
                    source: Span {
                        data: "abc",
                        line: 5,
                        col: 1,
                        offset: 19,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),
                Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 24,
                        },
                        rendered: "def",
                    },
                    source: Span {
                        data: "def",
                        line: 7,
                        col: 1,
                        offset: 24,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),
            ],
            source: Span {
                data: "\n\n= Example Title\n\nabc\n\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn blank_lines_and_comment_before_header() {
    let doc = Parser::default().parse("\n// ignore this comment\n= Example Title\n\nabc\n\ndef");

    assert_eq!(
        doc,
        Document {
            header: Header {
                title_source: Some(Span {
                    data: "Example Title",
                    line: 3,
                    col: 3,
                    offset: 26,
                },),
                title: Some("Example Title",),
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[Span {
                    data: "// ignore this comment",
                    line: 2,
                    col: 1,
                    offset: 1,
                },],
                source: Span {
                    data: "// ignore this comment\n= Example Title",
                    line: 2,
                    col: 1,
                    offset: 1,
                },
            },
            blocks: &[
                Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "abc",
                            line: 5,
                            col: 1,
                            offset: 41,
                        },
                        rendered: "abc",
                    },
                    source: Span {
                        data: "abc",
                        line: 5,
                        col: 1,
                        offset: 41,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),
                Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "def",
                            line: 7,
                            col: 1,
                            offset: 46,
                        },
                        rendered: "def",
                    },
                    source: Span {
                        data: "def",
                        line: 7,
                        col: 1,
                        offset: 46,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },),
            ],
            source: Span {
                data: "\n// ignore this comment\n= Example Title\n\nabc\n\ndef",
                line: 1,
                col: 1,
                offset: 0,
            },
            warnings: &[],
        }
    );
}

#[test]
fn extra_space_before_title() {
    assert_eq!(
        Parser::default().parse("=   Example Title\n\nabc"),
        Document {
            header: Header {
                title_source: Some(Span {
                    data: "Example Title",
                    line: 1,
                    col: 5,
                    offset: 4,
                }),
                title: Some("Example Title"),
                attributes: &[],
                author_line: None,
                revision_line: None,
                comments: &[],
                source: Span {
                    data: "=   Example Title",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
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
        Parser::default()
            .parse("= Title\nJane Smith <jane@example.com>\nv1, 2025-09-28\nnot an attribute\n"),
        Document {
            header: Header {
                title_source: Some(Span {
                    data: "Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                title: Some("Title"),
                attributes: &[],
                author_line: Some(AuthorLine {
                    authors: &[Author {
                        name: "Jane Smith",
                        firstname: "Jane",
                        middlename: None,
                        lastname: Some("Smith"),
                        email: Some("jane@example.com"),
                    }],
                    source: Span {
                        data: "Jane Smith <jane@example.com>",
                        line: 2,
                        col: 1,
                        offset: 8,
                    },
                }),
                revision_line: Some(RevisionLine {
                    revnumber: Some("1",),
                    revdate: "2025-09-28",
                    revremark: None,
                    source: Span {
                        data: "v1, 2025-09-28",
                        line: 3,
                        col: 1,
                        offset: 38,
                    },
                },),
                comments: &[],
                source: Span {
                    data: "= Title\nJane Smith <jane@example.com>\nv1, 2025-09-28",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[Block::Simple(SimpleBlock {
                content: Content {
                    original: Span {
                        data: "not an attribute",
                        line: 4,
                        col: 1,
                        offset: 53,
                    },
                    rendered: "not an attribute",
                },
                source: Span {
                    data: "not an attribute",
                    line: 4,
                    col: 1,
                    offset: 53,
                },
                title_source: None,
                title: None,
                anchor: None,
                attrlist: None,
            })],
            source: Span {
                data: "= Title\nJane Smith <jane@example.com>\nv1, 2025-09-28\nnot an attribute",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: &[Warning {
                source: Span {
                    data: "not an attribute",
                    line: 4,
                    col: 1,
                    offset: 53,
                },
                warning: WarningType::DocumentHeaderNotTerminated,
            },],
        }
    );
}

#[test]
fn err_bad_header_and_bad_macro() {
    assert_eq!(
        Parser::default().parse("= Title\nJane Smith <jane@example.com>\nv1, 2025-09-28\nnot an attribute\n\n== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]"),
        Document {
            header: Header {
                title_source: Some(Span {
                    data: "Title",
                    line: 1,
                    col: 3,
                    offset: 2,
                }),
                title: Some("Title"),
                attributes: &[],
                author_line: Some(AuthorLine {
                    authors: &[Author {
                        name: "Jane Smith",
                        firstname: "Jane",
                        middlename: None,
                        lastname: Some("Smith"),
                        email: Some("jane@example.com"),
                    }],
                    source: Span {
                        data: "Jane Smith <jane@example.com>",
                        line: 2,
                        col: 1,
                        offset: 8,
                    },
                }),
                revision_line: Some(
                    RevisionLine {
                        revnumber: Some(
                            "1",
                        ),
                        revdate: "2025-09-28",
                        revremark: None,
                        source: Span {
                            data: "v1, 2025-09-28",
                            line: 3,
                            col: 1,
                            offset: 38,
                        },
                    },
                ),
                comments: &[],
                source: Span {
                    data: "= Title\nJane Smith <jane@example.com>\nv1, 2025-09-28",
                    line: 1,
                    col: 1,
                    offset: 0,
                }
            },
            blocks: &[
                Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "not an attribute",
                            line: 4,
                            col: 1,
                            offset: 53,
                        },
                        rendered: "not an attribute",
                    },
                    source: Span {
                        data: "not an attribute",
                        line: 4,
                        col: 1,
                        offset: 53,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                }
            ),
            Block::Section(
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 6,
                            col: 4,
                            offset: 74,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[
                        Block::Media(
                            MediaBlock {
                                type_: MediaType::Image,
                                target: Span {
                                    data: "bar",
                                    line: 8,
                                    col: 8,
                                    offset: 96,
                                },
                                macro_attrlist: Attrlist {
                                    attributes: &[
                                        ElementAttribute {
                                            name: Some("alt"),
                                            shorthand_items: &[],
                                            value: "Sunset"
                                        },
                                        ElementAttribute {
                                            name: Some("width"),
                                            shorthand_items: &[],
                                            value: "300"
                                        },
                                        ElementAttribute {
                                            name: Some("height"),
                                            shorthand_items: &[],
                                            value: "400"
                                        },
                                    ],
                                    anchor: None,
                                    source: Span {
                                        data: "alt=Sunset,width=300,,height=400",
                                        line: 8,
                                        col: 12,
                                        offset: 100,
                                    },
                                },
                                source: Span {
                                    data: "image::bar[alt=Sunset,width=300,,height=400]",
                                    line: 8,
                                    col: 1,
                                    offset: 89,
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
                        line: 6,
                        col: 1,
                        offset: 71,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    attrlist: None,
                },
            )],
            source: Span {
                data: "= Title\nJane Smith <jane@example.com>\nv1, 2025-09-28\nnot an attribute\n\n== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                line: 1,
                col: 1,
                offset: 0
            },
            warnings: &[
                Warning {
                    source: Span {
                        data: "not an attribute",
                        line: 4,
                        col: 1,
                        offset: 53,
                    },
                    warning: WarningType::DocumentHeaderNotTerminated,
                },
                Warning {
                    source: Span {
                        data: "alt=Sunset,width=300,,height=400",
                        line: 8,
                        col: 12,
                        offset: 100,
                    },
                    warning: WarningType::EmptyAttributeValue,
                },
            ],
        }
    );
}
