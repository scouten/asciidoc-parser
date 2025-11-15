use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{ContentModel, IsBlock, metadata::BlockMetadata},
    span::MatchedItem,
    strings::CowStr,
};

/// A break block is used to represent a thematic or page break macro.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Break<'src> {
    type_: BreakType,
    source: Span<'src>,
    title_source: Option<Span<'src>>,
    title: Option<String>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

/// A break may be one of two different types.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum BreakType {
    /// A thematic break (aka horizontal rule).
    Thematic,

    /// A hint to the converter to insert a page break.
    Page,
}

impl std::fmt::Debug for BreakType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BreakType::Thematic => write!(f, "BreakType::Thematic"),
            BreakType::Page => write!(f, "BreakType::Page"),
        }
    }
}

impl<'src> Break<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        _parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let line = metadata.block_start.take_normalized_line();

        let type_ = match line.item.data() {
            "'''" | "---" | "- - -" | "***" | "* * *" => BreakType::Thematic,
            "<<<" => BreakType::Page,
            _ => {
                return None;
            }
        };

        let source: Span = metadata.source.trim_remainder(line.after);
        let source = source.slice(0..source.trim().len());

        Some(MatchedItem {
            item: Self {
                type_,
                source,
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                attrlist: metadata.attrlist.clone(),
            },

            after: line.after.discard_empty_lines(),
        })
    }

    /// Return a [`Span`] describing the macro name.
    pub fn type_(&self) -> BreakType {
        self.type_
    }
}

impl<'src> IsBlock<'src> for Break<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Empty
    }

    fn raw_context(&self) -> CowStr<'src> {
        match self.type_ {
            BreakType::Thematic => "thematic_break",
            BreakType::Page => "page_break",
        }
        .into()
    }

    fn title_source(&'src self) -> Option<Span<'src>> {
        self.title_source
    }

    fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        self.anchor
    }

    fn anchor_reftext(&'src self) -> Option<Span<'src>> {
        None
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for Break<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}
