// Adapted from Asciidoctor's paragraphs test suite, found in
// https://github.com/asciidoctor/asciidoctor/blob/main/test/paragraphs_test.rb.
//
// IMPORTANT: In porting this, I've disregarded compatibility mode (stated
// limitation of `asciidoc-parser` crate) and alternate (non-HTML) back ends.

mod normal {
    use std::collections::HashMap;

    use pretty_assertions_sorted::assert_eq;

    use crate::{Parser, document::RefType, tests::prelude::*};

    #[test]
    fn should_treat_plain_text_separated_by_blank_lines_as_paragraphs() {
        let doc =
            Parser::default().parse("Plain text for the win!\n\nYep. Text. Plain and simple.");

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
                        offset: 0,
                    },
                },
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "Plain text for the win!",
                                line: 1,
                                col: 1,
                                offset: 0,
                            },
                            rendered: "Plain text for the win!",
                        },
                        source: Span {
                            data: "Plain text for the win!",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "Yep. Text. Plain and simple.",
                                line: 3,
                                col: 1,
                                offset: 25,
                            },
                            rendered: "Yep. Text. Plain and simple.",
                        },
                        source: Span {
                            data: "Yep. Text. Plain and simple.",
                            line: 3,
                            col: 1,
                            offset: 25,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                ],
                source: Span {
                    data: "Plain text for the win!\n\nYep. Text. Plain and simple.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn should_associate_block_title_with_paragraph() {
        let doc = Parser::default().parse(".Titled\nParagraph.\n\nWinning.");

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
                        offset: 0,
                    },
                },
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "Paragraph.",
                                line: 2,
                                col: 1,
                                offset: 8,
                            },
                            rendered: "Paragraph.",
                        },
                        source: Span {
                            data: ".Titled\nParagraph.",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        title_source: Some(Span {
                            data: "Titled",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },),
                        title: Some("Titled",),
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "Winning.",
                                line: 4,
                                col: 1,
                                offset: 20,
                            },
                            rendered: "Winning.",
                        },
                        source: Span {
                            data: "Winning.",
                            line: 4,
                            col: 1,
                            offset: 20,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                ],
                source: Span {
                    data: ".Titled\nParagraph.\n\nWinning.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn no_duplicate_block_before_next_section() {
        let doc = Parser::default().parse("= Title\n\nPreamble\n\n== First Section\n\nParagraph 1\n\nParagraph 2\n\n== Second Section\n\nLast words");

        assert_eq!(
            doc,
            Document {
                header: Header {
                    title_source: Some(Span {
                        data: "Title",
                        line: 1,
                        col: 3,
                        offset: 2,
                    },),
                    title: Some("Title",),
                    attributes: &[],
                    author_line: None,
                    revision_line: None,
                    comments: &[],
                    source: Span {
                        data: "= Title",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                },
                blocks: &[
                    Block::Preamble(Preamble {
                        blocks: &[Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "Preamble",
                                    line: 3,
                                    col: 1,
                                    offset: 9,
                                },
                                rendered: "Preamble",
                            },
                            source: Span {
                                data: "Preamble",
                                line: 3,
                                col: 1,
                                offset: 9,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                        },),],
                        source: Span {
                            data: "Preamble",
                            line: 3,
                            col: 1,
                            offset: 9,
                        },
                    },),
                    Block::Section(SectionBlock {
                        level: 1,
                        section_title: Content {
                            original: Span {
                                data: "First Section",
                                line: 5,
                                col: 4,
                                offset: 22,
                            },
                            rendered: "First Section",
                        },
                        blocks: &[
                            Block::Simple(SimpleBlock {
                                content: Content {
                                    original: Span {
                                        data: "Paragraph 1",
                                        line: 7,
                                        col: 1,
                                        offset: 37,
                                    },
                                    rendered: "Paragraph 1",
                                },
                                source: Span {
                                    data: "Paragraph 1",
                                    line: 7,
                                    col: 1,
                                    offset: 37,
                                },
                                title_source: None,
                                title: None,
                                anchor: None,
                                anchor_reftext: None,
                                attrlist: None,
                            },),
                            Block::Simple(SimpleBlock {
                                content: Content {
                                    original: Span {
                                        data: "Paragraph 2",
                                        line: 9,
                                        col: 1,
                                        offset: 50,
                                    },
                                    rendered: "Paragraph 2",
                                },
                                source: Span {
                                    data: "Paragraph 2",
                                    line: 9,
                                    col: 1,
                                    offset: 50,
                                },
                                title_source: None,
                                title: None,
                                anchor: None,
                                anchor_reftext: None,
                                attrlist: None,
                            },),
                        ],
                        source: Span {
                            data: "== First Section\n\nParagraph 1\n\nParagraph 2",
                            line: 5,
                            col: 1,
                            offset: 19,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                        section_type: SectionType::Normal,
                        section_id: Some("_first_section",),
                        section_number: None,
                    },),
                    Block::Section(SectionBlock {
                        level: 1,
                        section_title: Content {
                            original: Span {
                                data: "Second Section",
                                line: 11,
                                col: 4,
                                offset: 66,
                            },
                            rendered: "Second Section",
                        },
                        blocks: &[Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "Last words",
                                    line: 13,
                                    col: 1,
                                    offset: 82,
                                },
                                rendered: "Last words",
                            },
                            source: Span {
                                data: "Last words",
                                line: 13,
                                col: 1,
                                offset: 82,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                        },),],
                        source: Span {
                            data: "== Second Section\n\nLast words",
                            line: 11,
                            col: 1,
                            offset: 63,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                        section_type: SectionType::Normal,
                        section_id: Some("_second_section",),
                        section_number: None,
                    },),
                ],
                source: Span {
                    data: "= Title\n\nPreamble\n\n== First Section\n\nParagraph 1\n\nParagraph 2\n\n== Second Section\n\nLast words",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([
                        (
                            "_first_section",
                            RefEntry {
                                id: "_first_section",
                                reftext: Some("First Section",),
                                ref_type: RefType::Section,
                            },
                        ),
                        (
                            "_second_section",
                            RefEntry {
                                id: "_second_section",
                                reftext: Some("Second Section",),
                                ref_type: RefType::Section,
                            },
                        ),
                    ]),
                    reftext_to_id: HashMap::from([
                        ("First Section", "_first_section",),
                        ("Second Section", "_second_section",),
                    ]),
                },
            }
        );
    }

    #[test]
    fn does_not_treat_wrapped_line_as_a_list_item() {
        let doc = Parser::default().parse("paragraph\n. wrapped line");

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
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "paragraph\n. wrapped line",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "paragraph\n. wrapped line",
                    },
                    source: Span {
                        data: "paragraph\n. wrapped line",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "paragraph\n. wrapped line",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn does_not_treat_wrapped_line_as_a_block_title() {
        let doc = Parser::default().parse("paragraph\n.wrapped line");

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
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "paragraph\n.wrapped line",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        rendered: "paragraph\n.wrapped line",
                    },
                    source: Span {
                        data: "paragraph\n.wrapped line",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                },),],
                source: Span {
                    data: "paragraph\n.wrapped line",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn interprets_normal_paragraph_style_as_normal_paragraph() {
        let doc = Parser::default().parse("[normal]\nNormal paragraph.\nNothing special.");

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
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: "Normal paragraph.\nNothing special.",
                            line: 2,
                            col: 1,
                            offset: 9,
                        },
                        rendered: "Normal paragraph.\nNothing special.",
                    },
                    source: Span {
                        data: "[normal]\nNormal paragraph.\nNothing special.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: Some(Attrlist {
                        attributes: &[ElementAttribute {
                            name: None,
                            value: "normal",
                            shorthand_items: &["normal",],
                        },],
                        anchor: None,
                        source: Span {
                            data: "normal",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: Span {
                    data: "[normal]\nNormal paragraph.\nNothing special.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn removes_indentation_from_literal_paragraph_marked_as_normal() {
        let doc = Parser::default()
            .parse("[normal]\n Normal paragraph.\n  Nothing special.\n Last line.");

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
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: " Normal paragraph.\n  Nothing special.\n Last line.",
                            line: 2,
                            col: 1,
                            offset: 9,
                        },
                        rendered: "Normal paragraph.\n Nothing special.\nLast line.",
                    },
                    source: Span {
                        data: "[normal]\n Normal paragraph.\n  Nothing special.\n Last line.",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: Some(Attrlist {
                        attributes: &[ElementAttribute {
                            name: None,
                            value: "normal",
                            shorthand_items: &["normal"],
                        },],
                        anchor: None,
                        source: Span {
                            data: "normal",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: Span {
                    data: "[normal]\n Normal paragraph.\n  Nothing special.\n Last line.",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn normal_paragraph_terminates_at_block_attribute_list() {
        let doc = Parser::default().parse("normal text\n[literal]\nliteral text");

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
                        offset: 0,
                    },
                },
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "normal text",
                                line: 1,
                                col: 1,
                                offset: 0,
                            },
                            rendered: "normal text",
                        },
                        source: Span {
                            data: "normal text",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "literal text",
                                line: 3,
                                col: 1,
                                offset: 22,
                            },
                            rendered: "literal text",
                        },
                        source: Span {
                            data: "[literal]\nliteral text",
                            line: 2,
                            col: 1,
                            offset: 12,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: Some(Attrlist {
                            attributes: &[ElementAttribute {
                                name: None,
                                value: "literal",
                                shorthand_items: &["literal"],
                            },],
                            anchor: None,
                            source: Span {
                                data: "literal",
                                line: 2,
                                col: 2,
                                offset: 13,
                            },
                        },),
                    },),
                ],
                source: Span {
                    data: "normal text\n[literal]\nliteral text",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn normal_paragraph_terminates_at_block_delimiter() {
        let doc = Parser::default().parse("normal text\n--\ntext in open block\n--");

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
                        offset: 0,
                    },
                },
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "normal text",
                                line: 1,
                                col: 1,
                                offset: 0,
                            },
                            rendered: "normal text",
                        },
                        source: Span {
                            data: "normal text",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                    Block::CompoundDelimited(CompoundDelimitedBlock {
                        blocks: &[Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "text in open block",
                                    line: 3,
                                    col: 1,
                                    offset: 15,
                                },
                                rendered: "text in open block",
                            },
                            source: Span {
                                data: "text in open block",
                                line: 3,
                                col: 1,
                                offset: 15,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                        },),],
                        context: "open",
                        source: Span {
                            data: "--\ntext in open block\n--",
                            line: 2,
                            col: 1,
                            offset: 12,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                ],
                source: Span {
                    data: "normal text\n--\ntext in open block\n--",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn normal_paragraph_terminates_at_list_continuation() {
        let doc = Parser::default().parse("normal text\n+");

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
                        offset: 0,
                    },
                },
                blocks: &[
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "normal text",
                                line: 1,
                                col: 1,
                                offset: 0,
                            },
                            rendered: "normal text",
                        },
                        source: Span {
                            data: "normal text",
                            line: 1,
                            col: 1,
                            offset: 0,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                    Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "+",
                                line: 2,
                                col: 1,
                                offset: 12,
                            },
                            rendered: "+",
                        },
                        source: Span {
                            data: "+",
                            line: 2,
                            col: 1,
                            offset: 12,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    },),
                ],
                source: Span {
                    data: "normal text\n+",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[test]
    fn normal_style_turns_literal_paragraph_into_normal_paragraph() {
        let doc =
            Parser::default().parse("[normal]\n normal paragraph,\n despite the leading indent");

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
                        offset: 0,
                    },
                },
                blocks: &[Block::Simple(SimpleBlock {
                    content: Content {
                        original: Span {
                            data: " normal paragraph,\n despite the leading indent",
                            line: 2,
                            col: 1,
                            offset: 9,
                        },
                        rendered: "normal paragraph,\ndespite the leading indent",
                    },
                    source: Span {
                        data: "[normal]\n normal paragraph,\n despite the leading indent",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: Some(Attrlist {
                        attributes: &[ElementAttribute {
                            name: None,
                            value: "normal",
                            shorthand_items: &["normal"],
                        },],
                        anchor: None,
                        source: Span {
                            data: "normal",
                            line: 1,
                            col: 2,
                            offset: 1,
                        },
                    },),
                },),],
                source: Span {
                    data: "[normal]\n normal paragraph,\n despite the leading indent",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                warnings: &[],
                source_map: SourceMap(&[]),
                catalog: Catalog {
                    refs: HashMap::from([]),
                    reftext_to_id: HashMap::from([]),
                },
            }
        );
    }

    #[ignore]
    #[test]
    fn port_from_ruby() {
        todo!(
            "Port this: {}",
            r###"
    test 'automatically promotes index terms in DocBook output if indexterm-promotion-option is set' do
      input = <<~'EOS'
      Here is an index entry for ((tigers)).
      indexterm:[Big cats,Tigers,Siberian Tiger]
      Here is an index entry for indexterm2:[Linux].
      (((Operating Systems,Linux)))
      Note that multi-entry terms generate separate index entries.
      EOS

      output = convert_string_to_embedded input, backend: 'docbook', attributes: { 'indexterm-promotion-option' => '' }
      assert_xpath '/simpara', output, 1
      term1 = xmlnodes_at_xpath '(//indexterm)[1]', output, 1
      assert_equal %(<indexterm>\n<primary>tigers</primary>\n</indexterm>), term1.to_s
      assert term1.next.content.start_with?('tigers')

      term2 = xmlnodes_at_xpath '(//indexterm)[2]', output, 1
      term2_elements = term2.elements
      assert_equal 3, term2_elements.size
      assert_equal '<primary>Big cats</primary>', term2_elements[0].to_s
      assert_equal '<secondary>Tigers</secondary>', term2_elements[1].to_s
      assert_equal '<tertiary>Siberian Tiger</tertiary>', term2_elements[2].to_s

      term3 = xmlnodes_at_xpath '(//indexterm)[3]', output, 1
      term3_elements = term3.elements
      assert_equal 2, term3_elements.size
      assert_equal '<primary>Tigers</primary>', term3_elements[0].to_s
      assert_equal '<secondary>Siberian Tiger</secondary>', term3_elements[1].to_s

      term4 = xmlnodes_at_xpath '(//indexterm)[4]', output, 1
      term4_elements = term4.elements
      assert_equal 1, term4_elements.size
      assert_equal '<primary>Siberian Tiger</primary>', term4_elements[0].to_s

      term5 = xmlnodes_at_xpath '(//indexterm)[5]', output, 1
      assert_equal %(<indexterm>\n<primary>Linux</primary>\n</indexterm>), term5.to_s
      assert term5.next.content.start_with?('Linux')

      assert_xpath '(//indexterm)[6]/*', output, 2
      assert_xpath '(//indexterm)[7]/*', output, 1
    end

    test 'does not automatically promote index terms in DocBook output if indexterm-promotion-option is not set' do
      input = <<~'EOS'
      The Siberian Tiger is one of the biggest living cats.
      indexterm:[Big cats,Tigers,Siberian Tiger]
      Note that multi-entry terms generate separate index entries.
      (((Operating Systems,Linux)))
      EOS

      output = convert_string_to_embedded input, backend: 'docbook'

      assert_css 'indexterm', output, 2

      terms = xmlnodes_at_css 'indexterm', output, 2
      term1 = terms[0]
      term1_elements = term1.elements
      assert_equal 3, term1_elements.size
      assert_equal '<primary>Big cats</primary>', term1_elements[0].to_s
      assert_equal '<secondary>Tigers</secondary>', term1_elements[1].to_s
      assert_equal '<tertiary>Siberian Tiger</tertiary>', term1_elements[2].to_s
      term2 = terms[1]
      term2_elements = term2.elements
      assert_equal 2, term2_elements.size
      assert_equal '<primary>Operating Systems</primary>', term2_elements[0].to_s
      assert_equal '<secondary>Linux</secondary>', term2_elements[1].to_s
    end

    test 'normal paragraph should honor explicit subs list' do
      input = <<~'EOS'
      [subs="specialcharacters"]
      *<Hey Jude>*
      EOS

      output = convert_string_to_embedded input
      assert_includes output, '*&lt;Hey Jude&gt;*'
    end

    test 'normal paragraph should honor specialchars shorthand' do
      input = <<~'EOS'
      [subs="specialchars"]
      *<Hey Jude>*
      EOS

      output = convert_string_to_embedded input
      assert_includes output, '*&lt;Hey Jude&gt;*'
    end

    test 'should add a hardbreak at end of each line when hardbreaks option is set' do
      input = <<~'EOS'
      [%hardbreaks]
      read
      my
      lips
      EOS

      output = convert_string_to_embedded input
      assert_css 'br', output, 2
      assert_xpath '//p', output, 1
      assert_includes output, "<p>read<br>\nmy<br>\nlips</p>"
    end

    test 'should be able to toggle hardbreaks by setting hardbreaks-option on document' do
      input = <<~'EOS'
      :hardbreaks-option:

      make
      it
      so

      :!hardbreaks:

      roll it back
      EOS

      output = convert_string_to_embedded input
      assert_xpath '(//p)[1]/br', output, 2
      assert_xpath '(//p)[2]/br', output, 0
    end
"###
        );
    }
}

