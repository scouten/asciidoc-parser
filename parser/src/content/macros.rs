use std::{borrow::Cow, path::Path, sync::LazyLock};

use percent_encoding::{AsciiSet, CONTROLS, utf8_percent_encode};
use regex::{Captures, Regex, Replacer};

use crate::{
    Parser, Span,
    attributes::Attrlist,
    content::Content,
    parser::{IconRenderParams, ImageRenderParams, LinkRenderParams, LinkRenderType},
};

pub(super) fn apply_macros(content: &mut Content<'_>, parser: &'_ Parser) {
    let /* mut */ text = content.rendered().to_string();
    let found_square_bracket = text.contains('[');
    let found_colon = text.contains(':');
    let found_macroish = found_square_bracket && found_colon;
    // let found_macroish_short = found_macroish && text.contains(":[");

    // TO DO (#262): Implement extensions that can define macros.
    // Port Ruby Asciidoctor's implementation from
    // https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/substitutors.rb#L306-L347.

    // TO DO (#263): Implement `kbd:` and `btn:` macros.
    // Port Ruby Asciidoctor's implementation from
    // https://github.com/asciidoctor/asciidoctor/blob/main/lib/asciidoctor/substitutors.rb#L349-L377.

    if found_macroish && (text.contains("image:") || text.contains("icon:")) {
        let replacer = InlineImageMacroReplacer(parser);

        if let Cow::Owned(new_result) = INLINE_IMAGE_MACRO.replace_all(content.rendered(), replacer)
        {
            content.rendered = new_result.into();
        }
    }

    /*
    if (text.contains("((") && text.contains("))"))
        || (found_macroish_short && text.contains("dexterm"))
    {
        todo!("Index term macro");
        // Port Ruby Asciidoctor's implementation from lines 439..536.
    }
    */

    if found_colon && text.contains("://") {
        let replacer = InlineLinkReplacer(parser);

        if let Cow::Owned(new_result) = INLINE_LINK.replace_all(content.rendered(), replacer) {
            content.rendered = new_result.into();
        }
    }

    if found_macroish && (text.contains("link:") || text.contains("ilto:")) {
        let replacer = InlineLinkMacroReplacer(parser);

        if let Cow::Owned(new_result) = INLINE_LINK_MACRO.replace_all(content.rendered(), replacer)
        {
            content.rendered = new_result.into();
        }
    }

    if text.contains('@') {
        let replacer = InlineEmailReplacer(parser);

        if let Cow::Owned(new_result) = INLINE_EMAIL.replace_all(content.rendered(), replacer) {
            content.rendered = new_result.into();
        }
    }

    /*
    if
    /* found_square_bracket && */
    false {
        // 'false' should be replaced with @context == :list_item && @parent.style ==
        // 'bibliography'.
        todo!("Port bibliography reference macro");
        // Port Ruby Asciidoctor's implementation from lines 719..721.
    }

    if (found_square_bracket && text.contains("[[")) || (found_macroish && text.contains("or:")) {
        todo!("Port inline anchor macro");
        // Port Ruby Asciidoctor's implementation from lines 723..739.
    }

    if (text.contains('&') && text.contains(";&l") || (found_macroish && text.contains("xref:"))) {
        todo!("Port cross-reference macro");
        // Port Ruby Asciidoctor's implementation from lines 742..840.
    }

    if found_macroish && text.contains("tnote") {
        todo!("Port footnote macro");
        // Port Ruby Asciidoctor's implementation from lines 842..884.
    }
    */
}

static INLINE_IMAGE_MACRO: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?xs)                    
            \\?                         # Optional escape: literal backslash
            i(?:mage|con):              # 'image:' or 'icon:' prefix

            (                           # Group 1: the target
                [^:\s\[\n]                  # First char: not colon, whitespace, [, or newline
                [^\[\n]*?                   # Middle chars: any except [ or newline, lazily
                [^\s\[\n]                   # Last char: not whitespace, [, or newline
            )?                          # Entire target group is optional

            \[                          # Opening square bracket

            (                           # Group 2: bracketed text
                |                       #   EITHER: empty alt text
                .*?[^\\]                #   OR: content ending in a non-backslash
            )

            \]                          # Closing square bracket
        "#,
    )
    .unwrap()
});

