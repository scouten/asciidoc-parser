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

/// Sections partition the document into a content hierarchy. A section is an
/// implicit enclosure. Each section begins with a title and ends at the next
/// sibling section, ancestor section, or end of document. Nested section levels
/// must be sequential.
#[derive(Clone, Eq, PartialEq)]
pub struct SectionBlock<'src> {
    level: usize,
    section_title: Content<'src>,
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
    title_source: Option<Span<'src>>,
    title: Option<String>,
    anchor: Option<Span<'src>>,
    anchor_reftext: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
    section_id: Option<String>,
    section_number: Option<SectionNumber>,
}

impl<'src> SectionBlock<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
        warnings: &mut Vec<Warning<'src>>,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.discard_empty_lines();
        let level_and_title = parse_title_line(source, warnings)?;

        // Take a snapshot of `sectids` value before reading child blocks because
        // the value might be altered while parsing.
        let sectids = parser.is_attribute_set("sectids");

        let mut most_recent_level = level_and_title.item.0;

        let mut maw_blocks = parse_blocks_until(
            level_and_title.after,
            |i| {
                peer_or_ancestor_section(
                    *i,
                    level_and_title.item.0,
                    &mut most_recent_level,
                    warnings,
                )
            },
            parser,
        );

        let blocks = maw_blocks.item;
        let source = metadata.source.trim_remainder(blocks.after);

        let mut section_title = Content::from(level_and_title.item.1);
        SubstitutionGroup::Title.apply(&mut section_title, parser, metadata.attrlist.as_ref());

        let proposed_base_id = generate_section_id(section_title.rendered(), parser);

        let manual_id = metadata
            .attrlist
            .as_ref()
            .and_then(|a| a.id())
            .or_else(|| metadata.anchor.as_ref().map(|anchor| anchor.data()));

        let reftext = metadata
            .attrlist
            .as_ref()
            .and_then(|a| a.named_attribute("reftext").map(|a| a.value()))
            .unwrap_or_else(|| section_title.rendered());

        let section_id = if let Some(catalog) = parser.catalog_mut() {
            if sectids && manual_id.is_none() {
                Some(catalog.generate_and_register_unique_id(
                    &proposed_base_id,
                    Some(reftext),
                    RefType::Section,
                ))
            } else {
                if let Some(manual_id) = manual_id
                    && catalog
                        .register_ref(manual_id, Some(reftext), RefType::Section)
                        .is_err()
                {
                    warnings.push(Warning {
                        source: metadata.source.trim_remainder(level_and_title.after),
                        warning: WarningType::DuplicateId(manual_id.to_string()),
                    });
                }

                None
            }
        } else {
            None
        };

        let level = level_and_title.item.0;

        let section_number = if parser.is_attribute_set("sectnums") && parser.sectnumlevels <= level
        {
            Some(parser.assign_section_number(level))
        } else {
            None
        };

        warnings.append(&mut maw_blocks.warnings);

        Some(MatchedItem {
            item: Self {
                level,
                section_title,
                blocks: blocks.item,
                source: source.trim_trailing_whitespace(),
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                anchor_reftext: metadata.anchor_reftext,
                attrlist: metadata.attrlist.clone(),
                section_id,
                section_number,
            },
            after: blocks.after,
        })
    }

    /// Return the section's level.
    ///
    /// The section title must be prefixed with a section marker, which
    /// indicates the section level. The number of equal signs in the marker
    /// represents the section level using a 0-based index (e.g., two equal
    /// signs represents level 1). A section marker can range from two to six
    /// equal signs and must be followed by a space.
    ///
    /// This function will return an integer between 1 and 5.
    pub fn level(&self) -> usize {
        self.level
    }

    /// Return a [`Span`] containing the section title source.
    pub fn section_title_source(&self) -> Span<'src> {
        self.section_title.original()
    }

    /// Return the processed section title after substitutions have been
    /// applied.
    pub fn section_title(&'src self) -> &'src str {
        self.section_title.rendered()
    }

    /// Accessor intended to be used for testing only. Use the `id()` accessor
    /// in the `IsBlock` trait to retrieve the effective ID for this block,
    /// which considers both auto-generated IDs and manually-set IDs.
    #[cfg(test)]
    pub(crate) fn section_id(&'src self) -> Option<&'src str> {
        self.section_id.as_deref()
    }

    /// Return the section number assigned to this section, if any.
    pub fn section_number(&'src self) -> Option<&'src SectionNumber> {
        self.section_number.as_ref()
    }
}

impl<'src> IsBlock<'src> for SectionBlock<'src> {
    fn content_model(&self) -> ContentModel {
        ContentModel::Compound
    }

    fn raw_context(&self) -> CowStr<'src> {
        "section".into()
    }

    fn nested_blocks(&'src self) -> Iter<'src, Block<'src>> {
        self.blocks.iter()
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

    fn id(&'src self) -> Option<&'src str> {
        // First try the default implementation (explicit IDs from anchor or attrlist)
        self.anchor()
            .map(|a| a.data())
            .or_else(|| self.attrlist().and_then(|attrlist| attrlist.id()))
            // Fall back to auto-generated ID if no explicit ID is set
            .or(self.section_id.as_deref())
    }
}

impl<'src> HasSpan<'src> for SectionBlock<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

impl std::fmt::Debug for SectionBlock<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SectionBlock")
            .field("level", &self.level)
            .field("section_title", &self.section_title)
            .field("blocks", &DebugSliceReference(&self.blocks))
            .field("source", &self.source)
            .field("title_source", &self.title_source)
            .field("title", &self.title)
            .field("anchor", &self.anchor)
            .field("anchor_reftext", &self.anchor_reftext)
            .field("attrlist", &self.attrlist)
            .field("section_id", &self.section_id)
            .field("section_number", &self.section_number)
            .finish()
    }
}

