//! Tracks https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang/-/blob/main/docs/modules/blocks/pages/delimited.adoc?ref_type=heads
//!
//! Tracking commit aa906159, current as of 2024-10-26.

use pretty_assertions_sorted::assert_eq;

use crate::{
    blocks::{Block, ContentModel, IsBlock},
    tests::fixtures::{
        blocks::{TBlock, TCompoundDelimitedBlock, TRawDelimitedBlock, TSimpleBlock},
        inlines::TInline,
        warnings::TWarning,
        TSpan,
    },
    warnings::WarningType,
    Span,
};

// = Delimited Blocks

// In AsciiDoc, a delimited block is a region of content that's bounded on
// either side by a pair of congruent linewise delimiters. A delimited block is
// used either to enclose other blocks (e.g., multiple paragraphs) or set the
// content model of the content (e.g., verbatim). Delimited blocks are a subset
// of all block types in AsciiDoc.

#[test]
fn overview() {
    // == Overview

    // Delimited blocks are defined using structural containers, which are the fixed
    // set of recognized block enclosures in the AsciiDoc syntax. Here's the
    // structural container for a literal block:

    // ----
    // ....
    // This text will be treated as verbatim content.
    // ....
    // ----

    let block = Block::parse(Span::new(
        "....\nThis text will be treated as verbatim content.\n....",
    ))
    .unwrap_if_no_warnings()
    .unwrap()
    .item;

    assert_eq!(
        block,
        TBlock::RawDelimited(TRawDelimitedBlock {
            lines: vec![TSpan {
                data: "This text will be treated as verbatim content.",
                line: 2,
                col: 1,
                offset: 5,
            },],
            content_model: ContentModel::Verbatim,
            context: "literal",
            source: TSpan {
                data: "....\nThis text will be treated as verbatim content.\n....",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        },)
    );

    // A structural container has an opening delimiter and a closing delimiter. The
    // opening delimiter follows the block metadata, if present. Leading and
    // trailing empty lines in a structure container are not considered significant
    // and are automatically removed. The remaining lines define a block's content.

    let block = Block::parse(Span::new(
        "....\n\n\nThis text will be treated as verbatim content.\n\n\n....",
    ))
    .unwrap_if_no_warnings()
    .unwrap()
    .item;

    assert_eq!(
        block,
        TBlock::RawDelimited(TRawDelimitedBlock {
            lines: vec![TSpan {
                data: "This text will be treated as verbatim content.",
                line: 4,
                col: 1,
                offset: 7,
            },],
            content_model: ContentModel::Verbatim,
            context: "literal",
            source: TSpan {
                data: "....\n\n\nThis text will be treated as verbatim content.\n\n\n....",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        },)
    );

    // These enclosures not only define the boundaries of a block's content, but
    // also imply a content model (e.g., verbatim content or a subtree). In
    // certain cases, they provide a mechanism for nesting blocks. However,
    // delimited blocks cannot be interleaved.

    // A delimited block has the unique ability of being able to be repurposed
    // by the AsciiDoc syntax, through both built-in mappings and mappings
    // for custom blocks defined by an extension. To understand how
    // delimited blocks work, it's important to understand structural
    // containers, their linewise delimiters, their default contexts, and
    // their expected content models, as well as block nesting and
    // masquerading.
}

#[test]
fn linewise_delimiters() {
    // == Linewise delimiters

    // A delimited block is characterized by a pair of congruent linewise
    // delimiters. The opening and closing delimiter must match exactly, both in
    // length and in sequence of characters. These delimiters, sometimes referred to
    // as fences, enclose the content and explicitly mark its boundaries. Within the
    // boundaries of a delimited block, you can enter any content or empty lines.
    // The block doesn't end until the ending delimiter is found. The block metadata
    // (block attribute and anchor lines) goes above the opening delimiter (thus
    // outside the delimited region).

    let maw_block = Block::parse(Span::new(
        "....\nThis text will be treated as verbatim content.\n.....",
    ));

    assert_eq!(
        maw_block.warnings,
        vec![TWarning {
            source: TSpan {
                data: "....",
                line: 1,
                col: 1,
                offset: 0,
            },
            warning: WarningType::UnterminatedDelimitedBlock,
        },]
    );

    // Here's an example of a delimited example block:

    // ----
    // ====
    // This is an example of an example block.
    // That's so meta.
    // ====
    // ----

    let block = Block::parse(Span::new(
        "====\nThis is an example of an example block.\nThat's so meta.\n====",
    ))
    .unwrap_if_no_warnings()
    .unwrap()
    .item;

    assert_eq!(
        block,
        TBlock::CompoundDelimited(TCompoundDelimitedBlock {
            blocks: vec![TBlock::Simple(TSimpleBlock {
                inline: TInline::Sequence(
                    vec![
                        TInline::Uninterpreted(TSpan {
                            data: "This is an example of an example block.",
                            line: 2,
                            col: 1,
                            offset: 5,
                        },),
                        TInline::Uninterpreted(TSpan {
                            data: "That's so meta.",
                            line: 3,
                            col: 1,
                            offset: 45,
                        },),
                    ],
                    TSpan {
                        data: "This is an example of an example block.\nThat's so meta.\n",
                        line: 2,
                        col: 1,
                        offset: 5,
                    },
                ),
                source: TSpan {
                    data: "This is an example of an example block.\nThat's so meta.\n",
                    line: 2,
                    col: 1,
                    offset: 5,
                },
                title: None,
                attrlist: None,
            },),],
            context: "example",
            source: TSpan {
                data: "====\nThis is an example of an example block.\nThat's so meta.\n====",
                line: 1,
                col: 1,
                offset: 0,
            },
            title: None,
            attrlist: None,
        },)
    );

    // Typically, the delimiter is written using the minimum allowable length (4
    // characters, with the exception of the open block, which currently has a
    // fixed length of 2 characters). The length of the delimiter lines can be
    // varied to accommodate nested blocks.

    // The valid set of delimiters for defining a delimited block, and the
    // meaning they have, is defined by the available structural containers,
    // covered next.
}

#[test]
fn structural_containers() {
    // == Structural containers

    // Structural containers are the fixed set of recognized block enclosures
    // (delimited regions) defined by the AsciiDoc language. These enclosures
    // provide a reusable building block in the AsciiDoc syntax. By evaluating
    // the structural container and the block metadata, the processor will
    // determine which kind of block to make.

    // Each structural container has an expected content model. For built-in
    // blocks, it's the context of the block that determines the content model,
    // though most built-in blocks adhere to the expected content model. Custom
    // blocks have the ability to designate the content model. Even in these
    // cases, though, the content model should be chosen to honor the semantics
    // of the structural container. This allows a text editor to understand how
    // to parse the block and provide a reasonable fallback when the extension
    // is not loaded.

    // Some structural containers are reused for different purposes, such as the
    // structural container for the quote block being used for a verse block.

    // === Summary of structural containers

    // The table below lists the structural containers, documenting the name,
    // default context, and delimiter line for each.

    // .Structural containers in AsciiDoc
    // [#table-of-structural-containers,cols="1,2m,2,2l"]
    // |===
    // |Type |Default context |Content model (expected) |Minimum delimiter

    // |comment
    // |_n/a_
    // |_n/a_
    // |////

    let mi = Block::parse(Span::new("////\n////"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Raw);
    assert_eq!(mi.item.context().as_ref(), "comment");

    // |example
    // |:example
    // |compound
    // |====

    let mi = Block::parse(Span::new("====\n===="))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().as_ref(), "example");

    // |listing
    // |:listing
    // |verbatim
    // |----

    let mi = Block::parse(Span::new("----\n----"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
    assert_eq!(mi.item.context().as_ref(), "listing");

    // |literal
    // |:literal
    // |verbatim
    // |....

    let mi = Block::parse(Span::new("....\n...."))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Verbatim);
    assert_eq!(mi.item.context().as_ref(), "literal");

    // |open
    // |:open
    // |compound
    // |--

    let mi = Block::parse(Span::new("--\n--"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().as_ref(), "open");

    // |sidebar
    // |:sidebar
    // |compound
    // |****

    let mi = Block::parse(Span::new("****\n****"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().as_ref(), "sidebar");

    // |table
    // |:table
    // |table
    // |\|===
    // ,===
    // :===
    // !===

    if false {
        todo!("Support for table parsing");
    }

    // |pass
    // |:pass
    // |raw
    // |++++

    let mi = Block::parse(Span::new("++++\n++++"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Raw);
    assert_eq!(mi.item.context().as_ref(), "pass");

    // |quote
    // |:quote
    // |compound
    // |____
    // |===

    let mi = Block::parse(Span::new("____\n____"))
        .unwrap_if_no_warnings()
        .unwrap();

    assert_eq!(mi.item.content_model(), ContentModel::Compound);
    assert_eq!(mi.item.context().as_ref(), "quote");

    // You may notice the absence of the source block. That's because source is
    // not a container type. Rather, it's a specialization of the listing (or
    // literal) container as designated by the block style. The verse and
    // admonition blocks are also noticeably absent. That's because they
    // repurpose the structural containers for quote and example blocks,
    // respectively.

    // The default context is assumed when an explicit block style is not
    // present.

    // Currently, table is a specialized structural container that cannot be
    // enlisted as a custom block.

    // Unlike other structural containers, a comment block is not preserved in
    // the parsed document and therefore doesn't have a context or content
    // model.

    // TIP: When creating a custom block, it's important to choose a structural
    // container that provides the right content model. This allows a text
    // editor to understand how to parse the block and provide a reasonable
    // fallback when the extension is not loaded.

    // Structural containers are used to define delimited blocks. The structural
    // container provides a default context and expected content model, but the
    // actual context and content model is determined after considering the
    // metadata on the block (specifically the declared block content).
}

#[test]
fn nesting_blocks() {
    // [#nesting]
    // == Nesting blocks

    // Using delimited blocks, you can nest blocks inside of one other.
    // (Blocks can also be nested inside sections, list items, and table cells,
    // which is a separate topic).

    // First, the parent block must have the compound content model.
    // The compound content model means that the block's content is a sequence of
    // zero or more blocks.

    // When nesting a block that uses a different structural container from the
    // parent, it's enough to ensure that the child block is entirely inside the
    // parent block. Delimited blocks cannot be interleaved.

    // [source]
    // ....
    // ====
    // Here's a sample AsciiDoc document:

    // ----
    // = Document Title
    // Author Name

    // Content goes here.
    // ----

    // The document header is useful, but not required.
    // ====
    // ....

    let block = Block::parse(Span::new(
        "====\nHere's a sample AsciiDoc document:\n\n----\n= Document Title\nAuthor Name\n\nContent goes here.\n----\n\nThe document header is useful, but not required.\n====\n",
    ))
    .unwrap_if_no_warnings()
    .unwrap()
    .item;

    assert_eq!(
        block,
        TBlock::CompoundDelimited(
            TCompoundDelimitedBlock {
                blocks: vec![
                    TBlock::Simple(
                        TSimpleBlock {
                            inline: TInline::Uninterpreted(
                                TSpan {
                                    data: "Here's a sample AsciiDoc document:",
                                    line: 2,
                                    col: 1,
                                    offset: 5,
                                },
                            ),
                            source: TSpan {
                                data: "Here's a sample AsciiDoc document:\n",
                                line: 2,
                                col: 1,
                                offset: 5,
                            },
                            title: None,
                                        attrlist: None,
                        },
                    ),
                    TBlock::RawDelimited(
                        TRawDelimitedBlock {
                            lines: vec![
                                TSpan {
                                    data: "= Document Title",
                                    line: 5,
                                    col: 1,
                                    offset: 46,
                                },
                                TSpan {
                                    data: "Author Name",
                                    line: 6,
                                    col: 1,
                                    offset: 63,
                                },
                                TSpan {
                                    data: "",
                                    line: 7,
                                    col: 1,
                                    offset: 75,
                                },
                                TSpan {
                                    data: "Content goes here.",
                                    line: 8,
                                    col: 1,
                                    offset: 76,
                                },
                            ],
                            content_model: ContentModel::Verbatim,
                            context: "listing",
                            source: TSpan {
                                data: "----\n= Document Title\nAuthor Name\n\nContent goes here.\n----\n",
                                line: 4,
                                col: 1,
                                offset: 41,
                            },
                            title: None,
                                        attrlist: None,
                        },
                    ),
                    TBlock::Simple(
                        TSimpleBlock{
                            inline: TInline::Uninterpreted(
                                TSpan {
                                    data: "The document header is useful, but not required.",
                                    line: 11,
                                    col: 1,
                                    offset: 101,
                                },
                            ),
                            source: TSpan {
                                data: "The document header is useful, but not required.\n",
                                line: 11,
                                col: 1,
                                offset: 101,
                            },
                            title: None,
                                        attrlist: None,
                        },
                    ),
                ],
                context: "example",
                source: TSpan {
                    data: "====\nHere's a sample AsciiDoc document:\n\n----\n= Document Title\nAuthor Name\n\nContent goes here.\n----\n\nThe document header is useful, but not required.\n====\n",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                title: None,
                            attrlist: None,
            },
        )
    );

    // When nesting a delimited block that uses the same structural container,
    // it's necessary to vary the length of the delimiter lines (i.e., make
    // the length of the delimiter lines for the child block different than
    // the length of the delimiter lines for the parent block). Varying the
    // delimiter line length allows the parser to distinguish one block from
    // another.

    // ----
    // ====
    // Here are your options:

    // .Red Pill
    // [%collapsible]
    // ======
    // Escape into the real world.
    // ======

    // .Blue Pill
    // [%collapsible]
    // ======
    // Live within the simulated reality without want or fear.
    // ======
    // ====
    // ----

    if false {
        todo!("Parse role attribute lines");
    }

    // The delimiter length for the nested structural container can either be
    // shorter or longer than the parent. That's a personal style choice.
}
