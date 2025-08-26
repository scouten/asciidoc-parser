use std::{borrow::Cow, path::Path, sync::LazyLock};

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

    if found_colon && text.contains("://") {
        // Don't panic here -- other tests are generating URLs now.
        // todo!("URL macro");
        // Port Ruby Asciidoctor's implementation from lines 538..634.
    }
    */

    if found_macroish && (text.contains("link:") || text.contains("ilto:")) {
        let replacer = InlineLinkMacroReplacer(parser);

        if let Cow::Owned(new_result) = INLINE_LINK_MACRO.replace_all(content.rendered(), replacer)
        {
            content.rendered = new_result.into();
        }
    }

    /*
    if text.contains('@') {
        todo!("Maybe found email macro");
        // Port Ruby Asciidoctor's implementation from lines 706..717.
    }

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

        let (mailto, _mailto_text, target) = if caps.get(1).is_some() {
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
        let mut id: Option<String> = None;

        let mut link_text = caps
            .get(5)
            .map(|c| c.as_str().to_string())
            .unwrap_or_default();

        let link_text_for_attrlist = link_text.clone();

        if !link_text.is_empty() {
            link_text = link_text.replace("\\]", "]");

            if let Some(_mailto) = mailto {
                todo!(
                    "Port this: {}",
                    r#"
                    if !doc.compat_mode && (link_text.include? ',')
                        # NOTE if a comma (,) is present, extract attributes from link text
                        link_text, attrs = extract_attributes_from_text link_text, ''
                        link_opts[:id] = attrs['id']
                        if attrs.key? 2
                            if attrs.key? 3
                                target = %(#{target}?subject=#{Helpers.encode_uri_component attrs[2]}&amp;body=#{Helpers.encode_uri_component attrs[3]})
                            else
                                target = %(#{target}?subject=#{Helpers.encode_uri_component attrs[2]})
                            end
                        end
                    end
                "#
                );
            } else if link_text.contains('=') {
                let link_text = Span::new(&link_text_for_attrlist);
                let attrs = Attrlist::parse(link_text, self.0).item.item;

                if let Some(id_attr) = attrs.named_attribute("id") {
                    id = Some(id_attr.value().to_string());
                }

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

        let mut roles: Vec<&str> = attrlist.roles();

        if link_text.is_empty() {
            // mailto is a special case; already processed.
            if let Some(_mailto) = mailto {
                todo!("link_text = mailto_text");
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

                if !roles.contains(&"bare") {
                    roles.insert(0, "bare");
                }
            }
        }

        if false {
            // Skipping for now.
            todo!("doc.register :links, (link_opts[:target] = target)");
        }

        let params = LinkRenderParams {
            target,
            link_text: link_text.clone(),
            id,
            roles,
            type_: link_type,
            attrlist: &attrlist,
            parser: self.0,
        };

        self.0.renderer.render_link(&params, dest);
    }
}
