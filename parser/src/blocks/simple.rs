use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        CompoundDelimitedBlock, ContentModel, IsBlock, ListItemMarker, RawDelimitedBlock,
        metadata::BlockMetadata,
    },
    content::{Content, SubstitutionGroup},
    span::MatchedItem,
    strings::CowStr,
};

/// The style of a simple block.
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SimpleBlockStyle {
    /// A paragraph block with normal substitutions.
    Paragraph,

    /// A literal block with no substitutions.
    Literal,

    /// Blocks and paragraphs assigned the listing style display their rendered
    /// content exactly as you see it in the source. Listing content is
    /// converted to preformatted text (i.e., `<pre>`). The content is presented
    /// in a fixed-width font and endlines are preserved. Only [special
    /// characters] and callouts are replaced when the document is converted.
    ///
    /// [special characters]: https://docs.asciidoctor.org/asciidoc/latest/subs/special-characters/
    Listing,

    /// A source block is a specialization of a listing block. Developers are
    /// accustomed to seeing source code colorized to emphasize the code’s
    /// structure (i.e., keywords, types, delimiters, etc.).
    Source,
}

impl std::fmt::Debug for SimpleBlockStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SimpleBlockStyle::Paragraph => write!(f, "SimpleBlockStyle::Paragraph"),
            SimpleBlockStyle::Literal => write!(f, "SimpleBlockStyle::Literal"),
            SimpleBlockStyle::Listing => write!(f, "SimpleBlockStyle::Listing"),
            SimpleBlockStyle::Source => write!(f, "SimpleBlockStyle::Source"),
        }
    }
}

/// A block that's treated as contiguous lines of paragraph text (and subject to
/// normal substitutions) (e.g., a paragraph block).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SimpleBlock<'src> {
    content: Content<'src>,
    source: Span<'src>,
    style: SimpleBlockStyle,
    title_source: Option<Span<'src>>,
    title: Option<String>,
    anchor: Option<Span<'src>>,
    anchor_reftext: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> SimpleBlock<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let MatchedItem {
            item: (content, style),
            after,
        } = parse_lines(metadata.block_start, &metadata.attrlist, false, parser)?;

        Some(MatchedItem {
            item: Self {
                content,
                source: metadata
                    .source
                    .trim_remainder(after)
                    .trim_trailing_whitespace(),
                style,
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                anchor_reftext: metadata.anchor_reftext,
                attrlist: metadata.attrlist.clone(),
            },
            after: after.discard_empty_lines(),
        })
    }

    pub(crate) fn parse_for_list_item(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let MatchedItem {
            item: (content, style),
            after,
        } = parse_lines(metadata.block_start, &metadata.attrlist, true, parser)?;

        Some(MatchedItem {
            item: Self {
                content,
                source: metadata
                    .source
                    .trim_remainder(after)
                    .trim_trailing_whitespace(),
                style,
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                anchor_reftext: metadata.anchor_reftext,
                attrlist: metadata.attrlist.clone(),
            },
            after,
        })
    }

    pub(crate) fn parse_fast(
        source: Span<'src>,
        parser: &Parser,
    ) -> Option<MatchedItem<'src, Self>> {
        let MatchedItem {
            item: (content, style),
            after,
        } = parse_lines(source, &None, false, parser)?;

        let source = content.original();

        Some(MatchedItem {
            item: Self {
                content,
                source,
                style,
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            },
            after: after.discard_empty_lines(),
        })
    }

    /// Return the interpreted content of this block.
    pub fn content(&self) -> &Content<'src> {
        &self.content
    }

    /// Return the style of this block.
    pub fn style(&self) -> SimpleBlockStyle {
        self.style
    }
}

