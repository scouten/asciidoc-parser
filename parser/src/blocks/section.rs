use std::{slice::Iter, sync::LazyLock};

use regex::Regex;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        Block, ContentModel, IsBlock, metadata::BlockMetadata, parse_utils::parse_blocks_until,
    },
    content::{Content, SubstitutionGroup},
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
    section_id: Option<String>,
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

        // TODO (https://github.com/scouten/asciidoc-parser/issues/411):
        // Track section ID whether automatically generated or manually specified and
        // warn on conflicts.

        let section_id = if metadata.anchor.is_none()
            && metadata
                .attrlist
                .as_ref()
                .map(|a| a.id().is_none())
                .unwrap_or(true)
            && parser.is_attribute_set("sectids")
        {
            Some(generate_section_id(section_title.rendered(), parser))
        } else {
            None
        };

        warnings.append(&mut maw_blocks.warnings);

        Some(MatchedItem {
            item: Self {
                level: level_and_title.item.0,
                section_title,
                blocks: blocks.item,
                source: source.trim_trailing_whitespace(),
                title_source: metadata.title_source,
                title: metadata.title.clone(),
                anchor: metadata.anchor,
                attrlist: metadata.attrlist.clone(),
                section_id,
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
    /// in the `IsBlock` to retrieve the effective ID for this block, which
    /// considers both auto-generated IDs and manually-set IDs.
    #[cfg(test)]
    pub(crate) fn section_id(&'src self) -> Option<&'src str> {
        self.section_id.as_deref()
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

/// Generate a section ID from the section title.
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

    let mut gen_id = format!("{}{}", idprefix, title.to_lowercase());

    #[allow(clippy::unwrap_used)]
    static INVALID_SECTION_ID_CHARS: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"<[^>]+>|&(?:[a-z][a-z]+\d{0,2}|#\d{2,5}|#x[\da-f]{2,4});|[^ \w\-.]+").unwrap()
    });

    gen_id = INVALID_SECTION_ID_CHARS
        .replace_all(&gen_id, "")
        .to_string();

    if idseparator.is_empty() {
        gen_id = gen_id.replace(' ', "");
    } else {
        // Take only first character of separator if multiple provided.
        let sep = idseparator.chars().next().unwrap_or('_');
        let sep_str = sep.to_string();

        // Replace spaces, hyphens, and periods with separator.
        if sep == '-' || sep == '.' {
            // For hyphen/period separators, replace spaces and dots/hyphens accordingly.
            gen_id = gen_id.replace([' ', '.', '-'], &sep_str);
        } else {
            // For other separators, replace space, period, and hyphen.
            gen_id = gen_id.replace([' ', '.', '-'], &sep_str);
        }

        // Remove repeating separator characters.
        while gen_id.contains(&format!("{}{}", sep_str, sep_str)) {
            gen_id = gen_id.replace(&format!("{}{}", sep_str, sep_str), &sep_str);
        }

        // Remove trailing separator.
        if gen_id.ends_with(&sep_str) {
            gen_id.pop();
        }

        // If `idprefix` is empty and generated ID starts with separator, remove leading
        // separator.
        if idprefix.is_empty() && gen_id.starts_with(&sep_str) {
            gen_id = gen_id[sep_str.len()..].to_string();
        }
    }

    gen_id
}
