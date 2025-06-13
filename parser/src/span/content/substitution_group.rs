use crate::{
    span::content::{Content, SubstitutionStep},
    Parser,
};

/// Each block and inline element has a default substitution group that is
/// applied unless you customize the substitutions for a particular element.
///
/// `SubstitutionGroup` specifies the default or overridden substitution group
/// to be applied.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
}

impl SubstitutionGroup {
    pub(crate) fn apply(&self, content: &mut Content<'_>, parser: &Parser) {
        match self {
            Self::Normal => {
                SubstitutionStep::SpecialCharacters.apply(content, parser);
                SubstitutionStep::Quotes.apply(content, parser);
                SubstitutionStep::AttributeReferences.apply(content, parser);
                SubstitutionStep::CharacterReplacements.apply(content, parser);
                // TO DO: Add these as they are implemented.
                // SubstitutionStep::Macros.apply(content, parser);
                SubstitutionStep::PostReplacement.apply(content, parser);
            }

            Self::Verbatim => {
                SubstitutionStep::SpecialCharacters.apply(content, parser);
            }

            Self::Pass | Self::None => {}

            _ => {
                todo!("Implement apply for SubstitutionGroup::{self:?}");
            }
        }
    }
}
