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

            // TO DO (https://github.com/scouten/asciidoc-parser/issues/335):
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
        r#"(?msx)
        ( ^ | link: | [\ \t] | \\?&lt;() | [>\(\)\[\];"'] )   # capture group 1: prefix
                                                              # capture group 2: flag for prefix == "&lt;"
        ( \\? (?: https? | file | ftp | irc ):// )            # capture group 3: scheme
        (?:
            ( [^\s\[\]]+ )                                    # capture group 4: target
            \[ ( | .*?[^\\] ) \]                              # capture group 5: attrlist
          | ( [^\s]+? ) &gt;                                  # capture group 6: URL inside <>
          | ( [^\s\[\]<]* ( [^\s,.?!\[\]<\)] ) )              # capture group 7: bare link,
                                                              # capture group 8: trailing char
        )
    "#,
    )
    .unwrap()
});

#[derive(Debug)]
struct InlineLinkReplacer<'p>(&'p Parser<'p>);

impl Replacer for InlineLinkReplacer<'_> {
    fn replace_append(&mut self, caps: &Captures<'_>, dest: &mut String) {
        let mut attrlist = Attrlist::parse(Span::new(""), self.0).item.item;

        if caps.get(2).is_some() && caps.get(5).is_none() {
            // Honor the escapes.
            if caps[1].starts_with('\\') {
                dest.push_str(&caps[0][1..]);
                return;
            }

            if caps[3].starts_with('\\') {
                dest.push_str(&caps[1]);
                dest.push_str(&caps[0][caps[1].len() + 1..]);
                return;
            }

            let Some(link_suffix) = caps.get(6) else {
                dest.push_str(&caps[0]);
                return;
            };

            let target = format!(
                "{scheme}{link_suffix}",
                scheme = &caps[3],
                link_suffix = link_suffix.as_str()
            );

            // TO DO (https://github.com/scouten/asciidoc-parser/issues/335):
            // doc.register :links, target

            let link_text = if self.0.is_attribute_set("hide-uri-scheme") {
                URI_SNIFF.replace_all(&target, "").into_owned()
            } else {
                target.clone()
            };

            let params = LinkRenderParams {
                target,
                link_text,
                extra_roles: vec!["bare"],
                window: None,
                type_: LinkRenderType::Link,
                attrlist: &attrlist,
                parser: self.0,
            };

            self.0.renderer.render_link(&params, dest);

            return;
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
            if prefix == "link:" {
                prefix = "".to_owned();
            }

            if !attrlist.is_empty() {
                link_text = Some(attrlist.as_str().to_owned());
            }
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
                target.truncate(target.len() - 1);
                suffix = tail.to_owned();

                if target.ends_with(')') {
                    target.truncate(target.len() - 1);
                    suffix = format!("){suffix}");
                }
            }
        }

        let mut bare = false;

        let link_text_for_attrlist = link_text.clone().unwrap_or_default();
        let span_for_attrlist = Span::new(&link_text_for_attrlist);
        let mut window: Option<&'static str> = None;

        let link_text = if let Some(mut link_text) = link_text {
            link_text = link_text.replace("\\]", "]");

            if link_text.contains('=') {
                let (lt, attrs) = extract_attributes_from_text(&span_for_attrlist, self.0, None);

                link_text = lt.replace("\\\"", "\"");
                attrlist = attrs; // ???
            }

            if link_text.ends_with('^') {
                link_text.truncate(link_text.len() - 1);
                window = Some("_blank");
            }

            if link_text.is_empty() {
                // NOTE: The modified target will not be a bare URI scheme (e.g., http://) in this case.
                if false {
                    todo!(
                        "link_text = (doc_attrs.key? 'hide-uri-scheme') ? (target.sub UriSniffRx, '') : target"
                    );
                }
                bare = true;
                target.clone()
            } else {
                link_text
            }
        } else {
            // NOTE: The modified target will not be a bare URI scheme (e.g., http://) in this case.
            bare = true;

            if self.0.is_attribute_set("hide-uri-scheme") {
                URI_SNIFF.replace_all(&target, "").into_owned()
            } else {
                target.clone()
            }
        };

        let extra_roles = if bare { vec!["bare"] } else { vec![] };

        // TO DO (https://github.com/scouten/asciidoc-parser/issues/335):
        // doc.register :links, (link_opts[:target] = target)

        dest.push_str(&prefix);

        let params = LinkRenderParams {
            target,
            link_text,
            extra_roles,
            window,
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
        let mut window: Option<&'static str> = None;

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
                link_text.truncate(link_text.len() - 1);
                window = Some("_blank");
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

        // TO DO (https://github.com/scouten/asciidoc-parser/issues/335):
        // doc.register :links, (link_opts[:target] = target)

        let params = LinkRenderParams {
            target,
            link_text: link_text.clone(),
            extra_roles,
            window,
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
            window: None,
            type_: LinkRenderType::Link,
            attrlist: &attrlist,
            parser: self.0,
        };

        self.0.renderer.render_link(&params, dest);
    }
}

static URI_SNIFF: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(r#"^\p{alpha}[\p{alpha}\p{digit}.+-]+:/{0,2}"#).unwrap()
});
