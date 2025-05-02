use crate::span::Content;

/// Each substitution type replaces characters, markup, attribute references,
/// and macros in text with the appropriate output for a given converter. When a
/// document is processed, up to six substitution types may be carried out
/// depending on the block or inline element’s assigned substitution group. The
/// processor runs the substitutions in the following order:
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum SubstitutionStep {
    /// Searches for three characters (`<`, `>`, `&`) and replaces them with
    /// their named character references.
    SpecialCharacters,

    /// Replacement of formatting markup on inline elements.
    Quotes,

    /// Replacement of attribute references by the values they reference.
    AttributeReferences,

    /// Replaces textual characters such as marks, arrows, and dashes and
    /// replaces them with the decimal format of their Unicode code point, i.e.,
    /// a numeric character reference.
    CharacterReplacements,

    /// Replaces a macro’s content with the appropriate built-in and
    /// user-defined configuration.
    Macros,

    /// Replaces the line break character, `+` with a line-end marker.
    PostReplacement,

    /// Processes callouts in literal, listing, and source blocks.
    Callouts,
}

impl SubstitutionStep {
    pub(crate) fn apply(&self, content: &mut Content<'_>) {
        match self {
            Self::SpecialCharacters => {
                apply_special_characters(content);
            }
            _ => {
                todo!("Implement apply for {self:?}");
            }
        }
    }
}

fn apply_special_characters(content: &mut Content<'_>) {
    if !content.rendered.contains(['<', '>', '&']) {
        return;
    }

    // TO DO: Can we optimize down to one .replace?
    let new_rendered = content
        .rendered
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;");

    content.rendered = new_rendered.into();
}
