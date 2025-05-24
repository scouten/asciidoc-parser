use std::{borrow::Cow, sync::LazyLock};

use regex::{Captures, Regex, RegexBuilder, Replacer};

use crate::{
    attributes::Attrlist,
    document::InterpretedValue,
    parser::{InlineSubstitutionRenderer, QuoteScope, QuoteType, SpecialCharacter},
    span::Content,
    Parser,
};

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
    pub(crate) fn apply(&self, content: &mut Content<'_>, parser: &Parser) {
        match self {
            Self::SpecialCharacters => {
                apply_special_characters(content, parser.renderer);
            }
            Self::Quotes => {
                apply_quotes(content, parser.renderer);
            }
            Self::AttributeReferences => {
                apply_attributes(content, parser);
            }
            _ => {
                todo!("Implement apply for SubstitutionStep::{self:?}");
            }
        }
    }
}

fn apply_special_characters(content: &mut Content<'_>, renderer: &dyn InlineSubstitutionRenderer) {
    if !content.rendered.contains(['<', '>', '&']) {
        return;
    }

    let mut result: Cow<'_, str> = content.rendered.to_string().into();
    let replacer = SpecialCharacterReplacer { renderer };

    if let Cow::Owned(new_result) = SPECIAL_CHARS.replace_all(&result, replacer) {
        result = new_result.into();
    }

    content.rendered = result.into();
}

#[derive(Debug)]
struct SpecialCharacterReplacer<'r> {
    renderer: &'r dyn InlineSubstitutionRenderer,
}

impl Replacer for SpecialCharacterReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        if let Some(which) = match caps[0].as_ref() {
            "<" => Some(SpecialCharacter::Lt),
            ">" => Some(SpecialCharacter::Gt),
            "&" => Some(SpecialCharacter::Ampersand),
            _ => None,
        } {
            self.renderer.render_special_character(which, dest);
        } else {
            dest.push_str(caps[0].as_ref());
        }
    }
}

static SPECIAL_CHARS: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new("[<>&]").unwrap()
});

static QUOTED_TEXT_SNIFF: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new("[*_`#^~]").unwrap()
});

struct QuoteSub {
    type_: QuoteType,
    scope: QuoteScope,
    pattern: Regex,
}