#[derive(Debug)]
struct InlineImageMacroReplacer<'p>(&'p Parser<'p>);

impl Replacer for InlineImageMacroReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        if caps[0].starts_with('\\') {
            // Honor the escape.
            dest.push_str(&caps[0][1..]);
            return;
        }

        let target = &caps[1];
        let span = Span::new(&caps[2]);
        let attrlist = Attrlist::parse(span, self.0).item.item;

        let default_alt = basename(&target.replace(['_', '-'], " "));
        // IMPORTANT: Implementations of `render_icon` and `render_image` need to
        // remember to use `default_alt` when attrlist doesn't contain a value for
        // `alt`.

        if caps[0].starts_with("image:") {
            // TO DO: Register image with parser?
            // IMPORTANT: May require interior mutability on Parser because it looks like we
            // can't pass mutable references to Parser in a recursive Regex replacement.
            // todo!("Port this: {}", "doc.register :images, target");

            let params = ImageRenderParams {
                target,
                alt: attrlist
                    .named_or_positional_attribute("alt", 1)
                    .map_or(default_alt, |a| {
                        normalize_text_lf_escaped_bracket(a.value())
                    }),
                width: attrlist
                    .named_or_positional_attribute("width", 2)
                    .map(|a| a.value()),
                height: attrlist
                    .named_or_positional_attribute("height", 3)
                    .map(|a| a.value()),
                attrlist: &attrlist,
                parser: self.0,
            };

            self.0.renderer.render_image(&params, dest);
        } else {
            let params = IconRenderParams {
                target,
                alt: attrlist.named_attribute("alt").map_or(default_alt, |a| {
                    normalize_text_lf_escaped_bracket(a.value())
                }),
                size: attrlist
                    .named_or_positional_attribute("size", 1)
                    .map(|a| a.value()),
                attrlist: &attrlist,
                parser: self.0,
            };

            self.0.renderer.render_icon(&params, dest);
        }
    }
}

fn basename(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or_default()
        .to_string()
}

fn normalize_text_lf_escaped_bracket(text: &str) -> String {
    text.replace("\n", " ").replace("\\]", "]")
}

static INLINE_LINK: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?mx)
      ( ^ | link: | [ \t] | \\?&lt;() | [>\(\)\[\];"'] )    # capture group 1: prefix
                                                            # capture group 2: flag for prefix == "&lt;"
      ( \\? (?: https? | file | ftp | irc ):// )            # capture group 3: scheme
      (?:
          ( [^\s\[\]]+ )                                    # capture group 4: target
          \[ ( | .*?[^\\] ) \]                              # capture group 5: attrlist
        | ( \\?(?:https?|file|ftp|irc):// [^\s]+? ) &gt;    # capture group 6: URL inside <>
        | ( [^\s\[\]<]* ( [^\s,.?!\[\]<\)] ) )              # capture group 7: bare link,
                                                            # capture group 8: trailing char
      )
    "#,
    )
    .unwrap()
});

#[derive(Debug)]
#[allow(unused)] // TEMPORARY
struct InlineLinkReplacer<'p>(&'p Parser<'p>);