#[ignore]
#[test]
fn port_from_ruby() {
    todo!(
        "Port this: {}",
        r###"

  context 'Literal' do
    test 'single-line literal paragraphs' do
      input = <<~'EOS'
      you know what?

       LITERALS

       ARE LITERALLY

       AWESOME!
      EOS
      output = convert_string_to_embedded input
      assert_xpath '//pre', output, 3
    end

    test 'multi-line literal paragraph' do
      input = <<~'EOS'
      Install instructions:

       yum install ruby rubygems
       gem install asciidoctor

      You're good to go!
      EOS
      output = convert_string_to_embedded input
      assert_xpath '//pre', output, 1
      # indentation should be trimmed from literal block
      assert_xpath %(//pre[text() = "yum install ruby rubygems\ngem install asciidoctor"]), output, 1
    end

    test 'literal paragraph' do
      input = <<~'EOS'
      [literal]
      this text is literally literal
      EOS
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="literalblock"]//pre[text()="this text is literally literal"]), output, 1
    end

    test 'should read content below literal style verbatim' do
      input = <<~'EOS'
      [literal]
      image::not-an-image-block[]
      EOS
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="literalblock"]//pre[text()="image::not-an-image-block[]"]), output, 1
      assert_css 'img', output, 0
    end

    test 'listing paragraph' do
      input = <<~'EOS'
      [listing]
      this text is a listing
      EOS
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="listingblock"]//pre[text()="this text is a listing"]), output, 1
    end

    test 'source paragraph' do
      input = <<~'EOS'
      [source]
      use the source, luke!
      EOS
      block = block_from_string input
      assert_equal :listing, block.context
      assert_equal 'source', (block.attr 'style')
      assert_equal :paragraph, (block.attr 'cloaked-context')
      assert_nil (block.attr 'language')
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="listingblock"]//pre[@class="highlight"]/code[text()="use the source, luke!"]), output, 1
    end

    test 'source code paragraph with language' do
      input = <<~'EOS'
      [source, perl]
      die 'zomg perl is tough';
      EOS
      block = block_from_string input
      assert_equal :listing, block.context
      assert_equal 'source', (block.attr 'style')
      assert_equal :paragraph, (block.attr 'cloaked-context')
      assert_equal 'perl', (block.attr 'language')
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="listingblock"]//pre[@class="highlight"]/code[@class="language-perl"][@data-lang="perl"][text()="die 'zomg perl is tough';"]), output, 1
    end

    test 'literal paragraph terminates at block attribute list' do
      input = <<~'EOS'
       literal text
      [normal]
      normal text
      EOS
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="literalblock"]), output, 1
      assert_xpath %(/*[@class="paragraph"]), output, 1
    end

    test 'literal paragraph terminates at block delimiter' do
      input = <<~'EOS'
       literal text
      --
      normal text
      --
      EOS
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="literalblock"]), output, 1
      assert_xpath %(/*[@class="openblock"]), output, 1
    end

    test 'literal paragraph terminates at list continuation' do
      input = <<~'EOS'
       literal text
      +
      EOS
      output = convert_string_to_embedded input
      assert_xpath %(/*[@class="literalblock"]), output, 1
      assert_xpath %(/*[@class="literalblock"]//pre[text() = "literal text"]), output, 1
      assert_xpath %(/*[@class="paragraph"]), output, 1
      assert_xpath %(/*[@class="paragraph"]/p[text() = "+"]), output, 1
    end
  end

  context 'Quote' do
    test 'single-line quote paragraph' do
      input = <<~'EOS'
      [quote]
      Famous quote.
      EOS
      output = convert_string input
      assert_xpath '//*[@class = "quoteblock"]', output, 1
      assert_xpath '//*[@class = "quoteblock"]//p', output, 0
      assert_xpath '//*[@class = "quoteblock"]//*[contains(text(), "Famous quote.")]', output, 1
    end

    test 'quote paragraph terminates at list continuation' do
      input = <<~'EOS'
      [quote]
      A famouse quote.
      +
      EOS
      output = convert_string_to_embedded input
      assert_css '.quoteblock:root', output, 1
      assert_css '.paragraph:root', output, 1
      assert_xpath %(/*[@class="paragraph"]/p[text() = "+"]), output, 1
    end

    test 'verse paragraph' do
      output = convert_string "[verse]\nFamous verse."
      assert_xpath '//*[@class = "verseblock"]', output, 1
      assert_xpath '//*[@class = "verseblock"]/pre', output, 1
      assert_xpath '//*[@class = "verseblock"]//p', output, 0
      assert_xpath '//*[@class = "verseblock"]/pre[normalize-space(text()) = "Famous verse."]', output, 1
    end

    test 'should perform normal subs on a verse paragraph' do
      input = <<~'EOS'
      [verse]
      _GET /groups/link:#group-id[\{group-id\}]_
      EOS

      output = convert_string_to_embedded input
      assert_includes output, '<pre class="content"><em>GET /groups/<a href="#group-id">{group-id}</a></em></pre>'
    end

    test 'quote paragraph should honor explicit subs list' do
      input = <<~'EOS'
      [subs="specialcharacters"]
      [quote]
      *Hey Jude*
      EOS

      output = convert_string_to_embedded input
      assert_includes output, '*Hey Jude*'
    end
  end

  context 'special' do
    test 'note multiline syntax' do
      Asciidoctor::ADMONITION_STYLES.each do |style|
        assert_xpath %(//div[@class='admonitionblock #{style.downcase}']), convert_string(%([#{style}]\nThis is a winner.))
      end
    end

    test 'note block syntax' do
      Asciidoctor::ADMONITION_STYLES.each do |style|
        assert_xpath %(//div[@class='admonitionblock #{style.downcase}']), convert_string(%([#{style}]\n====\nThis is a winner.\n====))
      end
    end

    test 'note inline syntax' do
      Asciidoctor::ADMONITION_STYLES.each do |style|
        assert_xpath %(//div[@class='admonitionblock #{style.downcase}']), convert_string(%(#{style}: This is important, fool!))
      end
    end

    test 'should process preprocessor conditional in paragraph content' do
      input = <<~'EOS'
      ifdef::asciidoctor-version[]
      [sidebar]
      First line of sidebar.
      ifdef::backend[The backend is {backend}.]
      Last line of sidebar.
      endif::[]
      EOS

      expected = <<~'EOS'.chop
      <div class="sidebarblock">
      <div class="content">
      First line of sidebar.
      The backend is html5.
      Last line of sidebar.
      </div>
      </div>
      EOS

      result = convert_string_to_embedded input
      assert_equal expected, result
    end

    context 'Styled Paragraphs' do
      test 'should wrap text in simpara for styled paragraphs when converted to DocBook' do
        input = <<~'EOS'
        = Book
        :doctype: book

        [preface]
        = About this book

        [abstract]
        An abstract for the book.

        = Part 1

        [partintro]
        An intro to this part.

        == Chapter 1

        [sidebar]
        Just a side note.

        [example]
        As you can see here.

        [quote]
        Wise words from a wise person.

        [open]
        Make it what you want.
        EOS

        output = convert_string input, backend: 'docbook'
        assert_css 'abstract > simpara', output, 1
        assert_css 'partintro > simpara', output, 1
        assert_css 'sidebar > simpara', output, 1
        assert_css 'informalexample > simpara', output, 1
        assert_css 'blockquote > simpara', output, 1
        assert_css 'chapter > simpara', output, 1
      end

      test 'should convert open paragraph to open block' do
        input = <<~'EOS'
        [open]
        Make it what you want.
        EOS

        output = convert_string_to_embedded input
        assert_css '.openblock', output, 1
        assert_css '.openblock p', output, 0
      end

      test 'should wrap text in simpara for styled paragraphs with title when converted to DocBook' do
        input = <<~'EOS'
        = Book
        :doctype: book

        [preface]
        = About this book

        [abstract]
        .Abstract title
        An abstract for the book.

        = Part 1

        [partintro]
        .Part intro title
        An intro to this part.

        == Chapter 1

        [sidebar]
        .Sidebar title
        Just a side note.

        [example]
        .Example title
        As you can see here.

        [quote]
        .Quote title
        Wise words from a wise person.
        EOS

        output = convert_string input, backend: 'docbook'
        assert_css 'abstract > title', output, 1
        assert_xpath '//abstract/title[text() = "Abstract title"]', output, 1
        assert_css 'abstract > title + simpara', output, 1
        assert_css 'partintro > title', output, 1
        assert_xpath '//partintro/title[text() = "Part intro title"]', output, 1
        assert_css 'partintro > title + simpara', output, 1
        assert_css 'sidebar > title', output, 1
        assert_xpath '//sidebar/title[text() = "Sidebar title"]', output, 1
        assert_css 'sidebar > title + simpara', output, 1
        assert_css 'example > title', output, 1
        assert_xpath '//example/title[text() = "Example title"]', output, 1
        assert_css 'example > title + simpara', output, 1
        assert_css 'blockquote > title', output, 1
        assert_xpath '//blockquote/title[text() = "Quote title"]', output, 1
        assert_css 'blockquote > title + simpara', output, 1
      end
    end

    context 'Inline doctype' do
      test 'should only format and output text in first paragraph when doctype is inline' do
        input = "http://asciidoc.org[AsciiDoc] is a _lightweight_ markup language...\n\nignored"
        output = convert_string input, doctype: 'inline'
        assert_equal '<a href="http://asciidoc.org">AsciiDoc</a> is a <em>lightweight</em> markup language&#8230;&#8203;', output
      end

      test 'should output nil and warn if first block is not a paragraph' do
        input = '* bullet'
        using_memory_logger do |logger|
          output = convert_string input, doctype: 'inline'
          assert_nil output
          assert_message logger, :WARN, '~no inline candidate'
        end
      end
    end
  end

  context 'Custom' do
    test 'should not warn if paragraph style is unregisted' do
      input = <<~'EOS'
      [foo]
      bar
      EOS
      using_memory_logger do |logger|
        convert_string_to_embedded input
        assert_empty logger.messages
      end
    end

    test 'should log debug message if paragraph style is unknown and debug level is enabled' do
      input = <<~'EOS'
      [foo]
      bar
      EOS
      using_memory_logger Logger::Severity::DEBUG do |logger|
        convert_string_to_embedded input
        assert_message logger, :DEBUG, '<stdin>: line 2: unknown style for paragraph: foo', Hash
      end
    end
  end

"###
    );
}
