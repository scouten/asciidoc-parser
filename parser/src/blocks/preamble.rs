#![allow(unused)] // TEMPORARY while building
use std::{fmt, slice::Iter, sync::LazyLock};

use regex::Regex;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        Block, ContentModel, IsBlock, metadata::BlockMetadata, parse_utils::parse_blocks_until,
    },
    content::{Content, SubstitutionGroup},
    document::RefType,
    internal::debug::DebugSliceReference,
    span::MatchedItem,
    strings::CowStr,
    warnings::{Warning, WarningType},
};

/// Content between the end of the document header and the first section title
/// in the document body is called the preamble.
#[derive(Clone, Eq, PartialEq)]
pub struct Preamble<'src> {
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
}

impl<'src> Preamble<'src> {
    pub(crate) fn from_blocks(blocks: Vec<Block<'src>>, source: Span<'src>) -> Self {
        todo!("Assemble source from blocks");
    }
}

impl<'src> IsBlock<'src> for Preamble<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn raw_context(&self) -> CowStr<'src> {
        "preamble".into()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.blocks.iter()
    }

    fn title_source(&'src self) -> Option<Span<'src>> {
        None
    }

    fn title(&self) -> Option<&str> {
        None
    }

    fn anchor(&'src self) -> Option<Span<'src>> {
        None
    }

    fn anchor_reftext(&'src self) -> Option<Span<'src>> {
        None
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        None
    }

    fn id(&'src self) -> Option<&'src str> {
        None
    }
}

impl<'src> HasSpan<'src> for Preamble<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

impl std::fmt::Debug for Preamble<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Preamble")
            .field("blocks", &DebugSliceReference(&self.blocks))
            .field("source", &self.source)
            .finish()
    }
}

/*
#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]
    #![allow(clippy::unwrap_used)]

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{IsBlock, metadata::BlockMetadata},
        tests::prelude::*,
        warnings::WarningType,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        let b1 = crate::blocks::Preamble::parse(
            &BlockMetadata::new("== Preamble Title"),
            &mut parser,
            &mut warnings,
        )
        .unwrap();

        let b2 = b1.item.clone();
        assert_eq!(b1.item, b2);
    }

    #[test]
    fn impl_debug() {
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        let preamble = crate::blocks::Preamble::parse(
            &BlockMetadata::new("== Preamble Title"),
            &mut parser,
            &mut warnings,
        )
        .unwrap()
        .item;

        assert_eq!(
            format!("{preamble:#?}"),
            r#"Preamble {
    level: 1,
    preamble_title: Content {
        original: Span {
            data: "Preamble Title",
            line: 1,
            col: 4,
            offset: 3,
        },
        rendered: "Preamble Title",
    },
    blocks: &[],
    source: Span {
        data: "== Preamble Title",
        line: 1,
        col: 1,
        offset: 0,
    },
    title_source: None,
    title: None,
    anchor: None,
    anchor_reftext: None,
    attrlist: None,
    preamble_type: PreambleType::Normal,
    preamble_id: Some(
        "_preamble_title",
    ),
    preamble_number: None,
}"#
        );
    }
}
*/
