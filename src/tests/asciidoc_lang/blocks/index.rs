//! Tracks https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang/-/blob/main/docs/modules/blocks/pages/index.adoc?ref_type=heads
//!
//! Tracking commit 447cfd22, current as of 2024-02-10.

// = Blocks
//
// Block elements form the main structure of an AsciiDoc document, starting with
// the document itself.

#[ignore]
#[test]
fn what_is_a_block() {
    // == What is a block?
    //
    // A block element (aka block) is a discrete, line-oriented chunk of content in
    // an AsciiDoc document. Once parsed, that chunk of content becomes a block
    // element in the parsed document model. Certain blocks may contain other
    // blocks, so we say that blocks can be nested. The converter visits each
    // block in turn, in document order, converting it to a corresponding chunk of
    // output.

    todo!("Verify block model.");
    // NOT SURE this needs a test, really.
}

#[ignore]
#[test]
fn block_forms() {
    // == Block forms
    //
    // How the boundaries of a block are defined in the AsciiDoc syntax varies.
    // The boundaries of some blocks, like lists, paragraphs, and block macro, are
    // implicit. Other blocks have boundaries that are explicitly marked using
    // delimiters (i.e., delimited blocks). The main commonality is that a block
    // is always line-oriented.
    //
    // A _paragraph block_ is defined as a discrete set of contiguous (non-empty)
    // lines. A _delimited block_ is bounded by delimiter lines.
    // A _section block_ (aka section) is defined by a section title that's prefixed
    // by one or more equal signs. The section includes all content that follows
    // the section title line until the next sibling or parent section title or the
    // document boundary. A _list block_ is defined by a group of sibling list
    // items, each denoted by a marker. A _description list_ block is defined by
    // a sibling group of list items, each denoted by one or more terms.
    // A _block macro_ is defined by a single line that matches the block macro
    // syntax. And the _document_ is also a block.
    //
    // A block (including its metadata lines) should always be bounded by an empty
    // line or document boundary on either side.
    //
    // Whether or not a block supports nested blocks depends on content model of the
    // block (and what the syntax allows).

    todo!("Redundant: Covered by content_model test below.");
}

mod content_model {
    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        Span,
    };

    // == Content model
    //
    // The content model of a block determines what kind of content the block can
    // have (if any) and how that content is processed. The content models of blocks
    // in AsciiDoc are as follows:

    #[ignore]
    #[test]
    fn compound() {
        // compound:: a block that may only contain other blocks (e.g., a section)
        todo!("Add test once a compound block is supported");
    }

    #[test]
    fn simple() {
        // simple:: a block that's treated as contiguous lines of paragraph text (and
        // subject to normal substitutions) (e.g., a paragraph block)

        let block = Block::parse(Span::new("abc")).unwrap();
        assert_eq!(block.t.content_model(), ContentModel::Simple);
    }

    #[ignore]
    #[test]
    fn verbatim() {
        // verbatim:: a block that holds verbatim text (displayed "`as is`") (and
        // subject to verbatim substitutions) (e.g., a listing block)
        todo!("Add test once a verbatim block is supported");
    }

    #[ignore]
    #[test]
    fn raw() {
        // raw:: a block that holds unprocessed content passed directly through to the
        // output with no substitutions applied (e.g., a passthrough block)
        todo!("Add test once a raw block is supported");
    }

    #[ignore]
    #[test]
    fn empty() {
        // empty:: a block that has no content (e.g., an image block)
        todo!("Add test once an empty block is supported");
    }

    #[ignore]
    #[test]
    fn table() {
        // table:: a special content model reserved for tables that
        // enforces a fixed structure
        todo!("Add test once a table block is supported");
    }

    #[ignore]
    #[test]
    fn custom() {
        // The content model is inferred for all built-in syntax (as determined by the
        // context), but can be configured for custom blocks. Blocks may also support
        // different content models under different circumstances. The circumstance is
        // determined by the context and style, and in the case of a delimited block,
        // the structural container as well.
        todo!("Add test once custom blocks are supported");
    }
}

mod context {
    use std::ops::Deref;

    use crate::{
        blocks::{Block, ContentModel, IsBlock},
        Span,
    };

    // == Context
    //
    // You may often hear a block referred to by a name, such as an example
    // block, a sidebar block, an admonition block, or a section. That name
    // is the block's context.