fn parse_title_line<'src>(
    source: Span<'src>,
    warnings: &mut Vec<Warning<'src>>,
) -> Option<MatchedItem<'src, (usize, Span<'src>)>> {
    let mi = source.take_non_empty_line()?;
    let mut line = mi.item;

    let mut count = 0;

    if line.starts_with('=') {
        while let Some(mi) = line.take_prefix("=") {
            count += 1;
            line = mi.after;
        }
    } else {
        while let Some(mi) = line.take_prefix("#") {
            count += 1;
            line = mi.after;
        }
    }

    if count == 1 {
        warnings.push(Warning {
            source: source.take_normalized_line().item,
            warning: WarningType::Level0SectionHeadingNotSupported,
        });

        return None;
    }

    if count > 6 {
        warnings.push(Warning {
            source: source.take_normalized_line().item,
            warning: WarningType::SectionHeadingLevelExceedsMaximum(count - 1),
        });

        return None;
    }

    let title = line.take_required_whitespace()?;

    Some(MatchedItem {
        item: (count - 1, title.after),
        after: mi.after,
    })
}

fn peer_or_ancestor_section<'src>(
    source: Span<'src>,
    level: usize,
    most_recent_level: &mut usize,
    warnings: &mut Vec<Warning<'src>>,
) -> bool {
    // Skip over any block metadata (title, anchor, attrlist) to find the actual
    // section line. We create a temporary parser to avoid modifying the real
    // parser state.
    let mut temp_parser = Parser::default();
    let source_after_metadata = BlockMetadata::parse(source, &mut temp_parser)
        .item
        .block_start;

    if let Some(mi) = parse_title_line(source_after_metadata, warnings) {
        let found_level = mi.item.0;

        if found_level > *most_recent_level + 1 {
            warnings.push(Warning {
                source: source.take_normalized_line().item,
                warning: WarningType::SectionHeadingLevelSkipped(*most_recent_level, found_level),
            });
        }

        *most_recent_level = found_level;

        mi.item.0 <= level
    } else {
        false
    }
}