impl Replacer for InlineLinkReplacer<'_> {
    #[allow(unused)] // TEMPORARY
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        dbg!(&caps);

        let mut attrlist = Attrlist::parse(Span::new(""), self.0).item.item;

        if caps.get(2).is_some() && caps.get(5).is_none() {
            todo!(
                "Port this: {}",
                r#"
                    # honor the escapes
                    next $&.slice 1, $&.length if $1.start_with? RS
                    next %(#{$1}#{$&.slice $1.length + 1, $&.length}) if $3.start_with? RS
                    next $& unless $6
                    doc.register :links, (target = $3 + $6)
                    link_text = (doc_attrs.key? 'hide-uri-scheme') ? (target.sub UriSniffRx, '') : target
                    (Inline.new self, :anchor, link_text, type: :link, target: target, attributes: { 'role' => 'bare' }).convert
"#
            );
        }

        let mut prefix = caps[1].to_string();
        let scheme = &caps[3];

        // Honor the escape.
        if scheme.starts_with('\\') {
            dest.push_str(&prefix);
            dest.push_str(&caps[0][prefix.len() + 1..]);
            return;
        }

        let mut target = format!(
            "{scheme}{link_text}",
            link_text = caps.get(4).map(|m| m.as_str()).unwrap_or_else(|| &caps[7])
        );

        let mut suffix = "".to_owned();
        let mut link_text: Option<String> = None;

        // NOTE: If capture group 5 exists (the attrlist), we're looking at a formal macro (e.g., https://example.org[]).
        if let Some(attrlist) = caps.get(5) {
            todo!(
                "Port this: {}",
                r#"
                    prefix = '' if prefix == 'link:'
                    link_text = nil if (link_text = $5).empty?
                    "#
            );
        } else {
            if prefix == "link" || prefix == "\"" || prefix == "'" {
                // Note from the Ruby implementation which also applies to this if clause:

                // Invalid macro syntax (link: prefix w/o trailing square brackets or URL
                // enclosed in quotes).

                // FIXME: We probably shouldn't even get here when the link: prefix is present.
                // The regex is doing too much.
                dest.push_str(&caps[0]);
                return;
            }

            let tail = &caps[8];
            if tail == ";" || tail == ":" {
                // Move trailing semicolon or colon and adjacent ) if it exists
                // out of the URL.
                if target.ends_with(')') {
                    target.truncate(target.len() - 2);
                    suffix = format!("){tail}");
                } else {
                    target.truncate(target.len() - 1);
                    suffix = tail.to_string();
                }
            }
        }

        let mut bare = false;

        let link_text = if let Some(link_text) = link_text {
            todo!(
                "Port this: {}",
                r#"
                    new_link_text = link_text = link_text.gsub ESC_R_SB, R_SB if link_text.include? R_SB
                    if !doc.compat_mode && (link_text.include? '=')
                        # NOTE if an equals sign (=) is present, extract attributes from link text
                        link_text, attrs = extract_attributes_from_text link_text, ''
                        new_link_text = link_text
                        link_opts[:id] = attrs['id']
                    end

                    if link_text.end_with? '^'
                        new_link_text = link_text = link_text.chop
                        if attrs
                            attrs['window'] ||= '_blank'
                        else
                            attrs = { 'window' => '_blank' }
                        end
                    end

                    if new_link_text && new_link_text.empty?
                        # NOTE the modified target will not be a bare URI scheme (e.g., http://) in this case
                        link_text = (doc_attrs.key? 'hide-uri-scheme') ? (target.sub UriSniffRx, '') : target
                        bare = true
                    end
                "#
            );
        } else {
            // NOTE: The modified target will not be a bare URI scheme (e.g., http://) in this case.
            bare = true;

            if false {
                todo!(
                    "Port this {}",
                    r#"(doc_attrs.key? 'hide-uri-scheme') ? (target.sub UriSniffRx, '') : target"#
                );
            }
            target.clone()
        };

        let extra_roles = if bare { vec!["bare"] } else { vec![] };

        if false {
            todo!(
                "Port this: {}",
                r#"
            doc.register :links, (link_opts[:target] = target)
            "#
            );
        }

        dest.push_str(&prefix);

        let params = LinkRenderParams {
            target,
            link_text,
            extra_roles,
            type_: LinkRenderType::Link,
            attrlist: &attrlist,
            parser: self.0,
        };

        self.0.renderer.render_link(&params, dest);

        dest.push_str(&suffix);
    }
}