    #[test]
    fn section_context() {
        // Let's consider the following normal section:
        //
        // ----
        // == Section Title
        //
        // Content of section.
        // ----
        //
        // The context of this block is `section`. We often refer to this as a section (or section block), using the context as an adjective to describe the block. The writer does not have to specify the context in this case since it's implied by the syntax.
        //
        // Every block has a context. The context is often implied by the syntax, but can be declared explicitly in certain cases. The context is what distinguishes one kind of block from another. You can think of the context as the block's type.

        let pr = Block::parse(Span::new("== Section Title\n\nContent of section.")).unwrap();

        assert_eq!(pr.t.context().deref(), "section");
    }

    #[test]
    #[ignore]
    fn block_style() {
        // The context can be further modified using a block style to create a
        // family of blocks that share a common type, as is the case with
        // admonition blocks and sections. We'll cover that modifier shortly.

        todo!("Redundant: Covered by block_style test below.");
    }

    #[test]
    #[ignore]
    fn block_name() {
        // For blocks, the context is sometimes referred to as the block name.
        // This comes up in particular when talking about custom blocks.
        // The block name is just another layer of abstraction.
        // All the built-in block names map to exactly one context.
        // But a block extension can map an arbitrary block name to one or more
        // contexts. Which context is ultimately used depends on what is returned
        // from the extension's process method. In the end, it's the context
        // that determines how the block is converted.

        todo!("I don't understand block names. Will add test later.");
    }

    #[test]
    fn sections_are_compound() {
        // The context often determines the content model.
        // For example, all sections implicitly have the compound content model
        // because a section may only contain other blocks.

        let pr = Block::parse(Span::new("== Section Title\n\nContent of section.")).unwrap();

        assert_eq!(pr.t.content_model(), ContentModel::Compound);
    }

    #[test]
    #[ignore]
    fn literal_blocks_are_verbatim() {
        // All literal blocks
        // implicitly have the verbatim content model because the purpose of
        // this block is to present verbatim output.

        todo!("Literal blocks aren't supported yet. Write test when ready.");
    }

    #[test]
    #[ignore]
    fn built_in_contexts() {
        // === Summary of built-in contexts
        //
        // Here's a list of the contexts of all the built-in blocks in AsciiDoc.
        //
        // NOTE: In the Asciidoctor API, the contexts are represented as symbols.
        // In Ruby, a symbol is a name prefixed with a colon (e.g., `:listing`).
        // This documentation will sometimes use this notation when referring to the
        // name of a context. However, this notation is not universal.
        // Some processors, such as Asciidoctor.js, store the context as a string
        // instead.
        //
        // .Built-in contexts
        // [#table-of-contexts,cols="1s,2"]
        // |===
        // |Name | Purpose
        //
        // |admonition
        // |One of five admonition blocks.
        //
        // |audio
        // |An audio block.
        //
        // |colist
        // |A callout list.
        //
        // |dlist
        // |A description list.
        //
        // |document
        // |The top-level document or the document in an AsciiDoc table cell
        //
        // |example
        // |An example block.
        //
        // |floating_title
        // |A discrete heading.
        //
        // |image
        // |An image block.
        //
        // |list_item
        // |An item in an ordered, unordered, or description list (only relevant
        // inside a list or description list block). In a description list, this
        // block is used to represent the term and the description.
        //
        // |listing
        // |A listing block.
        //
        // |literal
        // |A literal block.
        //
        // |olist
        // |An ordered list.
        //
        // |open
        // |An open block.
        //
        // |page_break
        // |A page break.
        //
        // |paragraph
        // |A paragraph.
        //
        // |pass
        // |A passthrough block.
        //
        // |preamble
        // |The preamble of the document.
        //
        // |quote
        // |A quote block (aka blockquote).
        //
        // |section
        // |A section.
        // May also be a part, chapter, or special section.
        //
        // |sidebar
        // |A sidebar block.
        //
        // |table
        // |A table block.
        //
        // |table_cell
        // |A table cell (only relevant inside a table block).
        //
        // |thematic_break
        // |A thematic break (aka horizontal rule).
        //
        // |toc
        // |A TOC block (to designate custom TOC placement).
        //
        // |ulist
        // |An unordered list.
        //
        // |verse
        // |A verse block.
        //
        // |video
        // |A video block.
        // |===

        todo!("Add coverage for built-in context types");
    }

    #[test]
    #[ignore]
    fn inline_context() {
        // NOTE: Each inline element also has a context, but those elements are not
        // (yet) accessible from the parsed document model.

        todo!("Spec doesn't yet describe what inline contexts mean.");
    }

    #[test]
    #[ignore]
    fn additional_contexts_via_extensions() {
        // Additional contexts may be introduced through the use of the block, block
        // macro, or inline macro extension points.

        todo!("Extension points are not yet defined.");
    }

