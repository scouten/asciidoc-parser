use std::slice::Iter;

use crate::{
    HasSpan, Span,
    attributes::Attrlist,
    blocks::{Block, ContentModel, IsBlock},
    internal::debug::DebugSliceReference,
    strings::CowStr,
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
        let preamble_source = if let Some(last_block) = blocks.last() {
            let after_last = last_block.span().discard_all();
            source.trim_remainder(after_last)
        } else {
            source.trim_remainder(source)
        };

        Self {
            blocks,
            source: preamble_source,
        }
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

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]
    #![allow(clippy::unwrap_used)]

    use pretty_assertions_sorted::assert_eq;

    use crate::{Parser, blocks::IsBlock};

    fn doc_fixture() -> crate::Document<'static> {
        Parser::default().parse("= Document Title\n\nSome early words go here.\n\n== First Section")
    }

    fn fixture_preamble<'src>(
        doc: &'src crate::Document<'src>,
    ) -> &'src crate::blocks::Block<'src> {
        doc.nested_blocks().next().unwrap()
    }

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let doc = doc_fixture();

        let b1 = fixture_preamble(&doc);
        let b2 = b1.clone();

        assert_eq!(b1, &b2);
    }

    #[test]
    fn impl_debug() {
        let doc = doc_fixture();
        let preamble = fixture_preamble(&doc);

        let crate::blocks::Block::Preamble(preamble) = preamble else {
            panic!("Unexpected block: {preamble:#?}");
        };

        dbg!(&preamble);

        assert_eq!(
            format!("{preamble:#?}"),
            r#"Preamble {
    blocks: &[
        Block::Simple(
            SimpleBlock {
                content: Content {
                    original: Span {
                        data: "Some early words go here.",
                        line: 3,
                        col: 1,
                        offset: 18,
                    },
                    rendered: "Some early words go here.",
                },
                source: Span {
                    data: "Some early words go here.",
                    line: 3,
                    col: 1,
                    offset: 18,
                },
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            },
        ),
    ],
    source: Span {
        data: "Some early words go here.",
        line: 3,
        col: 1,
        offset: 18,
    },
}"#
        );
    }
}