static INLINE_LINK_MACRO: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?xs)                # (?x) extended mode, (?s) dot matches newline

        \\?                     # Optional backslash escape before macro

        (?:                     # Non-capturing group for macro name
            link                #   'link'
          | (mailto)            #   capture group 1: 'mailto'
        )

        :                       # Colon after macro name

        (?:                     # Non-capturing outer group
            ().                 #   capture group 2: empty target
          | ([^:\s\[] [^\s\[]*) #   capture group 3: valid target (no colon/space/'[')
        )

        \[                      # Opening square bracket

        (?:                     # Non-capturing outer group
            ()                  #   capture group 4: empty label
          | (.*?[^\\])          #   capture group 5: minimally match anything, not ending in '\'
        )

        \]                      # Closing square bracket
    "#,
    )
    .unwrap()
});

#[derive(Debug)]
struct InlineLinkMacroReplacer<'p>(&'p Parser<'p>);

impl Replacer for InlineLinkMacroReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        if caps[0].starts_with('\\') {
            // Honor the escape.
            dest.push_str(&caps[0][1..]);
            return;
        }

        let (mailto, mailto_text, mut target) = if caps.get(1).is_some() {
            let mailto_text = &caps[3];
            (
                caps.get(1).map(|c| c.as_str()),
                Some(mailto_text),
                format!("mailto:{mailto_text}"),
            )
        } else {
            (None, None, caps[3].to_string())
        };

        let mut attrlist: Option<Attrlist<'_>> = None;
        let link_type = LinkRenderType::Link;

        let mut link_text = caps
            .get(5)
            .map(|c| c.as_str().to_string())
            .unwrap_or_default();

        let link_text_for_attrlist = link_text.replace("\n", " ");
        let span_for_attrlist = Span::new(&link_text_for_attrlist);

        if !link_text.is_empty() {
            link_text = link_text.replace("\\]", "]");

            if let Some(_mailto) = mailto {
                if link_text.contains(',') {
                    let (lt, attrs) =
                        extract_attributes_from_text(&span_for_attrlist, self.0, None);

                    link_text = lt;

                    if let Some(target_attr) = attrs.nth_attribute(2) {
                        target = format!(
                            "{target}?subject={subject}",
                            subject = encode_uri_component(target_attr.value())
                        );

                        if let Some(body) = attrs.nth_attribute(3) {
                            target = format!(
                                "{target}&amp;body={body}",
                                body = encode_uri_component(body.value())
                            );
                        }
                    }

                    attrlist = Some(attrs);
                }
            } else if link_text.contains('=') {
                let (lt, attrs) = extract_attributes_from_text(&span_for_attrlist, self.0, None);
                link_text = lt;

                attrlist = Some(attrs);
            }

            if link_text.ends_with('^') {
                todo!(
                    "Port this: {}",
                    r#"
                    link_text = link_text.chop
                    if attrs
                        attrs['window'] ||= '_blank'
                    else
                        attrs = { 'window' => '_blank' }
                    end
                "#
                );
            }
        }

        let attrlist = if let Some(attrlist) = attrlist {
            attrlist
        } else {
            Attrlist::parse(Span::new(""), self.0).item.item
        };

        let mut extra_roles: Vec<&str> = vec![];

        if link_text.is_empty() {
            // mailto is a special case; already processed.
            if let Some(_mailto) = mailto {
                link_text = match mailto_text {
                    Some(txt) => txt.to_owned(),
                    None => "".to_owned(),
                };
            } else {
                if false {
                    // Skip for the moment?
                    todo!(
                        "Port this: {}",
                        r#"
                        if doc_attrs.key? 'hide-uri-scheme'
                            if (link_text = target.sub UriSniffRx, '').empty?
                                link_text = target
                            end
                        else
                            link_text = target
                        end
                        "#
                    );
                }
                link_text = target.clone();
                extra_roles.push("bare");
            }
        }

        if false {
            // Skipping for now.
            todo!("doc.register :links, (link_opts[:target] = target)");
        }

        let params = LinkRenderParams {
            target,
            link_text: link_text.clone(),
            extra_roles,
            type_: link_type,
            attrlist: &attrlist,
            parser: self.0,
        };

        self.0.renderer.render_link(&params, dest);
    }
}