/// Parse the content-bearing lines for this block.
fn parse_lines<'src>(
    source: Span<'src>,
    attrlist: &Option<Attrlist<'src>>,
    mut stop_for_list_items: bool,
    parser: &Parser,
) -> Option<MatchedItem<'src, (Content<'src>, SimpleBlockStyle)>> {
    let source_after_whitespace = source.discard_whitespace();
    let strip_indent = source_after_whitespace.col() - 1;

    let mut style = if source_after_whitespace.col() == source.col() {
        SimpleBlockStyle::Paragraph
    } else {
        stop_for_list_items = false;
        SimpleBlockStyle::Literal
    };

    // Block style can override the interpretation of literal from reading
    // indentation.
    if let Some(attrlist) = attrlist {
        match attrlist.block_style() {
            Some("normal") => {
                style = SimpleBlockStyle::Paragraph;
            }

            Some("literal") => {
                stop_for_list_items = false;
                style = SimpleBlockStyle::Literal;
            }

            Some("listing") => {
                stop_for_list_items = false;
                style = SimpleBlockStyle::Listing;
            }

            Some("source") => {
                stop_for_list_items = false;
                style = SimpleBlockStyle::Source;
            }

            _ => {}
        }
    }

    let mut next = source;
    let mut filtered_lines: Vec<&'src str> = vec![];

    while let Some(line_mi) = next.take_non_empty_line() {
        let mut line = line_mi.item;

        // There are several stop conditions for simple paragraph blocks. These
        // "shouldn't" be encountered on the first line (we shouldn't be calling
        // `SimpleBlock::parse` in these conditions), but in case it is, we simply
        // ignore them on the first line.
        if !filtered_lines.is_empty() {
            if stop_for_list_items && let Some(_marker) = ListItemMarker::parse(line, parser) {
                break;
            }

            if line.data() == "+" {
                break;
            }

            if line.starts_with('[') && line.ends_with(']') {
                break;
            }

            if (line.starts_with('/')
                || line.starts_with('-')
                || line.starts_with('.')
                || line.starts_with('+')
                || line.starts_with('=')
                || line.starts_with('*')
                || line.starts_with('_'))
                && (RawDelimitedBlock::is_valid_delimiter(&line)
                    || CompoundDelimitedBlock::is_valid_delimiter(&line))
            {
                break;
            }
        }

        next = line_mi.after;

        if line.starts_with("//") && !line.starts_with("///") {
            continue;
        }

        // Strip at most the number of leading whitespace characters found on the first
        // line.
        if strip_indent > 0
            && let Some(n) = line.position(|c| c != ' ' && c != '\t')
        {
            line = line.into_parse_result(n.min(strip_indent)).after;
        };

        filtered_lines.push(line.trim_trailing_whitespace().data());
    }

    let source = source.trim_remainder(next).trim_trailing_whitespace();
    if source.is_empty() {
        return None;
    }

    let filtered_lines = filtered_lines.join("\n");
    let mut content: Content<'src> = Content::from_filtered(source, filtered_lines);

    let sub_group = match style {
        // Only apply Verbatim substitutions to literal blocks detected by indentation.
        // Listing and Source styles declared via attribute list still use Normal subs.
        SimpleBlockStyle::Literal => SubstitutionGroup::Verbatim,
        SimpleBlockStyle::Listing | SimpleBlockStyle::Source | SimpleBlockStyle::Paragraph => {
            SubstitutionGroup::Normal
        }
    };

    sub_group.override_via_attrlist(attrlist.as_ref()).apply(
        &mut content,
        parser,
        attrlist.as_ref(),
    );

    Some(MatchedItem {
        item: (content, style),
        after: next,
    })
}

