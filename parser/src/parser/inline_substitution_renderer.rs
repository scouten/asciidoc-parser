#![allow(missing_docs)] // TEMPORARY while building

use std::fmt::Debug;

/// An implementation of `InlineSubstitutionRenderer` is used when converting
/// the basic raw text of a simple block to the format which will ultimately be
/// presented in the final converted output.
///
/// An implementation is provided for HTML output; alternative implementations
/// (not provided in this crate) could support other output formats.
pub trait InlineSubstitutionRenderer: Debug {
    /// Renders the substitution for a special character.
    ///
    /// The renderer should write the appropriate rendering to `dest`.
    fn render_special_character(&self, type_: SpecialCharacter, dest: &mut String);

    /// Renders the content of a [NEED LINK] quote substitution.
    ///
    /// The renderer should write the appropriate rendering to `dest`.
    fn render_quoted_substitition(
        &self,
        type_: QuoteType,
        scope: QuoteScope,
        id: Option<String>,
        body: &str,
        dest: &mut String,
    );
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecialCharacter {
    Lt,
    Gt,
    Ampersand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuoteType {
    Strong,
    DoubleQuote,
    SingleQuote,
    Monospaced,
    Emphasis,
    Mark,
    Superscript,
    Subscript,
    Unquoted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuoteScope {
    Constrained,
    Unconstrained,
}

/// Implementation of `InlineSubstitutionHandler` that renders substitutions for
/// common HTML-based applications.
#[derive(Debug)]
pub struct HtmlSubstitutionRenderer {}

impl InlineSubstitutionRenderer for HtmlSubstitutionRenderer {
    fn render_special_character(&self, type_: SpecialCharacter, dest: &mut String) {
        match type_ {
            SpecialCharacter::Lt => {
                dest.push_str("&lt;");
            }
            SpecialCharacter::Gt => {
                dest.push_str("&gt;");
            }
            SpecialCharacter::Ampersand => {
                dest.push_str("&amp;");
            }
        }
    }

    fn render_quoted_substitition(
        &self,
        type_: QuoteType,
        _scope: QuoteScope,
        _id: Option<String>,
        body: &str,
        dest: &mut String,
    ) {
        match type_ {
            QuoteType::Strong => {
                // TO DO: How will we use scope here?
                dest.push_str("<em>");
                dest.push_str(body);
                dest.push_str("</em>");
            }

            QuoteType::DoubleQuote => {
                dest.push_str("&#8220;");
                dest.push_str(body);
                dest.push_str("&#8221;");
            }

            QuoteType::SingleQuote => {
                dest.push_str("&#8216;");
                dest.push_str(body);
                dest.push_str("&#8217;");
            }

            QuoteType::Monospaced => {
                dest.push_str("<code>");
                dest.push_str(body);
                dest.push_str("</code>");
            }

            QuoteType::Superscript => {
                dest.push_str("<sup>");
                dest.push_str(body);
                dest.push_str("</sup>");
            }

            _ => {
                todo!("Render substitution for {type_:?}");
            }
        }
    }
}