/// This function is used in cases when the attrlist can be mixed with the text
/// of a macro. If no attributes are detected aside from the first positional
/// attribute, and the first positional attribute matches the attrlist, then the
/// original text is returned.
///
/// Precondition: Any new-line characters (`\n`) must be replaced with spaces
/// prior to calling this function.
fn extract_attributes_from_text<'src>(
    text: &'src Span<'src>,
    parser: &Parser,
    default_text: Option<&str>,
) -> (String, Attrlist<'src>) {
    let attrlist_maw = Attrlist::parse(*text, parser);
    let attrs = attrlist_maw.item.item;

    if let Some(resolved_text) = attrs.nth_attribute(1) {
        // NOTE: If resolved text remains unchanged, return an empty attribute list and
        // return unparsed text.
        if resolved_text.value() == text.data() {
            const EMPTY_SPAN: &Span = &Span::new("");
            let empty_attrs = Attrlist::parse(*EMPTY_SPAN, parser).item.item;
            (text.data().to_owned(), empty_attrs)
        } else {
            (resolved_text.value().to_owned(), attrs)
        }
    } else {
        let default_text = default_text.map(|s| s.to_string());
        (default_text.unwrap_or_default(), attrs)
    }
}

// Ruby CGI.escape allows A-Z a-z 0-9 *_.-
// It encodes space as '+'. (We'll fix afterward.)
// Start with the standard URL encoding set.
const CGI_ESCAPE_SET: &AsciiSet = &CONTROLS
    .add(b' ') // space
    .add(b'!')
    .add(b'"')
    .add(b'#')
    .add(b'$')
    .add(b'%')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'+') // plus must be escaped
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'<')
    .add(b'=')
    .add(b'>')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b'\\')
    .add(b']')
    .add(b'^')
    .add(b'`')
    .add(b'{')
    .add(b'|')
    .add(b'}');

fn encode_uri_component(s: &str) -> String {
    // First escape with percent-encoding.
    let encoded = utf8_percent_encode(s, CGI_ESCAPE_SET).to_string();

    // Then apply the Ruby `.gsub('+', '%20')` logic.
    // But note: percent-encoding gives us "%20" for space already,
    // so we need to manually *introduce* '+' for space first,
    // then swap them out.
    let with_plus = encoded.replace("%20", "+");
    with_plus.replace('+', "%20")
}

/// Matches an inline e-mail address.
///
/// # Example
/// `doc.writer@example.com`
static INLINE_EMAIL: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)                         # verbose mode (ignore whitespace & comments)

        ([\\>:/]?)                      # capture group 1: prefix that causes mismatch: \, >, :, or /

        (                               # capture group 2: actual e-mail address
            [\w_]                           # leading word character
            (?: &amp; | [\w\-.%+] )*        # subsequent word chars or symbols (&amp;, ., -, %, +)
            @                               # at sign
            [\p{L}\p{Nd}]                   # leading letter or digit in domain
            [\p{L}\p{Nd}_\-.]*              # rest of domain
            \.[a-zA-Z]{2,5}                 # dot + TLD (2–5 ASCII letters)
        )

        \b                              # word boundary
        "#,
    )
    .unwrap()
});

#[derive(Debug)]
#[allow(unused)] // TEMPORARY while building
struct InlineEmailReplacer<'p>(&'p Parser<'p>);

#[allow(unused)] // TEMPORARY while building
impl Replacer for InlineEmailReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        if let Some(escape) = &caps.get(1)
            && !escape.is_empty()
        {
            if escape.as_str() == "\\" {
                dest.push_str(&caps[0][1..]);
            } else {
                dest.push_str(&caps[0]);
            }
            return;
        }

        let target = format!("mailto:{mailto}", mailto = &caps[2]);
        let attrlist = Attrlist::parse(Span::new(""), self.0).item.item;

        let params = LinkRenderParams {
            target: target.clone(),
            link_text: caps[2].to_owned(),
            extra_roles: vec![],
            type_: LinkRenderType::Link,
            attrlist: &attrlist,
            parser: self.0,
        };

        self.0.renderer.render_link(&params, dest);
    }
}
