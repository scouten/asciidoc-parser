use std::{borrow::Cow, sync::LazyLock};

use regex::{Captures, Match, Regex, RegexBuilder, Replacer};

use crate::{
    attributes::Attrlist,
    parser::{QuoteScope, QuoteType},
    span::content::SubstitutionGroup,
    Content, Parser, Span,
};

/// Saves the content of one passthrough (`+++` or similarly bracketed) passage
/// for later re-expansion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Passthrough {
    pub(crate) text: String,
    pub(crate) subs: SubstitutionGroup,
    pub(crate) type_: Option<QuoteType>,
    pub(crate) attrlist: Option<String>,
}

/// Saves content of passthrough (`+++`-bracketed) passages for later
/// re-expansion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Passthroughs(pub(crate) Vec<Passthrough>);

impl Passthroughs {
    pub(crate) fn extract_from(content: &mut Content<'_>) -> Self {
        let mut passthroughs = Self(vec![]);

        // TRANSLATION GUIDE:
        // * compat_mode => always false
        // * passthroughs => self.saved_spans
        // * old_behavior => appears to affect the entire span

        {
            let text = content.rendered.as_ref();
            if text.contains("++") || text.contains("$$") || text.contains("ss:") {
                let replacer = InlinePassMacroReplacer(&mut passthroughs);

                if let Cow::Owned(new_result) = INLINE_PASS_MACRO.replace_all(text, replacer) {
                    content.rendered = new_result.into();
                }
            }
        }

        {
            let text = content.rendered.as_ref();
            if text.contains('+') || text.contains("-]") {
                dbg!(&text);
                let replacer = InlinePassReplacer(&mut passthroughs);

                if let Cow::Owned(new_result) = INLINE_PASS.replace_all(text, replacer) {
                    content.rendered = new_result.into();
                }
            }
        }

        if true {
            return passthroughs;
        }

        todo!(
            "{}",
            r###"
                # NOTE we need to do the stem in a subsequent step to allow it to be escaped by the former
                text = text.gsub InlineStemMacroRx do
                  # honor the escape
                  next $&.slice 1, $&.length if $&.start_with? RS

                  if (type = $1.to_sym) == :stem
                    type = STEM_TYPE_ALIASES[@document.attributes['stem']].to_sym
                  end
                  subs = $2
                  content = normalize_text $3, nil, true
                  # NOTE drop enclosing $ signs around latexmath for backwards compatibility with AsciiDoc.py
                  content = content.slice 1, content.length - 2 if type == :latexmath && (content.start_with? '$') && (content.end_with? '$')
                  subs = subs ? (resolve_pass_subs subs) : ((@document.basebackend? 'html') ? BASIC_SUBS : nil)
                  passthrus[passthru_key = passthrus.size] = { text: content, subs: subs, type: type }
                  %(#{PASS_START}#{passthru_key}#{PASS_END})
                end if (text.include? ':') && ((text.include? 'stem:') || (text.include? 'math:'))
            "###
        );

        // passthroughs
    }

    pub(crate) fn restore_to(&self, content: &mut Content<'_>, parser: &Parser<'_>) {
        if self.0.is_empty() {
            return;
        }

        dbg!(&self);

        let replacer = PassthroughRestoreReplacer(self, parser);

        if let Cow::Owned(new_result) =
            PASS_WITH_INDEX.replace_all(content.rendered().as_ref(), replacer)
        {
            content.rendered = new_result.into();
        }
    }

    pub(super) fn push(&mut self, passthrough: Passthrough, dest: &mut String) {
        dbg!(&passthrough);
        dbg!(&dest);

        let index = self.0.len();
        self.0.push(passthrough);
        dbg!(&index);

        dest.push('\u{96}');
        dest.push_str(&format!("{index}"));
        dest.push('\u{97}');
    }
}

/// Matches several variants of the passthrough inline macro, which may span
/// multiple lines.
///
/// ## Examples
///
/// * `+++text+++`
/// * `$$text$$`
/// * `pass:quotes[text]`
///
/// NOTE: We have to support an empty `pass:[]` for compatibility with
/// AsciiDoc.py.
static INLINE_PASS_MACRO: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?xs)
        (?:
            # Optional: attrlist
            (?:
                (\\?)              # Group 1: optional backslash before [
                \[
                    ([^\[\]]+)     # Group 2: attrlist contents
                \]
            )?
            
            (\\{0,2})              # Group 3: optional escape prefix (e.g., \ or \\)

            # Passthrough span delimiters: +++, ++, or $$
            (?:
                (\+\+\+) (.*?) (\+\+\+) |   # Groups 4,5,6: triple plus
                (\+\+)   (.*?) (\+\+)   |   # Groups 7,8,9: double plus
                (\$\$)   (.*?) (\$\$)       # Groups 10,11,12: double dollar
            )

        |

            # Alternative: pass-through directive
            (\\?)                       # Group 13: optional escape before pass
            pass:
                ([a-z]+(?:,[a-z-]+)*)?  # Group 14: optional substitution step list
                \[
                     (|.*?[^\\])        # Group 15: optional content
                                        # (avoiding escape of trailing bracket)
                \]
        )"#,
    )
    .unwrap()
});

