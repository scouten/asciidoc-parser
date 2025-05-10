#![allow(missing_docs)] // TEMPORARY while building

use std::fmt::Debug;

use crate::attributes::Attrlist;

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
        attrlist: Option<Attrlist<'_>>,
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
        attrlist: Option<Attrlist<'_>>,
        _id: Option<String>,
        body: &str,
        dest: &mut String,
    ) {
        // TO DO: Check this assumption: A block style contributes to the list of styles
        // that is otherwise built from `role` attribute(s).
        //
        // Inspired by https://github.com/asciidoctor/asciidoctor/blob/main/test/substitutions_test.rb#L201-L204.

        let mut roles: Vec<&str> = attrlist
            .as_ref()
            .map(|a| a.roles().iter().map(|r| r.data()).collect())
            .unwrap_or_default();

        if let Some(block_style) = attrlist
            .as_ref()
            .and_then(|a| a.nth_attribute(1))
            .and_then(|attr1| attr1.block_style())
        {
            roles.insert(0, block_style.data());
        }

        match type_ {
            QuoteType::Strong => {
                // TO DO: How will we use scope here?
                dest.push_str("<strong>");
                dest.push_str(body);
                dest.push_str("</strong>");
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

            QuoteType::Emphasis => {
                // TO DO: How will we use scope here?
                dest.push_str("<em>");
                dest.push_str(body);
                dest.push_str("</em>");
            }

            QuoteType::Mark => {
                if roles.is_empty() {
                    dest.push_str("<mark>");
                    dest.push_str(body);
                    dest.push_str("</mark>");
                } else {
                    let roles = roles.join(" ");
                    dest.push_str("<span class=\"");
                    dest.push_str(&roles);
                    dest.push_str("\">");
                    dest.push_str(body);
                    dest.push_str("</span>");
                }
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
