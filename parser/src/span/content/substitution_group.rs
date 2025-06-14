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
    pub(crate) fn apply(
        &self,
        content: &mut Content<'_>,
        parser: &Parser,
        attrlist: Option<&Attrlist>,
    ) {
        let mut passthroughs: Option<Passthroughs<'_>> = None;

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

            _ => {
                // Do passthroughs if sub steps includes macros.
                todo!("Implement apply for SubstitutionGroup::{self:?}");
            }
        }

        if let Some(passthroughs) = passthroughs {
            passthroughs.restore_to(content);
        }
    }
}
