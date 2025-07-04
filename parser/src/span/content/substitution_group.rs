use std::ops::Sub;

use crate::{
    attributes::Attrlist,
    span::content::{Content, Passthroughs, SubstitutionStep},
    Parser,
};

/// Each block and inline element has a default substitution group that is
/// applied unless you customize the substitutions for a particular element.
///
/// `SubstitutionGroup` specifies the default or overridden substitution group
/// to be applied.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubstitutionGroup {
    /// The normal substitution group is applied to the majority of the AsciiDoc
    /// block and inline elements except for specific elements described in the
    /// next sections.
    Normal,

    /// The header substitution group is applied to metadata lines (author and
    /// revision information) in the document header. It’s also applied to the
    /// values of attribute entries, regardless of whether those entries are
    /// defined in the document header or body. Only special characters,
    /// attribute references, and the inline pass macro are replaced in elements
    /// that fall under the header group.
    ///
    /// You can use the inline pass macro in attribute entries to customize the
    /// substitution types applied to the attribute’s value.
    Header,

    /// Literal, listing, and source blocks are processed using the verbatim
    /// substitution group. Only special characters are replaced in these
    /// blocks.
    Verbatim,

    /// No substitutions are applied to three of the elements in the pass
    /// substitution group. These elements include the passthrough block, inline
    /// pass macro, and triple plus macro.
    ///
    /// The inline single plus and double plus macros also belong to the pass
    /// group. Only the special characters substitution is applied to these
    /// elements.
    Pass,

    /// The none substitution group is applied to comment blocks. No
    /// substitutions are applied to comments.
    None,

    /// The attribute entry value substitution group is applied to attribute
    /// values. Only special characters and attribute references are applied to
    /// these values.
    AttributeEntryValue,

    /// You can customize the substitutions applied to the content of an inline
    /// pass macro by specifying one or more substitution values. Multiple
    /// values must be separated by commas and may not contain any spaces. The
    /// substitution value is either the formal name of a substitution type or
    /// group, or its shorthand.
    ///
    /// See [Custom substitutions].
    ///
    /// [Custom substitutions]: https://docs.asciidoctor.org/asciidoc/latest/pass/pass-macro/#custom-substitutions
    Custom(Vec<SubstitutionStep>),
}

impl SubstitutionGroup {
    /// Parse the custom substitution group syntax defined in [Custom
    /// substitutions].
    ///
    /// [Custom substitutions]: https://docs.asciidoctor.org/asciidoc/latest/pass/pass-macro/#custom-substitutions
    pub(crate) fn from_custom_string(mut custom: &str) -> Option<Self> {
        custom = custom.trim();

        if custom == "n" || custom == "normal" {
            return Some(Self::Normal);
        }

        if custom == "v" || custom == "verbatim" {
            return Some(Self::Verbatim);
        }

        let mut steps: Vec<SubstitutionStep> = vec![];

        for mut step in custom.split(",") {
            step = step.trim();

            if step == "n" || step == "normal" {
                steps = vec![
                    SubstitutionStep::SpecialCharacters,
                    SubstitutionStep::Quotes,
                    SubstitutionStep::AttributeReferences,
                    SubstitutionStep::CharacterReplacements,
                    SubstitutionStep::Macros,
                    SubstitutionStep::PostReplacement,
                ];
                continue;
            }

            if step == "v" || step == "verbatim" {
                steps = vec![SubstitutionStep::SpecialCharacters];
                continue;
            }

            let subtract = if step.starts_with('-') {
                step = &step[1..];
                true
            } else {
                false
            };

            let step = match step {
                "c" | "specialcharacters" | "specialchars" => SubstitutionStep::SpecialCharacters,
                "q" | "quotes" => SubstitutionStep::Quotes,
                "a" | "attributes" => SubstitutionStep::AttributeReferences,
                "r" | "replacements" => SubstitutionStep::CharacterReplacements,
                "m" | "macros" => SubstitutionStep::Macros,
                "p" | "post replacements" => SubstitutionStep::PostReplacement,
                _ => {
                    return None;
                }
            };

            if subtract {
                steps.retain(|s| s != &step);
            } else {
                steps.push(step);
            }
        }

        Some(Self::Custom(steps))
    }

    pub(crate) fn apply(
        &self,
        content: &mut Content<'_>,
        parser: &Parser,
        attrlist: Option<&Attrlist>,
    ) {
        let mut passthroughs: Option<Passthroughs> = None;

        match self {
            Self::Normal => {
                passthroughs = Some(Passthroughs::extract_from(content));

                SubstitutionStep::SpecialCharacters.apply(content, parser, attrlist);
                SubstitutionStep::Quotes.apply(content, parser, attrlist);
                SubstitutionStep::AttributeReferences.apply(content, parser, attrlist);
                SubstitutionStep::CharacterReplacements.apply(content, parser, attrlist);
                // TO DO: Add these as they are implemented.
                // SubstitutionStep::Macros.apply(content, parser);
                SubstitutionStep::PostReplacement.apply(content, parser, attrlist);
            }

            Self::Verbatim => {
                SubstitutionStep::SpecialCharacters.apply(content, parser, attrlist);
            }

            Self::Pass | Self::None => {}

            Self::AttributeEntryValue => {
                SubstitutionStep::SpecialCharacters.apply(content, parser, attrlist);
                SubstitutionStep::AttributeReferences.apply(content, parser, attrlist);
            }

            Self::Custom(ref steps) => {
                for step in steps {
                    step.apply(content, parser, attrlist);
                }
            }

            _ => {
                // Do passthroughs if sub steps includes macros.
                todo!("Implement apply for SubstitutionGroup::{self:?}");
            }
        }

        if let Some(passthroughs) = passthroughs {
            passthroughs.restore_to(content, parser);
        }
    }
}
