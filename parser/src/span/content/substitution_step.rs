use std::{borrow::Cow, sync::LazyLock};

use regex::{Captures, Regex, Replacer};

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

            Self::Quotes => {
                apply_quotes(content);
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

static QUOTED_TEXT_SNIFF: LazyLock<Regex> = LazyLock::new(|| Regex::new("[*_`#^~]").unwrap());

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QuoteSubType {
    Strong,
    DoubleQuote,
    SingleQuote,
    Monospaced,
    Emphasis,
    Mark,
    Superscript,
    Subscript,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum QuoteSubScope {
    Constrained,
    Unconstrained,
}

struct QuoteSub {
    type_: QuoteSubType,
    scope: QuoteSubScope,
    pattern: Regex,
}

const QUOTE_ATTR_LIST_RXT: &str = "\\[([^\\[\\]]+)\\]";

// Adapted from QUOTE_SUBS in Ruby Asciidoctor implementation,
// found in https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor.rb#L440.
//
// Translation notes:
// * The `\m` modifier on Ruby regex means the `.` pattern *can* match a new
//   line. This appears to translate to `?m?` in Rust regex syntax.
// * The `(?!#{CG_WORD})` look-ahead syntax is not available in Rust regex. It
//   looks like the `\b{end-half}` pattern can take its place. (This pattern
//   requires that a non-word character or end of haystack follow the match
//   point.)
static QUOTE_SUBS: LazyLock<Vec<QuoteSub>> = LazyLock::new(|| {
    vec![QuoteSub {
        type_: QuoteSubType::Strong,
        scope: QuoteSubScope::Constrained,
        pattern: Regex::new("\\b{start-half}\\*(\\S|\\S.*?\\S)\\*\\b{end-half}").unwrap(),
        // NOTE: Removed (?:#{QuoteAttributeListRxt})? to bootstrap
        //       [:strong, :constrained,
        // /(^|[^#{CC_WORD};:}])(?:#{QuoteAttributeListRxt})?\*(\S|\S#{CC_ALL}*?\S)\*(?!#{CG_WORD})/
        // m],

        // pattern: Regex::new(&format!("\\?(?:{QUOTE_ATTR_LIST_RXT})?\\*\\*(.+?)\\*\\*")).unwrap(),
    }]
});

#[derive(Debug)]
struct QuoteReplacer {
    type_: QuoteSubType,
    scope: QuoteSubScope,
}

impl Replacer for QuoteReplacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String) {
        // Adaapted from Asciidoctor#convert_quoted_text, found in
        // https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/substitutors.rb#L1419-L1445.

        dbg!(&self);
        dbg!(caps);
        dbg!(&dst);

        // dst.push_str(&caps["first"]);
        // dst.push_str(" ");
        // dst.push_str(&caps["last"]);

        // if match[0].start_with? RS
        //     if scope == :constrained && (attrs = match[2])
        //         unescaped_attrs = %([#{attrs}])
        //     else
        //         return match[0].slice 1, match[0].length
        //     end
        // end

        match self.scope {
            QuoteSubScope::Constrained => {
                if false {
                    todo!(
                        "{}",
                        r##"
                        if unescaped_attrs
                            %(#{unescaped_attrs}#{Inline.new(self, :quoted, match[3], type: type).convert})
                        else
                            if (attrlist = match[2])
                            id = (attributes = parse_quoted_text_attributes attrlist)['id']
                            type = :unquoted if type == :mark
                            end
                            %(#{match[1]}#{Inline.new(self, :quoted, match[3], type: type, id: id, attributes: attributes).convert})
                        end
                    "##
                    );
                }

                // TEMPORARY: POC with simplest possible implementation for now.
                dst.push_str("<em>");
                dst.push_str(&caps[1]);
                dst.push_str("</em>");
            }

            QuoteSubScope::Unconstrained => {
                todo!(
                    r#"
                    if (attrlist = match[1])
                        id = (attributes = parse_quoted_text_attributes attrlist)['id']
                        type = :unquoted if type == :mark
                    end
                        Inline.new(self, :quoted, match[2], type: type, id: id, attributes: attributes).convert
                    end
"#
                );
            }
        }
    }
}

fn apply_quotes(content: &mut Content<'_>) {
    if !QUOTED_TEXT_SNIFF.is_match(content.rendered.as_ref()) {
        eprintln!("QT sniff said no match");
        return;
    }

    let mut result: Cow<'_, str> = content.rendered.to_string().into();

    for sub in &*QUOTE_SUBS {
        let replacer = QuoteReplacer {
            type_: sub.type_,
            scope: sub.scope,
        };

        dbg!(&replacer);

        if let Cow::Owned(new_result) = sub.pattern.replace_all(&result, replacer) {
            result = new_result.into();
        }
        // If it's Cow::Borrowed, there was no match for this pattern, so no
        // need to pay for a new string allocation.
    }

    content.rendered = result.into();
}

// https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/substitutors.rb#L1419-L1445