#[derive(Debug)]
struct InlinePassMacroReplacer<'p>(&'p mut Passthroughs);

impl Replacer for InlinePassMacroReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        dbg!(&caps);

        if caps.get(4).is_some() {
            // +++
            self.handle_quoted_text(caps, 5, dest);
        } else if caps.get(7).is_some() {
            // ++
            self.handle_quoted_text(caps, 8, dest);
        } else if caps.get(10).is_some() {
            // %%
            self.handle_quoted_text(caps, 11, dest);
        } else {
            // NOTE: We don't look for nested `pass:[]` macros.

            if caps.get(13).is_some_and(|m| !m.as_str().is_empty()) {
                // Honor escape of `pass:` macro.
                dest.push_str("pass:");
                if let Some(subs) = caps.get(14) {
                    dest.push_str(subs.as_str());
                }
                dest.push('[');
                dest.push_str(&caps[15]);
                dest.push(']');
                return;
            }

            dbg!(&caps);

            let subs = caps
                .get(14)
                .and_then(|m| SubstitutionGroup::from_custom_string(m.as_str()))
                .unwrap_or(SubstitutionGroup::Normal);

            self.0.push(
                Passthrough {
                    text: normalize_text(&caps[15], false, true),
                    subs,
                    type_: None,
                    attrlist: None,
                },
                dest,
            );
        }
    }
}

impl InlinePassMacroReplacer<'_> {
    fn handle_quoted_text(
        &mut self,
        caps: &Captures<'_>,
        quoted_text_index: usize,
        dest: &mut String,
    ) {
        dbg!(&self);
        dbg!(&caps);

        let escape_count = caps.get(3).map_or(0, |m| m.len());

        let boundary = caps.get(4).or_else(|| caps.get(7)).or_else(|| caps.get(10));
        let boundary = boundary.map(|m| m.as_str()).unwrap_or_default();
        dbg!(&boundary);

        let quoted_text = caps.get(5).or_else(|| caps.get(8)).or_else(|| caps.get(11));
        let quoted_text = quoted_text.map(|m| m.as_str()).unwrap_or_default();

        let mut preceding: Option<String> = None;
        let mut old_behavior = false;

        let attrlist: Option<String> = if let Some(attrlist) = caps.get(2) {
            let attrlist = attrlist.as_str();

            if escape_count > 0 {
                dest.push_str(caps[1].as_ref());
                dest.push('[');
                dest.push_str(caps[2].as_ref());
                dest.push(']');
                dest.push_str(&("\\".repeat(escape_count - 1)));
                dest.push_str(caps[quoted_text_index - 1].as_ref());
                dest.push_str(caps[quoted_text_index].as_ref());
                dest.push_str(caps[quoted_text_index - 1].as_ref());
                return;
            }

            if &caps[1] == "\\" {
                preceding = Some(format!("[{attrlist}]", attrlist = &caps[2]));
                None
            } else if boundary == "++" {
                if attrlist == "x-" {
                    old_behavior = true;
                    Some("".to_owned())
                } else if attrlist.ends_with(" x-") {
                    old_behavior = true;
                    Some(attrlist[0..attrlist.len() - 3].to_owned())
                } else {
                    Some(attrlist.to_owned())
                }
            } else {
                Some(attrlist.to_owned())
            }
        } else if escape_count > 0 {
            // NOTE: We don't look for nested unconstrained pass macros.
            dest.push_str(&("\\".repeat(escape_count - 1)));
            dest.push_str(boundary);
            dest.push_str(quoted_text);
            dest.push_str(boundary);
            return;
        } else {
            None
        };

        let passthrough = if let Some(attrlist) = attrlist {
            if old_behavior {
                Passthrough {
                    text: caps
                        .get(quoted_text_index)
                        .map(|m| m.as_str().to_owned())
                        .unwrap_or_default(),
                    subs: SubstitutionGroup::Normal,
                    type_: Some(QuoteType::Monospaced),
                    attrlist: Some(attrlist),
                }
            } else {
                Passthrough {
                    text: caps
                        .get(quoted_text_index)
                        .map(|m| m.as_str().to_owned())
                        .unwrap_or_default(),
                    subs: if boundary == "+++" {
                        SubstitutionGroup::None
                    } else {
                        SubstitutionGroup::Verbatim
                    },
                    type_: Some(QuoteType::Unquoted),
                    attrlist: Some(attrlist),
                }
            }
        } else {
            Passthrough {
                text: caps
                    .get(quoted_text_index)
                    .map(|m| m.as_str().to_owned())
                    .unwrap_or_default(),
                subs: if boundary == "+++" {
                    SubstitutionGroup::None
                } else {
                    SubstitutionGroup::Verbatim
                },
                type_: None,
                attrlist: None,
            }
        };

        dbg!(&passthrough);
        self.0.push(passthrough, dest);
    }
}

