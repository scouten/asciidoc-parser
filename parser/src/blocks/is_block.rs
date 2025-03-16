use std::{fmt::Debug, slice::Iter};

use crate::{
    attributes::Attrlist,
    blocks::{is_built_in_context, Block},
    strings::CowStr,
    HasSpan, Span,
};

/// **Block elements** form the main structure of an AsciiDoc document, starting
/// with the document itself.
///
/// A block element (aka **block**) is a discrete, line-oriented chunk of
/// content in an AsciiDoc document. Once parsed, that chunk of content becomes
/// a block element in the parsed document model. Certain blocks may contain
/// other blocks, so we say that blocks can be nested. The converter visits each
/// block in turn, in document order, converting it to a corresponding chunk of
/// output.
///
/// This trait implements many of the same core methods as the [`Block`] enum
/// but provides a mechanism for third-party code to extend the behavior of
/// blocks.
pub trait IsBlock<'src>: HasSpan<'src> + Clone + Debug + Eq + PartialEq {
    /// Returns the [`ContentModel`] for this block.
    fn content_model(&self) -> ContentModel;

    /// Returns the resolved context for this block.
    ///
    /// A block’s context is also sometimes referred to as a name, such as an
    /// example block, a sidebar block, an admonition block, or a section.
    ///
    /// Every block has a context. The context is often implied by the syntax,
    /// but can be declared explicitly in certain cases. The context is what
    /// distinguishes one kind of block from another. You can think of the
    /// context as the block’s type.
    ///
    /// For that reason, the context is not defined as an enumeration, but
    /// rather as a string type that is optimized for the case where predefined
    /// constants are viable.
    ///
    /// A block's context can be replaced by a block style that matches a
    /// built-in context. Unlike [`raw_context()`], that transformation _is_
    /// performed by this function.
    ///
    /// [`raw_context()`]: Self::raw_context
    fn resolved_context(&'src self) -> CowStr<'src> {
        if let Some(declared_style) = self.declared_style() {
            let declared_style = declared_style.data();
            if is_built_in_context(declared_style) {
                return declared_style.into();
            }
        }

        self.raw_context()
    }

    /// Returns the raw (uninterpreted) context for this block.
    ///
    /// A block’s context is also sometimes referred to as a name, such as an
    /// example block, a sidebar block, an admonition block, or a section.
    ///
    /// Every block has a context. The context is often implied by the syntax,
    /// but can be declared explicitly in certain cases. The context is what
    /// distinguishes one kind of block from another. You can think of the
    /// context as the block’s type.
    ///
    /// For that reason, the context is not defined as an enumeration, but
    /// rather as a string type that is optimized for the case where predefined
    /// constants are viable.
    ///
    /// A block's context can be replaced by a block style that matches a
    /// built-in context. That transformation is only performed by
    /// [`resolved_context()`], not this function.
    ///
    /// [`resolved_context()`]: Self::resolved_context
    fn raw_context(&self) -> CowStr<'src>;

    /// Returns the declared (uninterpreted) style for this block.
    ///
    /// Above some blocks, you may notice a name at the start of the block
    /// attribute list (e.g., `[source]` or `[verse]`). The first positional
    /// (unnamed) attribute in the block attribute list is used to declare the
    /// block style.
    ///
    /// The declared block style is the value the author supplies.
    ///
    /// That value is then interpreted and resolved. That interpretation is not
    /// performed by this function.
    fn declared_style(&'src self) -> Option<Span<'src>> {
        self.attrlist()
            .and_then(|attrlist| attrlist.nth_attribute(1))
            .and_then(|attr| attr.block_style())
    }

    /// Returns an iterator over the nested blocks contained within
    /// this block.
    ///
    /// Many block types do not have nested blocks so the default implementation
    /// returns an empty iterator.
    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        const NO_BLOCKS: &[Block<'static>] = &[];
        NO_BLOCKS.iter()
    }

    /// Returns the ID for this block, if present.
    ///
    /// You can assign an ID to a block using the shorthand syntax, the longhand
    /// syntax, or a legacy block anchor.
    ///
    /// In the shorthand syntax, you prefix the name with a hash (`#`) in the
    /// first position attribute:
    ///
    /// ```asciidoc
    /// [#goals]
    /// * Goal 1
    /// * Goal 2
    /// ```
    ///
    /// In the longhand syntax, you use a standard named attribute:
    ///
    /// ```asciidoc
    /// [id=goals]
    /// * Goal 1
    /// * Goal 2
    /// ```
    ///
    /// In the legacy block anchor syntax, you surround the name with double
    /// square brackets:
    ///
    /// ```asciidoc
    /// [[goals]]
    /// * Goal 1
    /// * Goal 2
    /// ```
    fn id(&'src self) -> Option<Span<'src>> {
        self.attrlist().and_then(|attrlist| attrlist.id())
    }

    /// Returns any role attributes that were found.
    ///
    /// You can assign one or more roles to blocks and most inline elements
    /// using the `role` attribute. The `role` attribute is a [named attribute].
    /// Even though the attribute name is singular, it may contain multiple
    /// (space-separated) roles. Roles may also be defined using a shorthand
    /// (dot-prefixed) syntax.
    ///
    /// A role:
    /// 1. adds additional semantics to an element
    /// 2. can be used to apply additional styling to a group of elements (e.g.,
    ///    via a CSS class selector)
    /// 3. may activate additional behavior if recognized by the converter
    ///
    /// **TIP:** The `role` attribute in AsciiDoc always get mapped to the
    /// `class` attribute in the HTML output. In other words, role names are
    /// synonymous with HTML class names, thus allowing output elements to be
    /// identified and styled in CSS using class selectors (e.g.,
    /// `sidebarblock.role1`).
    ///
    /// [named attribute]: https://docs.asciidoctor.org/asciidoc/latest/attributes/positional-and-named-attributes/#named
    fn roles(&'src self) -> Vec<Span<'src>> {
        match self.attrlist() {
            Some(attrlist) => attrlist.roles(),
            None => vec![],
        }
    }

    /// Returns the title for this block, if present.
    fn title(&'src self) -> Option<Span<'src>>;

    /// Returns the attribute list for this block, if present.
    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>>;
}

/// The content model of a block determines what kind of content the block can
/// have (if any) and how that content is processed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContentModel {
    /// A block that may only contain other blocks (e.g., a section)
    Compound,

    /// A block that's treated as contiguous lines of paragraph text (and
    /// subject to normal substitutions) (e.g., a paragraph block)
    Simple,

    /// A block that holds verbatim text (displayed "as is") (and subject to
    /// verbatim substitutions) (e.g., a listing block)
    Verbatim,

    /// A block that holds unprocessed content passed directly through to the
    /// output with no substitutions applied (e.g., a passthrough block)
    Raw,

    /// A block that has no content (e.g., an image block)
    Empty,

    /// A special content model reserved for tables that enforces a fixed
    /// structure
    Table,
}