    #[test]
    fn contexts_used_by_converter() {
        // === Contexts used by the converter
        //
        // The context is what the converter uses to dispatch to a convert
        // method. The style is then used by the converter to apply
        // special behavior to blocks of the same family.
        //
        // With two exceptions, there's a 1-to-1 mapping between the contexts
        // and the handler methods of a converter. Those exceptions are
        // the `list_item` and `table_cell` contexts, which are not
        // mapped to a handler method. In the converter, these blocks
        // must be accessed from their parent block.

        // NO-OP: This crate isn't a converter, so this part of the spec doesn't
        // apply.
    }
}

mod block_style {
    // [#block-style]
    // == Block style
    //
    // The context does not always tell the whole story of a block's identity.
    // Some blocks require specialization.
    // That's where the block style comes into play.
    //
    // Above some blocks, you may notice a name at the start of the block
    // attribute list (e.g., `[source]` or `[verse]`). The first positional
    // (unnamed) attribute in the block attribute list is used to declare
    // the block style.
    //
    // The declared block style is the value the author supplies.
    // That value is then interpreted and resolved.
    // The resolved block style, if non-empty, specializes the block's context.
    // (It may instead, or in addition to, alter the block's context).
    //
    // Consider the following example of a source block:
    //
    // [source]
    // ....
    // [source,ruby]
    // ----
    // puts "Hello, World!"
    // ----
    // ....
    //
    // The context of a source block is `listing` (as inferred from the block
    // delimiters) and the style is `source` (as specified by the writer).
    // We say that the style specializes the block as a source block.
    // (Technically, the presence of a source language already implies the
    // `source` style, but under the covers this is what's happening). The
    // context of the block is still the same, but it has additional
    // metadata to indicate that it requires special processing.
    //
    // We also see the block style used for other purposes.
    // The `appendix` block style (e.g., `[appendix]`) above the section title
    // specializes the section as an appendix (a special section) and thus has
    // special semantics and behavior. In the model, the section's style is
    // dually stored as the `sectname`. One of the five admonition styles
    // (e.g., `[TIP]`) above an example block transforms the example block
    // into an admonition block with that name (i.e., label). In the model,
    // the admonition style in lowercase is stored in the `name` attribute.
    // A block style (e.g., `[circle]` or `[loweralpha]`) above an unordered
    // or ordered list, respectively, alters the marker used in front of
    // list items when displayed. A block style (e.g., `[qanda]` and
    // `[horizontal]`) above a description list can either change its
    // semantics or layout.
    //
    // The declared block style can be used to change the context of a block,
    // referred to as xref:masquerading.adoc[block masquerading]. Consider the
    // case of this alternate syntax for a listing block using the literal
    // block delimiters.
    //
    // [source]
    // ----
    // [listing]
    // ....
    // a > b
    // ....
    // ----
    //
    // Since the declared block style matches the name of a context, the context
    // of the block becomes `listing` and the resolved block style remains
    // unset. That means the resolved block style differs from the declared
    // block style. To learn more about how to change the context of a block
    // using the declared block style, see xref:masquerading.adoc[].
    //
    // To get a complete picture of a block's identity, you must consider both
    // the context and the style. The resolved style specializes the context
    // to give it special behavior or semantics.
}

#[test]
#[ignore]
fn block_commonalities() {
    // == Block commonalities
    //
    // Blocks are defined using some form of line-oriented syntax.
    // Section blocks begin with a section title line.
    // Delimited blocks are enclosed in a matching pair of delimiter lines.
    // Paragraph blocks must be contiguous lines.
    //
    // All blocks accommodate zero or more lines of metadata stacked linewise
    // directly on top of the block. These lines populate the properties of the
    // block, such as the ID, title, and options. These metadata lines are as
    // follows:
    //
    // * Zero or more block attribute lines (which populate the block's attributes)
    // * An optional block anchor line
    // * An optional block title line (many blocks also support a corresponding
    //   caption)
    // * An optional ID
    // * An optional set of roles
    // * An optional set of options
    //
    // For example, consider a sidebar block with a title and ID:
    //
    // ----
    // .Styles of music
    // [#music-styles]
    // ****
    // Go off on a tangent to describe what a style of music is.
    // ****
    // ----
    //
    // When it comes to processing content, blocks split off into different groups.
    // These groups are primarily associated with the block's content model.
    //
    // Paragraph blocks and verbatim blocks have an implicit and modifiable set of
    // xref:subs:index.adoc[substitutions]. Substitutions do not apply to compound
    // blocks (i.e., blocks that may contain nested blocks).

    todo!("Add support for block metadata");
}
