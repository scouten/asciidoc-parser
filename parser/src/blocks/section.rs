use std::slice::Iter;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        Block, ContentModel, IsBlock, metadata::BlockMetadata, parse_utils::parse_blocks_until,
    },
    content::{Content, SubstitutionGroup},
    document::RefType,
    span::MatchedItem,
    strings::CowStr,
    warnings::{Warning, WarningType},
};

/// Sections partition the document into a content hierarchy. A section is an
/// implicit enclosure. Each section begins with a title and ends at the next
/// sibling section, ancestor section, or end of document. Nested section levels
/// must be sequential.
///
/// **WARNING:** This is a very preliminary implementation. There are many **TO
/// DO** items in this code.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SectionBlock<'src> {
    level: usize,
    section_title: Content<'src>,
    blocks: Vec<Block<'src>>,
    source: Span<'src>,
    title_source: Option<Span<'src>>,
    title: Option<String>,
    anchor: Option<Span<'src>>,
    attrlist: Option<Attrlist<'src>>,
}

impl<'src> SectionBlock<'src> {
    pub(crate) fn parse(
        metadata: &BlockMetadata<'src>,
        parser: &mut Parser,
        warnings: &mut Vec<Warning<'src>>,
    ) -> Option<MatchedItem<'src, Self>> {
        let source = metadata.block_start.discard_empty_lines();
        let level_and_title = parse_title_line(source, warnings)?;

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

        warnings.append(&mut maw_blocks.warnings);

        let section = Self {
            level: level_and_title.item.0,
            section_title,
            blocks: blocks.item,
            source: source.trim_trailing_whitespace(),
            title_source: metadata.title_source,
            title: metadata.title.clone(),
            anchor: metadata.anchor,
            attrlist: metadata.attrlist.clone(),
        };

        if parser.is_attribute_set("sectids")
            && let Some(id) = section.id()
            && let Some(catalog) = parser.catalog_mut()
            && let Err(_duplicate_error) =
                catalog.register_ref(id, section.title(), RefType::Section)
        {
            warnings.push(Warning {
                source: section.source,
                warning: WarningType::DuplicateId(id.to_string()),
            });
        }

        Some(MatchedItem {
            item: section,
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

    fn attrlist(&'src self) -> Option<&'src Attrlist<'src>> {
        self.attrlist.as_ref()
    }
}

impl<'src> HasSpan<'src> for SectionBlock<'src> {
    fn span(&self) -> Span<'src> {
        self.source
    }
}

fn parse_title_line<'src>(
    source: Span<'src>,
    warnings: &mut Vec<Warning<'src>>,
) -> Option<MatchedItem<'src, (usize, Span<'src>)>> {
    let mi = source.take_non_empty_line()?;
    let mut line = mi.item;

    // TO DO: Disallow empty title.

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
    if let Some(mi) = parse_title_line(source, warnings) {
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

#[cfg(test)]
mod tests {
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                            attrlist: None,
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                            attrlist: None,
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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
            assert!(mi.item.id().is_none());
            assert!(mi.item.roles().is_empty());
            assert!(mi.item.options().is_empty());
            assert!(mi.item.title_source().is_none());
            assert!(mi.item.title().is_none());
            assert!(mi.item.anchor().is_none());
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
                    attrlist: None,
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

        assert!(warnings.is_empty());
    }
}