static PASS_WITH_INDEX: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new("\u{96}(\\d+)\u{97}").unwrap()
});

/// Matches an inline passthrough, which may span multiple lines.
///
/// ## Examples
///
/// * `+text+`
/// * `[x-]+text+`
/// * `[x-]\`text\``
///
/// NOTE: We do not support compat-mode in the Rust implementation.
static INLINE_PASS: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?xs)
            \b{start-half}                # Must not follow a word
                                          # Separately (enforce in code, must not
                                          # follow ;, :, or \)

            (?:                           # Option 1: attrlist + quoted text
                \[([^\[\]]+)\]            # Group 1: [attrlist] -- required for backtick passthroughs
                (\\{0,2})                 # Group 2: optional escapes
                (?:
                    \+(\S(?:.*?\S)?)\+    # Group 3: +...+ content (surrounded by non-space)
                  | \`(\S(?:.*?\S)?)\`    # Group 4: `...` content -- only allowed with attrlist
                )

            |                             # --OR--
                                          # Option 2: +...+ content without attrlist

                (\\{0,2})                 # Group 5: optional escapes (no attrlist branch)
                \+(\S(?:.*?\S)?)\+        # Group 6: +...+ content (no attrlist required)
            )

            \b{end-half}                  # Must not be followed by a word character
        "#,
    )
    .unwrap()
});

#[derive(Debug)]
struct InlinePassReplacer<'p>(&'p mut Passthroughs);

impl Replacer for InlinePassReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        dbg!(&dest);
        dbg!(&caps);

        let escapes = caps.get(2).or_else(|| caps.get(5));
        let escape_count = escapes.map_or(0, |m| m.len());

        let orig_attrlist_body = caps.get(1).map(|m| m.as_str());
        dbg!(&orig_attrlist_body);

        let (attrlist_body, old_behavior) = caps.get(1).map_or((None, false), |m| {
            let body = m.as_str();
            if body == "x-" {
                (Some("".to_string()), true)
            } else if body.ends_with(" x-") {
                (Some(body[0..body.len() - 3].to_string()), true)
            } else {
                (Some(body.to_string()), false)
            }
        });

        dbg!(&attrlist_body);
        dbg!(&old_behavior);
        // ALSO: old_behavior_forced

        let quoted_text = caps.get(3).or_else(|| caps.get(4)).or_else(|| caps.get(6));
        let quoted_text = quoted_text.map_or("", |m| m.as_str());
        dbg!(quoted_text);

        let format_mark = if caps.get(4).is_some() { '`' } else { '+' };

        if !old_behavior && format_mark == '`' {
            // TO DO: Review whether this is still needed.

            // The Rust version of the INLINE_PASS regex can't quite as nuanced as the
            // original Ruby version due to the lack of lookaround support. We compensate by
            // restoring the original text when we get false positives (notably
            // backtick-wrapped code snippets, which will get translated later by the quotes
            // substition step).
            dest.push_str(&caps[0]);
            return;
        }

        if let Some(ref orig_attrlist_body) = orig_attrlist_body {
            dbg!(&orig_attrlist_body);

            if escape_count > 0 {
                // Honor the escape of the formatting mark.
                dest.push('[');
                dest.push_str(orig_attrlist_body);
                dest.push(']');
                dest.push_str(&("\\".repeat(escape_count - 1)));
                dest.push(format_mark);
                dest.push_str(quoted_text);
                dest.push(format_mark);
                return;
            }

            if dest.ends_with('\\') {
                if old_behavior && format_mark == '`' {
                    // Honor the escape of the attributes.
                    dest.push('[');
                    dest.push_str(orig_attrlist_body);
                    dest.push(']');
                    dest.push_str(quoted_text);
                    return;
                }

                // I don't understand this:
                todo!("{}", "preceding = %([#{attrlist}])")
            }
        } else if escape_count > 0 {
            // Honor the escape of the formatting mark.
            dest.push_str(&("\\".repeat(escape_count - 1)));
            dest.push_str(quoted_text);
            return;
        };

        let subs = if attrlist_body.is_some() && old_behavior && format_mark == '`' {
            SubstitutionGroup::Normal
        } else {
            SubstitutionGroup::Verbatim
        };

        let type_ = if attrlist_body.is_some() {
            if old_behavior {
                Some(QuoteType::Monospaced)
            } else {
                Some(QuoteType::Unquoted)
            }
        } else {
            None
        };

        eprintln!("replace_append:445");
        self.0.push(
            Passthrough {
                text: quoted_text.to_string(),
                subs,
                type_,
                attrlist: attrlist_body,
            },
            dest,
        );
    }
}