// Adapted from QUOTE_SUBS in Ruby Asciidoctor implementation,
// found in https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor.rb#L440.
//
// Translation notes:
// * The `\m` modifier on Ruby regex means the `.` pattern *can* match a new
//   line. We use the `.dot_matches_new_line(true)` option on `RegexBuilder` to
//   implement this instead.
// * The `(?!#{CG_WORD})` look-ahead syntax is not available in Rust regex. It
//   looks like the `\b{end-half}` pattern can take its place. (This pattern
//   requires that a non-word character or end of haystack follow the match
//   point.)
// * `#{CC_ALL}` just means any character (`.`).
// * Replace `#{QuoteAttributeListRxt}` with `\\[([^\\[\\]]+)\\]`. (This seems
//   preferable to having yet another level of backslash escaping.)
//
// Notes from the original Ruby implementation:
// * Unconstrained quotes can appear anywhere.
// * Constrained quotes must be bordered by non-word characters.
// * NOTE: These substitutions are processed in the order they appear here and
//   the order in which they are replaced is important.
static QUOTE_SUBS: LazyLock<Vec<QuoteSub>> = LazyLock::new(|| {
    vec![
        QuoteSub {
            // **strong**
            type_: QuoteType::Strong,
            scope: QuoteScope::Unconstrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(r#"\\?(?:\[([^\[\]]+)\])?\*\*(.+?)\*\*"#)
                .dot_matches_new_line(true)
                .build()
                .unwrap(),
        },
        QuoteSub {
            // *strong*
            type_: QuoteType::Strong,
            scope: QuoteScope::Constrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(
                r#"(^|[^\w&;:}])(?:\[([^\[\]]+)\])?\*(\S|\S.*?\S)\*\b{end-half}"#,
            )
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        },
        QuoteSub {
            // "`double-quoted`"
            type_: QuoteType::DoubleQuote,
            scope: QuoteScope::Constrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(
                r#"(^|[^\w&;:}])(?:\[([^\[\]]+)\])?"`(\S|\S.*?\S)`"\b{end-half}"#,
            )
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        },
        QuoteSub {
            // '`single-quoted`'
            type_: QuoteType::SingleQuote,
            scope: QuoteScope::Constrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(
                r#"(^|[^\w&;:}])(?:\[([^\[\]]+)\])?'`(\S|\S.*?\S)`'\b{end-half}"#,
            )
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        },
        QuoteSub {
            // ``monospaced``
            type_: QuoteType::Monospaced,
            scope: QuoteScope::Unconstrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(r#"\\?(?:\[([^\[\]]+)\])?``(.+?)``"#)
                .dot_matches_new_line(true)
                .build()
                .unwrap(),
        },
        QuoteSub {
            // `monospaced`
            type_: QuoteType::Monospaced,
            scope: QuoteScope::Constrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(
                r#"(^|[^\w&;:"'`}])(?:\[([^\[\]]+)\])?`(\S|\S.*?\S)`\b{end-half}"#,
                // NB: We don't have look-ahead in Rust Regex, so we might miss some edge cases
                // because Ruby's version matches `(?![#{CC_WORD}"'`])` which is slightly more
                // detailed than our `\b{end-half}`.
            )
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        },
        QuoteSub {
            // __emphasis__
            type_: QuoteType::Emphasis,
            scope: QuoteScope::Unconstrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(r#"\\?(?:\[([^\[\]]+)\])?__(.+?)__"#)
                .dot_matches_new_line(true)
                .build()
                .unwrap(),
        },
        QuoteSub {
            // _emphasis_
            type_: QuoteType::Emphasis,
            scope: QuoteScope::Constrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(
                r#"(^|[^\w&;:}])(?:\[([^\[\]]+)\])?_(\S|\S.*?\S)_\b{end-half}"#,
            )
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        },
        QuoteSub {
            // ##mark##
            type_: QuoteType::Mark,
            scope: QuoteScope::Unconstrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(r#"\\?(?:\[([^\[\]]+)\])?##(.+?)##"#)
                .dot_matches_new_line(true)
                .build()
                .unwrap(),
        },
        QuoteSub {
            // #mark#
            type_: QuoteType::Mark,
            scope: QuoteScope::Constrained,
            #[allow(clippy::unwrap_used)]
            pattern: RegexBuilder::new(
                r#"(^|[^\w&;:}])(?:\[([^\[\]]+)\])?#(\S|\S.*?\S)#\b{end-half}"#,
            )
            .dot_matches_new_line(true)
            .build()
            .unwrap(),
        },
        QuoteSub {
            // ^superscript^
            type_: QuoteType::Superscript,
            scope: QuoteScope::Unconstrained,
            #[allow(clippy::unwrap_used)]
            pattern: Regex::new(r#"\\?(?:\[([^\[\]]+)\])?\^(\S+?)\^"#).unwrap(),
        },
        QuoteSub {
            // ~subscript~
            type_: QuoteType::Subscript,
            scope: QuoteScope::Unconstrained,
            #[allow(clippy::unwrap_used)]
            pattern: Regex::new(r#"\\?(?:\[([^\[\]]+)\])?~(\S+?)~"#).unwrap(),
        },
    ]
});

#[derive(Debug)]
struct QuoteReplacer<'r> {
    type_: QuoteType,
    scope: QuoteScope,
    renderer: &'r dyn InlineSubstitutionRenderer,
}

impl Replacer for QuoteReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        // Adapted from Asciidoctor#convert_quoted_text, found in
        // https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/substitutors.rb#L1419-L1445.

        let unescaped_attrs: Option<String> = if caps[0].starts_with('\\') {
            let maybe_attrs = caps.get(2).map(|a| a.as_str());
            if self.scope == QuoteScope::Constrained && maybe_attrs.is_some() {
                Some(format!(
                    "[{attrs}]",
                    attrs = maybe_attrs.unwrap_or_default()
                ))
            } else {
                dest.push_str(&caps[0][1..]);
                return;
            }
        } else {
            None
        };

        match self.scope {
            QuoteScope::Constrained => {
                if let Some(attrs) = unescaped_attrs {
                    dest.push_str(&attrs);
                    self.renderer.render_quoted_substitition(
                        self.type_, self.scope, None, None, &caps[3], dest,
                    );
                } else {
                    let (attrlist, type_): (Option<Attrlist<'_>>, QuoteType) =
                        if let Some(attrlist) = caps.get(2) {
                            let type_ = if self.type_ == QuoteType::Mark {
                                QuoteType::Unquoted
                            } else {
                                self.type_
                            };

                            (
                                Some(
                                    Attrlist::parse(crate::Span::new(attrlist.as_str()))
                                        .item
                                        .item,
                                ),
                                type_,
                            )
                        } else {
                            (None, self.type_)
                        };

                    if let Some(prefix) = caps.get(1) {
                        dest.push_str(prefix.as_str());
                    }

                    let id = attrlist
                        .as_ref()
                        .and_then(|a| a.id().map(|s| s.to_string()));

                    self.renderer.render_quoted_substitition(
                        type_, self.scope, attrlist, id, &caps[3], dest,
                    );
                }
            }

            QuoteScope::Unconstrained => {
                let (attrlist, type_): (Option<Attrlist<'_>>, QuoteType) =
                    if let Some(attrlist) = caps.get(1) {
                        let type_ = if self.type_ == QuoteType::Mark {
                            QuoteType::Unquoted
                        } else {
                            self.type_
                        };

                        (
                            Some(
                                Attrlist::parse(crate::Span::new(attrlist.as_str()))
                                    .item
                                    .item,
                            ),
                            type_,
                        )
                    } else {
                        (None, self.type_)
                    };

                let id = attrlist
                    .as_ref()
                    .and_then(|a| a.id().map(|s| s.to_string()));

                self.renderer
                    .render_quoted_substitition(type_, self.scope, attrlist, id, &caps[2], dest);
            }
        }
    }
}

fn apply_quotes(content: &mut Content<'_>, renderer: &dyn InlineSubstitutionRenderer) {
    if !QUOTED_TEXT_SNIFF.is_match(content.rendered.as_ref()) {
        return;
    }

    let mut result: Cow<'_, str> = content.rendered.to_string().into();

    for sub in &*QUOTE_SUBS {
        let replacer = QuoteReplacer {
            type_: sub.type_,
            scope: sub.scope,
            renderer,
        };

        if let Cow::Owned(new_result) = sub.pattern.replace_all(&result, replacer) {
            result = new_result.into();
        }
        // If it's Cow::Borrowed, there was no match for this pattern, so no
        // need to pay for a new string allocation.
    }

    content.rendered = result.into();
}

static ATTRIBUTE_REFERENCE: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new("\\?{([A-Za-z0-9_][A-Za-z0-9_-]*)}").unwrap()
});

#[derive(Debug)]
struct AttributeReplacer<'p>(&'p Parser<'p>);

impl<'p> Replacer for AttributeReplacer<'p> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        let attr_name = &caps[1];

        // TO DO: Handle alternative responses ('skip', etc.) for missing attributes.
        if !self.0.has_attribute(attr_name) {
            dest.push_str(&caps[0]);
            return;
        }

        if caps[0].starts_with('\\') {
            dest.push_str(&caps[0]);
            return;
        }

        match self.0.attribute_value(attr_name) {
            InterpretedValue::Value(value) => {
                dest.push_str(value.as_ref());
            }
            x => {
                unimplemented!("What is the replacement value for InterpretedValue::{x:?}?");
            }
        }
    }
}

fn apply_attributes(content: &mut Content<'_>, parser: &Parser) {
    if !content.rendered.contains('{') {
        return;
    }

    let mut result: Cow<'_, str> = content.rendered.to_string().into();

    for sub in &*QUOTE_SUBS {
        let replacer = AttributeReplacer(parser);

        if let Cow::Owned(new_result) = sub.pattern.replace_all(&result, replacer) {
            result = new_result.into();
        }
        // If it's Cow::Borrowed, there was no match for this pattern, so no
        // need to pay for a new string allocation.
    }

    content.rendered = result.into();
}