/// Propose a section ID from the section title.
///
/// This function is called when (1) no `id` attribute is specified explicitly,
/// and (2) the `sectids` document attribute is set.
///
/// The ID is generated as described in the AsciiDoc language definition in [How
/// a section ID is computed].
///
/// [How a section ID is computed](https://docs.asciidoctor.org/asciidoc/latest/sections/auto-ids/)
fn generate_section_id(title: &str, parser: &Parser) -> String {
    let idprefix = parser
        .attribute_value("idprefix")
        .as_maybe_str()
        .unwrap_or_default()
        .to_owned();

    let idseparator = parser
        .attribute_value("idseparator")
        .as_maybe_str()
        .unwrap_or_default()
        .to_owned();

    let mut gen_id = title.to_lowercase().to_owned();

    #[allow(clippy::unwrap_used)]
    static INVALID_SECTION_ID_CHARS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(
            r"<[^>]+>|&lt;[^&]*&gt;|&(?:[a-z][a-z]+\d{0,2}|#\d{2,5}|#x[\da-f]{2,4});|[^ \w\-.]+",
        )
        .unwrap()
    });

    gen_id = INVALID_SECTION_ID_CHARS
        .replace_all(&gen_id, "")
        .to_string();

    // Take only first character of separator if multiple provided.
    let sep = idseparator
        .chars()
        .next()
        .map(|s| s.to_string())
        .unwrap_or_default();

    gen_id = gen_id.replace([' ', '.', '-'], &sep);

    if !sep.is_empty() {
        while gen_id.contains(&format!("{}{}", sep, sep)) {
            gen_id = gen_id.replace(&format!("{}{}", sep, sep), &sep);
        }

        if gen_id.ends_with(&sep) {
            gen_id.pop();
        }

        if idprefix.is_empty() && gen_id.starts_with(&sep) {
            gen_id = gen_id[sep.len()..].to_string();
        }
    }

    format!("{idprefix}{gen_id}")
}

/// Represents an assigned section number.
///
/// Section numbers aren't assigned by default, but can be enabled using the
/// `sectnums` and `sectnumlevels` attributes as described in [Section Numbers].
///
/// [Section Numbers]: https://docs.asciidoctor.org/asciidoc/latest/sections/numbers/
#[derive(Clone, Default, Eq, PartialEq)]
pub struct SectionNumber {
    components: Vec<usize>,
}

impl SectionNumber {
    /// Generate the next section number for the specified level, based on this
    /// section number.
    ///
    /// `level` should be between 1 and 5, though this is not enforced.
    pub(crate) fn assign_next_number(&mut self, level: usize) {
        // Drop any ID components beyond the desired level.
        self.components.truncate(level);

        if self.components.len() < level {
            self.components.resize(level, 1);
        } else if level > 0 {
            self.components[level - 1] += 1;
        }
    }

    /// Iterate over the components of the section number.
    pub fn components(&self) -> &[usize] {
        &self.components
    }
}

impl fmt::Display for SectionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            &self
                .components
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("."),
        )
    }
}