#[derive(Debug)]
struct PassthroughRestoreReplacer<'p>(&'p Passthroughs, &'p Parser<'p>);

impl Replacer for PassthroughRestoreReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        dbg!(&self);
        dbg!(&caps);
        dbg!(&dest);

        let index = caps[1].parse::<usize>().unwrap_or_default();

        dbg!(index);

        if let Some(pass) = self.0 .0.get(index) {
            let span = Span::new(&pass.text);
            dbg!(&span);

            let mut subbed_text = Content::from(span);
            pass.subs.apply(&mut subbed_text, self.1, None);

            if let Some(type_) = pass.type_ {
                dbg!(type_);
                let attrlist = pass.attrlist.as_ref().map(|attrlist_body| {
                    let span = Span::new(attrlist_body);
                    let maw = Attrlist::parse(span);
                    maw.item.item
                });

                dbg!(&attrlist);

                let id = if let Some(attrlist) = attrlist.as_ref() {
                    attrlist.id().map(|a| a.data().to_string())
                } else {
                    None
                };
                dbg!(&id);

                let mut new_text = String::default();
                self.1.renderer.render_quoted_substitition(
                    type_,
                    QuoteScope::Unconstrained,
                    attrlist,
                    id,
                    &pass.text,
                    &mut new_text,
                );

                subbed_text.rendered = new_text.into();

                dbg!(&dest);
            }

            dbg!(&pass);

            if let Some(type_) = pass.type_ {
                let attrlist = pass.attrlist.as_ref().map(|attrlist_body| {
                    let span = Span::new(&attrlist_body);
                    let maw = Attrlist::parse(span);
                    maw.item.item
                });

                let id = attrlist
                    .as_ref()
                    .and_then(|attrlist| attrlist.id().map(|id| id.to_string()));

                let mut new_text = String::default();
                self.1.renderer.render_quoted_substitition(
                    type_,
                    QuoteScope::Unconstrained,
                    attrlist,
                    id,
                    &pass.text,
                    &mut new_text,
                );

                subbed_text.rendered = new_text.into();
            }

            if subbed_text.rendered().contains('\u{96}') {
                // Recursively apply passthrough replacement and write the result.
                let replacer = PassthroughRestoreReplacer(self.0, self.1);

                let new_result =
                    PASS_WITH_INDEX.replace_all(subbed_text.rendered().as_ref(), replacer);

                dest.push_str(new_result.as_ref());
            } else {
                dest.push_str(subbed_text.rendered());
            }
        } else {
            todo!(
                "{}",
                r#"
              logger.error %(unresolved passthrough detected: #{text})
              '??pass??'
            "#
            );
        }
    }
}

/// Normalize text to prepare it for parsing.
///
/// If `normalize_whitespace` is true, strip surrounding whitespace and fold
/// newlines. If `unescape_closing_square_bracket` is true, unescape any escaped
/// closing square brackets.
///
/// Returns the normalized text string.
fn normalize_text(
    text: &str,
    normalize_whitespace: bool,
    unescape_closing_square_brackets: bool,
) -> String {
    if text.is_empty() {
        return "".to_string();
    }

    let text = if normalize_whitespace {
        let text = text.trim();
        text.replace('\n', " ")
    } else {
        text.to_string()
    };

    if unescape_closing_square_brackets {
        text.replace("\\]", "]")
    } else {
        text
    }
}
