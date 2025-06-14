use std::{borrow::Cow, sync::LazyLock};

use regex::{Captures, Match, Regex, RegexBuilder, Replacer};

use crate::{attributes::Attrlist, span::content::SubstitutionGroup, Content, Span};

/// Saves the content of one passthrough (`+++` or similarly bracketed) passage
/// for later re-expansion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Passthrough<'src> {
    pub(crate) text: String,
    pub(crate) subs: SubstitutionGroup,
    // pub(crate) type_: what is this?,
    pub(crate) attrlist: Option<Attrlist<'src>>,
}

/// Saves content of passthrough (`+++`-bracketed) passages for later
/// re-expansion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Passthroughs<'src>(pub(crate) Vec<Passthrough<'src>>);

impl<'src> Passthroughs<'src> {
    pub(crate) fn extract_from(content: &mut Content<'src>) -> Self {
        let mut passthroughs = Self(vec![]);

        // TRANSLATION GUIDE:
        // * compat_mode => always false
        // * passthroughs => self.saved_spans

        let mut result: Cow<'_, str> = content.rendered.to_string().into();

        {
            let text = result.as_ref();
            if text.contains("++") || text.contains("$$") || text.contains("ss:") {
                let replacer = InlinePassReplacer(&mut passthroughs);

                if let Cow::Owned(new_result) = INLINE_PASS_MACRO.replace_all(text, replacer) {
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
			pass_inline_char1, pass_inline_char2, pass_inline_rx = InlinePassRx[compat_mode]
			text = text.gsub pass_inline_rx do
			  preceding = $1
			  attrlist = $4 || $3
			  escaped = true if $5
			  quoted_text = $6
			  format_mark = $7
			  content = $8
		
			  if compat_mode
				old_behavior = true
			  elsif attrlist && (attrlist == 'x-' || (attrlist.end_with? ' x-'))
				old_behavior = old_behavior_forced = true
			  end
		
			  if attrlist
				if escaped
				  # honor the escape of the formatting mark
				  next %(#{preceding}[#{attrlist}]#{quoted_text.slice 1, quoted_text.length})
				elsif preceding == RS
				  # honor the escape of the attributes
				  next %(#{preceding}[#{attrlist}]#{quoted_text}) if old_behavior_forced && format_mark == '`'
				  preceding = %([#{attrlist}])
				elsif old_behavior_forced
				  attributes = attrlist == 'x-' ? {} : (parse_quoted_text_attributes attrlist.slice 0, attrlist.length - 3)
				else
				  attributes = parse_quoted_text_attributes attrlist
				end
			  elsif escaped
				# honor the escape of the formatting mark
				next %(#{preceding}#{quoted_text.slice 1, quoted_text.length})
			  elsif compat_mode && preceding == RS
				next quoted_text
			  end
		
			  if compat_mode
				passthrus[passthru_key = passthrus.size] = { text: content, subs: BASIC_SUBS, attributes: attributes, type: :monospaced }
			  elsif attributes
				if old_behavior
				  subs = format_mark == '`' ? BASIC_SUBS : NORMAL_SUBS
				  passthrus[passthru_key = passthrus.size] = { text: content, subs: subs, attributes: attributes, type: :monospaced }
				else
				  passthrus[passthru_key = passthrus.size] = { text: content, subs: BASIC_SUBS, attributes: attributes, type: :unquoted }
				end
			  else
				passthrus[passthru_key = passthrus.size] = { text: content, subs: BASIC_SUBS }
			  end
		
			  %(#{preceding}#{PASS_START}#{passthru_key}#{PASS_END})
			end if (text.include? pass_inline_char1) || (pass_inline_char2 && (text.include? pass_inline_char2))
		
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
		
			text
            "###
        );

        // passthroughs
    }

    pub(super) fn restore_to(&self, content: &mut Content<'src>) {
        if !self.0.is_empty() {
            todo!("Restore!");
        }
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
    RegexBuilder::new(
        r#"(?x)
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
                ([a-z]+(?:,[a-z-]+)*)?  # Group 14: optional language list
                \[
                    (|.*?[^\\])         # Group 15: optional content, not ending in \
                \]
        )"#,
    )
    .dot_matches_new_line(true)
    .build()
    .unwrap()
});

#[derive(Debug)]
struct InlinePassReplacer<'r, 'p>(&'p mut Passthroughs<'r>);

impl Replacer for InlinePassReplacer<'_, '_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
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
            // pass:[]

            // TRANSLATION GUIDE:
            // * compat_mode => always false
            // * passthroughs => self.saved_spans

            todo!(
                "{}",
                r###"
		  else # pass:[]
			# NOTE we don't look for nested pass:[] macros
			# honor the escape
			next $&.slice 1, $&.length if $6 == RS
			if (subs = $7)
			  passthrus[passthru_key = passthrus.size] = { text: (normalize_text $8, nil, true), subs: (resolve_pass_subs subs) }
			else
			  passthrus[passthru_key = passthrus.size] = { text: (normalize_text $8, nil, true) }
			end
		  end
	
		  %(#{preceding || ''}#{PASS_START}#{passthru_key}#{PASS_END})
        "###
            );
        }
    }
}

impl<'r> InlinePassReplacer<'_, 'r> {
    fn handle_quoted_text(
        &mut self,
        caps: &Captures<'_>,
        quoted_text_index: usize,
        dest: &mut String,
    ) {
        dbg!(&self);
        dbg!(&caps);

        let escape_count = caps.get(3).map_or(0, |m| m.len());

        let mut old_behavior = false;
        let mut attrlist: Option<Attrlist<'r>> = None;

        if let Some(attrlist) = caps.get(2) {
            todo!(
                "{}",
                r###"
			  if (escape_count = $3.length) > 0
				# NOTE we don't look for nested unconstrained pass macros
				next %(#{$1}[#{attrlist}]#{RS * (escape_count - 1)}#{boundary}#{$5}#{boundary})
			  elsif $1 == RS
				preceding = %([#{attrlist}])
			  elsif boundary == '++'
				if attrlist == 'x-'
				  old_behavior = true
				  attributes = {}
				elsif attrlist.end_with? ' x-'
				  old_behavior = true
				  attributes = parse_quoted_text_attributes attrlist.slice 0, attrlist.length - 3
				else
				  attributes = parse_quoted_text_attributes attrlist
				end
			  else
				attributes = parse_quoted_text_attributes attrlist
			  end
        "###
            );
        } else if escape_count > 0 {
            todo!(
                "{}",
                r###"
			elsif (escape_count = $3.length) > 0
			  # NOTE we don't look for nested unconstrained pass macros
			  next %(#{RS * (escape_count - 1)}#{boundary}#{$5}#{boundary})
			end
        "###
            );
        }

        if let Some(attrlist) = attrlist {
            todo!(
                "{}",
                r###"
			  if old_behavior
				passthrus[passthru_key = passthrus.size] = { text: $5, subs: NORMAL_SUBS, type: :monospaced, attributes: attributes }
			  else
				passthrus[passthru_key = passthrus.size] = { text: $5, subs: subs, type: :unquoted, attributes: attributes }
			  end
        "###
            );
        } else {
            self.0 .0.push(Passthrough {
                text: caps
                    .get(quoted_text_index)
                    .map(|m| m.as_str().to_owned())
                    .unwrap_or_default(),
                subs: SubstitutionGroup::Verbatim,
                attrlist: None,
            });
            //   passthrus[passthru_key = passthrus.size] = { text: $5, subs:
            // subs }
        }

        dest.push('\u{96}');
        dest.push_str(&format!("{}", self.0 .0.len() - 1));
        dest.push('\u{97}');
    }
}