impl fmt::Debug for SectionNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SectionNumber")
            .field("components", &DebugSliceReference(&self.components))
            .finish()
    }
}

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

        let b1 = crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("== Section Title"),
            &mut parser,
            &mut warnings,
        )
        .unwrap();

        let b2 = b1.item.clone();
        assert_eq!(b1.item, b2);
    }

    #[test]
    fn err_empty_source() {
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        assert!(
            crate::blocks::SectionBlock::parse(&BlockMetadata::new(""), &mut parser, &mut warnings)
                .is_none()
        );
    }

    #[test]
    fn err_only_spaces() {
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        assert!(
            crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("    "),
                &mut parser,
                &mut warnings
            )
            .is_none()
        );
    }

    #[test]
    fn err_not_section() {
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        assert!(
            crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("blah blah"),
                &mut parser,
                &mut warnings
            )
            .is_none()
        );
    }

    mod asciidoc_style_headers {
        use std::ops::Deref;

        use pretty_assertions_sorted::assert_eq;

        use crate::{
            Parser,
            blocks::{ContentModel, IsBlock, MediaType, metadata::BlockMetadata},
            content::SubstitutionGroup,
            tests::prelude::*,
            warnings::WarningType,
        };

        #[test]
        fn err_missing_space_before_title() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            assert!(
                crate::blocks::SectionBlock::parse(
                    &BlockMetadata::new("=blah blah"),
                    &mut parser,
                    &mut warnings
                )
                .is_none()
            );
        }

        #[test]
        fn simplest_section_block() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("== Section Title"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[],
                    source: Span {
                        data: "== Section Title",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 1,
                    col: 17,
                    offset: 16
                }
            );
        }

        #[test]
        fn has_child_block() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("== Section Title\n\nabc"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 18,
                            },
                            rendered: "abc",
                        },
                        source: Span {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "== Section Title\n\nabc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 3,
                    col: 4,
                    offset: 21
                }
            );
        }

        #[test]
        fn has_macro_block_with_extra_blank_line() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new(
                    "== Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]\n\n",
                ),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Media(MediaBlock {
                        type_: MediaType::Image,
                        target: Span {
                            data: "bar",
                            line: 3,
                            col: 8,
                            offset: 25,
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
                                }
                            ],
                            anchor: None,
                            source: Span {
                                data: "alt=Sunset,width=300,height=400",
                                line: 3,
                                col: 12,
                                offset: 29,
                            }
                        },
                        source: Span {
                            data: "image::bar[alt=Sunset,width=300,height=400]",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "== Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 5,
                    col: 1,
                    offset: 63
                }
            );
        }

        #[test]
        fn has_child_block_with_errors() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new(
                    "== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                ),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Media(MediaBlock {
                        type_: MediaType::Image,
                        target: Span {
                            data: "bar",
                            line: 3,
                            col: 8,
                            offset: 25,
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
                                }
                            ],
                            anchor: None,
                            source: Span {
                                data: "alt=Sunset,width=300,,height=400",
                                line: 3,
                                col: 12,
                                offset: 29,
                            }
                        },
                        source: Span {
                            data: "image::bar[alt=Sunset,width=300,,height=400]",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "== Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 3,
                    col: 45,
                    offset: 62
                }
            );

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "alt=Sunset,width=300,,height=400",
                        line: 3,
                        col: 12,
                        offset: 29,
                    },
                    warning: WarningType::EmptyAttributeValue,
                }]
            );
        }

        #[test]
        fn dont_stop_at_child_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("== Section Title\n\nabc\n\n=== Section 2\n\ndef"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[
                        Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "abc",
                                    line: 3,
                                    col: 1,
                                    offset: 18,
                                },
                                rendered: "abc",
                            },
                            source: Span {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 18,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                        }),
                        Block::Section(SectionBlock {
                            level: 2,
                            section_title: Content {
                                original: Span {
                                    data: "Section 2",
                                    line: 5,
                                    col: 5,
                                    offset: 27,
                                },
                                rendered: "Section 2",
                            },
                            blocks: &[Block::Simple(SimpleBlock {
                                content: Content {
                                    original: Span {
                                        data: "def",
                                        line: 7,
                                        col: 1,
                                        offset: 38,
                                    },
                                    rendered: "def",
                                },
                                source: Span {
                                    data: "def",
                                    line: 7,
                                    col: 1,
                                    offset: 38,
                                },
                                title_source: None,
                                title: None,
                                anchor: None,
                                anchor_reftext: None,
                                attrlist: None,
                            })],
                            source: Span {
                                data: "=== Section 2\n\ndef",
                                line: 5,
                                col: 1,
                                offset: 23,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                            section_id: Some("_section_2"),
                            section_number: None,
                        })
                    ],
                    source: Span {
                        data: "== Section Title\n\nabc\n\n=== Section 2\n\ndef",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 7,
                    col: 4,
                    offset: 41
                }
            );
        }

        #[test]
        fn stop_at_peer_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("== Section Title\n\nabc\n\n== Section 2\n\ndef"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 18,
                            },
                            rendered: "abc",
                        },
                        source: Span {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "== Section Title\n\nabc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "== Section 2\n\ndef",
                    line: 5,
                    col: 1,
                    offset: 23
                }
            );
        }

        #[test]
        fn stop_at_ancestor_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("=== Section Title\n\nabc\n\n== Section 2\n\ndef"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 2,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        rendered: "Section Title",
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
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "=== Section Title\n\nabc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "== Section 2\n\ndef",
                    line: 5,
                    col: 1,
                    offset: 24
                }
            );
        }

        #[test]
        fn section_title_with_markup() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("== Section with *bold* text"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(
                mi.item.section_title_source(),
                Span {
                    data: "Section with *bold* text",
                    line: 1,
                    col: 4,
                    offset: 3,
                }
            );

            assert_eq!(
                mi.item.section_title(),
                "Section with <strong>bold</strong> text"
            );

            assert_eq!(mi.item.id().unwrap(), "_section_with_bold_text");
        }

        #[test]
        fn section_title_with_special_chars() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("== Section with <brackets> & ampersands"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(
                mi.item.section_title_source(),
                Span {
                    data: "Section with <brackets> & ampersands",
                    line: 1,
                    col: 4,
                    offset: 3,
                }
            );

            assert_eq!(
                mi.item.section_title(),
                "Section with &lt;brackets&gt; &amp; ampersands"
            );

            assert_eq!(mi.item.id().unwrap(), "_section_with_ampersands");
        }

        #[test]
        fn err_level_0_section_heading() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let result = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("= Document Title"),
                &mut parser,
                &mut warnings,
            );

            assert!(result.is_none());

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "= Document Title",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    warning: WarningType::Level0SectionHeadingNotSupported,
                }]
            );
        }

        #[test]
        fn err_section_heading_level_exceeds_maximum() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let result = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("======= Level 6 Section"),
                &mut parser,
                &mut warnings,
            );

            assert!(result.is_none());

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "======= Level 6 Section",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    warning: WarningType::SectionHeadingLevelExceedsMaximum(6),
                }]
            );
        }

        #[test]
        fn valid_maximum_level_5_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("====== Level 5 Section"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert!(warnings.is_empty());

            assert_eq!(mi.item.level(), 5);
            assert_eq!(mi.item.section_title(), "Level 5 Section");
            assert_eq!(mi.item.id().unwrap(), "_level_5_section");
        }

        #[test]
        fn warn_section_level_skipped() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("== Level 1\n\n==== Level 3 (skipped level 2)"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.level(), 1);
            assert_eq!(mi.item.section_title(), "Level 1");
            assert_eq!(mi.item.nested_blocks().len(), 1);
            assert_eq!(mi.item.id().unwrap(), "_level_1");

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "==== Level 3 (skipped level 2)",
                        line: 3,
                        col: 1,
                        offset: 12,
                    },
                    warning: WarningType::SectionHeadingLevelSkipped(1, 3),
                }]
            );
        }
    }

    mod markdown_style_headings {
        use std::ops::Deref;

        use pretty_assertions_sorted::assert_eq;

        use crate::{
            Parser,
            blocks::{ContentModel, IsBlock, MediaType, metadata::BlockMetadata},
            content::SubstitutionGroup,
            tests::prelude::*,
            warnings::WarningType,
        };

        #[test]
        fn err_missing_space_before_title() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            assert!(
                crate::blocks::SectionBlock::parse(
                    &BlockMetadata::new("#blah blah"),
                    &mut parser,
                    &mut warnings
                )
                .is_none()
            );
        }

        #[test]
        fn simplest_section_block() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("## Section Title"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[],
                    source: Span {
                        data: "## Section Title",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 1,
                    col: 17,
                    offset: 16
                }
            );
        }

        #[test]
        fn has_child_block() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("## Section Title\n\nabc"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 18,
                            },
                            rendered: "abc",
                        },
                        source: Span {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "## Section Title\n\nabc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 3,
                    col: 4,
                    offset: 21
                }
            );
        }

        #[test]
        fn has_macro_block_with_extra_blank_line() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new(
                    "## Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]\n\n",
                ),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Media(MediaBlock {
                        type_: MediaType::Image,
                        target: Span {
                            data: "bar",
                            line: 3,
                            col: 8,
                            offset: 25,
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
                                }
                            ],
                            anchor: None,
                            source: Span {
                                data: "alt=Sunset,width=300,height=400",
                                line: 3,
                                col: 12,
                                offset: 29,
                            }
                        },
                        source: Span {
                            data: "image::bar[alt=Sunset,width=300,height=400]",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "## Section Title\n\nimage::bar[alt=Sunset,width=300,height=400]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 5,
                    col: 1,
                    offset: 63
                }
            );
        }

        #[test]
        fn has_child_block_with_errors() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new(
                    "## Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                ),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Media(MediaBlock {
                        type_: MediaType::Image,
                        target: Span {
                            data: "bar",
                            line: 3,
                            col: 8,
                            offset: 25,
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
                                }
                            ],
                            anchor: None,
                            source: Span {
                                data: "alt=Sunset,width=300,,height=400",
                                line: 3,
                                col: 12,
                                offset: 29,
                            }
                        },
                        source: Span {
                            data: "image::bar[alt=Sunset,width=300,,height=400]",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "## Section Title\n\nimage::bar[alt=Sunset,width=300,,height=400]",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 3,
                    col: 45,
                    offset: 62
                }
            );

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "alt=Sunset,width=300,,height=400",
                        line: 3,
                        col: 12,
                        offset: 29,
                    },
                    warning: WarningType::EmptyAttributeValue,
                }]
            );
        }

        #[test]
        fn dont_stop_at_child_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("## Section Title\n\nabc\n\n### Section 2\n\ndef"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[
                        Block::Simple(SimpleBlock {
                            content: Content {
                                original: Span {
                                    data: "abc",
                                    line: 3,
                                    col: 1,
                                    offset: 18,
                                },
                                rendered: "abc",
                            },
                            source: Span {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 18,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                        }),
                        Block::Section(SectionBlock {
                            level: 2,
                            section_title: Content {
                                original: Span {
                                    data: "Section 2",
                                    line: 5,
                                    col: 5,
                                    offset: 27,
                                },
                                rendered: "Section 2",
                            },
                            blocks: &[Block::Simple(SimpleBlock {
                                content: Content {
                                    original: Span {
                                        data: "def",
                                        line: 7,
                                        col: 1,
                                        offset: 38,
                                    },
                                    rendered: "def",
                                },
                                source: Span {
                                    data: "def",
                                    line: 7,
                                    col: 1,
                                    offset: 38,
                                },
                                title_source: None,
                                title: None,
                                anchor: None,
                                anchor_reftext: None,
                                attrlist: None,
                            })],
                            source: Span {
                                data: "### Section 2\n\ndef",
                                line: 5,
                                col: 1,
                                offset: 23,
                            },
                            title_source: None,
                            title: None,
                            anchor: None,
                            anchor_reftext: None,
                            attrlist: None,
                            section_id: Some("_section_2"),
                            section_number: None,
                        })
                    ],
                    source: Span {
                        data: "## Section Title\n\nabc\n\n### Section 2\n\ndef",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "",
                    line: 7,
                    col: 4,
                    offset: 41
                }
            );
        }

        #[test]
        fn stop_at_peer_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("## Section Title\n\nabc\n\n## Section 2\n\ndef"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 1,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 4,
                            offset: 3,
                        },
                        rendered: "Section Title",
                    },
                    blocks: &[Block::Simple(SimpleBlock {
                        content: Content {
                            original: Span {
                                data: "abc",
                                line: 3,
                                col: 1,
                                offset: 18,
                            },
                            rendered: "abc",
                        },
                        source: Span {
                            data: "abc",
                            line: 3,
                            col: 1,
                            offset: 18,
                        },
                        title_source: None,
                        title: None,
                        anchor: None,
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "## Section Title\n\nabc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "## Section 2\n\ndef",
                    line: 5,
                    col: 1,
                    offset: 23
                }
            );
        }

        #[test]
        fn stop_at_ancestor_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("### Section Title\n\nabc\n\n## Section 2\n\ndef"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.content_model(), ContentModel::Compound);
            assert_eq!(mi.item.raw_context().deref(), "section");
            assert_eq!(mi.item.resolved_context().deref(), "section");
            assert!(mi.item.declared_style().is_none());
            assert_eq!(mi.item.id().unwrap(), "_section_title");
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
            assert!(mi.item.anchor_reftext().is_none());
            assert!(mi.item.attrlist().is_none());
            assert_eq!(mi.item.substitution_group(), SubstitutionGroup::Normal);

            assert_eq!(
                mi.item,
                SectionBlock {
                    level: 2,
                    section_title: Content {
                        original: Span {
                            data: "Section Title",
                            line: 1,
                            col: 5,
                            offset: 4,
                        },
                        rendered: "Section Title",
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
                        anchor_reftext: None,
                        attrlist: None,
                    })],
                    source: Span {
                        data: "### Section Title\n\nabc",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    title_source: None,
                    title: None,
                    anchor: None,
                    anchor_reftext: None,
                    attrlist: None,
                    section_id: Some("_section_title"),
                    section_number: None,
                }
            );

            assert_eq!(
                mi.after,
                Span {
                    data: "## Section 2\n\ndef",
                    line: 5,
                    col: 1,
                    offset: 24
                }
            );
        }

        #[test]
        fn section_title_with_markup() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("## Section with *bold* text"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(
                mi.item.section_title_source(),
                Span {
                    data: "Section with *bold* text",
                    line: 1,
                    col: 4,
                    offset: 3,
                }
            );

            assert_eq!(
                mi.item.section_title(),
                "Section with <strong>bold</strong> text"
            );

            assert_eq!(mi.item.id().unwrap(), "_section_with_bold_text");
        }

        #[test]
        fn section_title_with_special_chars() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("## Section with <brackets> & ampersands"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(
                mi.item.section_title_source(),
                Span {
                    data: "Section with <brackets> & ampersands",
                    line: 1,
                    col: 4,
                    offset: 3,
                }
            );

            assert_eq!(
                mi.item.section_title(),
                "Section with &lt;brackets&gt; &amp; ampersands"
            );
        }

        #[test]
        fn err_level_0_section_heading() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let result = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("# Document Title"),
                &mut parser,
                &mut warnings,
            );

            assert!(result.is_none());

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "# Document Title",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    warning: WarningType::Level0SectionHeadingNotSupported,
                }]
            );
        }

        #[test]
        fn err_section_heading_level_exceeds_maximum() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let result = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("####### Level 6 Section"),
                &mut parser,
                &mut warnings,
            );

            assert!(result.is_none());

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "####### Level 6 Section",
                        line: 1,
                        col: 1,
                        offset: 0,
                    },
                    warning: WarningType::SectionHeadingLevelExceedsMaximum(6),
                }]
            );
        }

        #[test]
        fn valid_maximum_level_5_section() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("###### Level 5 Section"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert!(warnings.is_empty());

            assert_eq!(mi.item.level(), 5);
            assert_eq!(mi.item.section_title(), "Level 5 Section");
            assert_eq!(mi.item.id().unwrap(), "_level_5_section");
        }

        #[test]
        fn warn_section_level_skipped() {
            let mut parser = Parser::default();
            let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

            let mi = crate::blocks::SectionBlock::parse(
                &BlockMetadata::new("## Level 1\n\n#### Level 3 (skipped level 2)"),
                &mut parser,
                &mut warnings,
            )
            .unwrap();

            assert_eq!(mi.item.level(), 1);
            assert_eq!(mi.item.section_title(), "Level 1");
            assert_eq!(mi.item.nested_blocks().len(), 1);
            assert_eq!(mi.item.id().unwrap(), "_level_1");

            assert_eq!(
                warnings,
                vec![Warning {
                    source: Span {
                        data: "#### Level 3 (skipped level 2)",
                        line: 3,
                        col: 1,
                        offset: 12,
                    },
                    warning: WarningType::SectionHeadingLevelSkipped(1, 3),
                }]
            );
        }
    }

    #[test]
    fn warn_multiple_section_levels_skipped() {
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        let mi = crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("== Level 1\n\n===== Level 4 (skipped levels 2 and 3)"),
            &mut parser,
            &mut warnings,
        )
        .unwrap();

        assert_eq!(mi.item.level(), 1);
        assert_eq!(mi.item.section_title(), "Level 1");
        assert_eq!(mi.item.nested_blocks().len(), 1);
        assert_eq!(mi.item.id().unwrap(), "_level_1");

        assert_eq!(
            warnings,
            vec![Warning {
                source: Span {
                    data: "===== Level 4 (skipped levels 2 and 3)",
                    line: 3,
                    col: 1,
                    offset: 12,
                },
                warning: WarningType::SectionHeadingLevelSkipped(1, 4),
            }]
        );
    }

    #[test]
    fn no_warning_for_consecutive_section_levels() {
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        let mi = crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("== Level 1\n\n=== Level 2 (no skip)"),
            &mut parser,
            &mut warnings,
        )
        .unwrap();

        assert_eq!(mi.item.level(), 1);
        assert_eq!(mi.item.section_title(), "Level 1");
        assert_eq!(mi.item.nested_blocks().len(), 1);
        assert_eq!(mi.item.id().unwrap(), "_level_1");

        assert!(warnings.is_empty());
    }

    #[test]
    fn section_id_generation_basic() {
        let input = "== Section One";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_section_one"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_with_special_characters() {
        let input = "== We're back! & Company";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_were_back_company"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_with_entities() {
        let input = "== Ben &amp; Jerry &#34;Ice Cream&#34;";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_ben_jerry_ice_cream"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_disabled_when_sectids_unset() {
        let input = ":!sectids:\n\n== Section One";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), None);
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_with_custom_prefix() {
        let input = ":idprefix: id_\n\n== Section One";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("id_section_one"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_with_custom_separator() {
        let input = ":idseparator: -\n\n== Section One";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_section-one"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_with_empty_prefix() {
        let input = ":idprefix:\n\n== Section One";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("section_one"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_removes_trailing_separator() {
        let input = ":idseparator: -\n\n== Section Title-";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_section-title"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_removes_leading_separator_when_prefix_empty() {
        let input = ":idprefix:\n:idseparator: -\n\n== -Section Title";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("section-title"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn section_id_generation_handles_multiple_trailing_separators() {
        let input = ":idseparator: _\n\n== Title with Multiple Dots...";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_title_with_multiple_dots"));
        } else {
            panic!("Expected section block");
        }
    }

    #[test]
    fn warn_duplicate_manual_section_id() {
        let input = "[#my_id]\n== First Section\n\n[#my_id]\n== Second Section";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        let mut warnings = document.warnings();

        assert_eq!(
            warnings.next().unwrap(),
            Warning {
                source: Span {
                    data: "[#my_id]\n== Second Section",
                    line: 4,
                    col: 1,
                    offset: 27,
                },
                warning: WarningType::DuplicateId("my_id".to_owned()),
            }
        );

        assert!(warnings.next().is_none());
    }

    #[test]
    fn section_with_custom_reftext_attribute() {
        let input = "[reftext=\"Custom Reference Text\"]\n== Section Title";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_section_title"));
        } else {
            panic!("Expected section block");
        }

        let catalog = document.catalog();
        let entry = catalog.get_ref("_section_title");
        assert!(entry.is_some());
        assert_eq!(
            entry.unwrap().reftext,
            Some("Custom Reference Text".to_string())
        );
    }

    #[test]
    fn section_without_reftext_uses_title() {
        let input = "== Section Title";
        let mut parser = Parser::default();
        let document = parser.parse(input);

        if let Some(crate::blocks::Block::Section(section)) = document.nested_blocks().next() {
            assert_eq!(section.id(), Some("_section_title"));
        } else {
            panic!("Expected section block");
        }

        let catalog = document.catalog();
        let entry = catalog.get_ref("_section_title");
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().reftext, Some("Section Title".to_string()));
    }

    #[test]
    fn impl_debug() {
        let mut parser = Parser::default();
        let mut warnings: Vec<crate::warnings::Warning<'_>> = vec![];

        let section = crate::blocks::SectionBlock::parse(
            &BlockMetadata::new("== Section Title"),
            &mut parser,
            &mut warnings,
        )
        .unwrap()
        .item;

        assert_eq!(
            format!("{section:#?}"),
            r#"SectionBlock {
    level: 1,
    section_title: Content {
        original: Span {
            data: "Section Title",
            line: 1,
            col: 4,
            offset: 3,
        },
        rendered: "Section Title",
    },
    blocks: &[],
    source: Span {
        data: "== Section Title",
        line: 1,
        col: 1,
        offset: 0,
    },
    title_source: None,
    title: None,
    anchor: None,
    anchor_reftext: None,
    attrlist: None,
    section_id: Some(
        "_section_title",
    ),
    section_number: None,
}"#
        );
    }

    mod section_number {
        mod assign_next_number {
            use crate::blocks::section::SectionNumber;

            #[test]
            fn default() {
                let sn = SectionNumber::default();
                assert_eq!(sn.components(), []);
                assert_eq!(sn.to_string(), "");
            }

            #[test]
            fn level_1() {
                let mut sn = SectionNumber::default();
                sn.assign_next_number(1);
                assert_eq!(sn.components(), [1]);
                assert_eq!(sn.to_string(), "1");
            }

            #[test]
            fn level_3() {
                let mut sn = SectionNumber::default();
                sn.assign_next_number(3);
                assert_eq!(sn.components(), [1, 1, 1]);
                assert_eq!(sn.to_string(), "1.1.1");
            }

            #[test]
            fn level_3_then_1() {
                let mut sn = SectionNumber::default();
                sn.assign_next_number(3);
                sn.assign_next_number(1);
                assert_eq!(sn.components(), [2]);
                assert_eq!(sn.to_string(), "2");
            }

            #[test]
            fn level_3_then_1_then_2() {
                let mut sn = SectionNumber::default();
                sn.assign_next_number(3);
                sn.assign_next_number(1);
                sn.assign_next_number(2);
                assert_eq!(sn.components(), [2, 1]);
                assert_eq!(sn.to_string(), "2.1");
            }
        }
    }
}