impl<'src> IsBlock<'src> for SimpleBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Simple
    }

    fn rendered_content(&self) -> Option<&str> {
        Some(self.content.rendered())
    }

    fn raw_context(&self) -> CowStr<'src> {
        "paragraph".into()
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
        self.anchor_reftext
    }

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for SimpleBlock<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used)]

    use std::ops::Deref;

    use pretty_assertions_sorted::assert_eq;

    use crate::{
        Parser,
        blocks::{ContentModel, IsBlock, SimpleBlockStyle, metadata::BlockMetadata},
        content::SubstitutionGroup,
        tests::prelude::*,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let mut parser = Parser::default();

        let b1 =
            crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc"), &mut parser).unwrap();

        let b2 = b1.item.clone();
        assert_eq!(b1.item, b2);
    }

    #[test]
    fn empty_source() {
        let mut parser = Parser::default();
        assert!(crate::blocks::SimpleBlock::parse(&BlockMetadata::new(""), &mut parser).is_none());
    }

    #[test]
    fn only_spaces() {
        let mut parser = Parser::default();
        assert!(
            crate::blocks::SimpleBlock::parse(&BlockMetadata::new("    "), &mut parser).is_none()
        );
    }

    #[test]
    fn single_line() {
        let mut parser = Parser::default();
        let mi =
            crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc"), &mut parser).unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
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
                style: SimpleBlockStyle::Paragraph,
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            },
        );

        assert_eq!(mi.item.content_model(), ContentModel::Simple);
        assert_eq!(mi.item.rendered_content().unwrap(), "abc");
        assert_eq!(mi.item.raw_context().deref(), "paragraph");
        assert_eq!(mi.item.resolved_context().deref(), "paragraph");
        assert!(mi.item.declared_style().is_none());
        assert!(mi.item.id().is_none());
        assert!(mi.item.roles().is_empty());
        assert!(mi.item.options().is_empty());
        assert!(mi.item.title_source().is_none());
        assert!(mi.item.title().is_none());
        assert!(mi.item.anchor().is_none());
        assert!(mi.item.anchor_reftext().is_none());
        assert!(mi.item.attrlist().is_none());
        assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );
    }

    #[test]
    fn multiple_lines() {
        let mut parser = Parser::default();
        let mi = crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc\ndef"), &mut parser)
            .unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
                content: Content {
                    original: Span {
                        data: "abc\ndef",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    rendered: "abc\ndef",
                },
                source: Span {
                    data: "abc\ndef",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                style: SimpleBlockStyle::Paragraph,
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "",
                line: 2,
                col: 4,
                offset: 7
            }
        );

        assert_eq!(mi.item.rendered_content().unwrap(), "abc\ndef");
    }

    #[test]
    fn consumes_blank_lines_after() {
        let mut parser = Parser::default();
        let mi = crate::blocks::SimpleBlock::parse(&BlockMetadata::new("abc\n\ndef"), &mut parser)
            .unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
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
                style: SimpleBlockStyle::Paragraph,
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: None,
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 3,
                col: 1,
                offset: 5
            }
        );
    }

    #[test]
    fn overrides_sub_group_via_subs_attribute() {
        let mut parser = Parser::default();
        let mi = crate::blocks::SimpleBlock::parse(
            &BlockMetadata::new("[subs=quotes]\na<b>c *bold*\n\ndef"),
            &mut parser,
        )
        .unwrap();

        assert_eq!(
            mi.item,
            SimpleBlock {
                content: Content {
                    original: Span {
                        data: "a<b>c *bold*",
                        line: 2,
                        col: 1,
                        offset: 14,
                    },
                    rendered: "a<b>c <strong>bold</strong>",
                },
                source: Span {
                    data: "[subs=quotes]\na<b>c *bold*",
                    line: 1,
                    col: 1,
                    offset: 0,
                },
                style: SimpleBlockStyle::Paragraph,
                title_source: None,
                title: None,
                anchor: None,
                anchor_reftext: None,
                attrlist: Some(Attrlist {
                    attributes: &[ElementAttribute {
                        name: Some("subs"),
                        value: "quotes",
                        shorthand_items: &[],
                    },],
                    anchor: None,
                    source: Span {
                        data: "subs=quotes",
                        line: 1,
                        col: 2,
                        offset: 1,
                    },
                },),
            }
        );

        assert_eq!(
            mi.after,
            Span {
                data: "def",
                line: 4,
                col: 1,
                offset: 28
            }
        );

        assert_eq!(
            mi.item.rendered_content().unwrap(),
            "a<b>c <strong>bold</strong>"
        );
    }
}
