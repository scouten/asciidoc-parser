use std::{borrow::Cow, path::Path, sync::LazyLock};

use regex::{Captures, Regex, Replacer};

use crate::{attributes::Attrlist, parser::ImageRenderParams, Span};
#[allow(unused)] // TEMPORARY while building
use crate::{Content, Parser};

pub(super) fn apply_macros(content: &mut Content<'_>, parser: &'_ Parser) {
    let text = content.rendered().to_string();
    let found_square_bracket = text.contains('[');
    let found_colon = text.contains(':');
    let found_macroish = found_square_bracket && found_colon;
    let found_macroish_short = found_macroish && text.contains(":[");

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

    if (text.contains("((") && text.contains("))"))
        || (found_macroish_short && text.contains("dexterm"))
    {
        todo!("Index term macro");
        // Port Ruby Asciidoctor's implementation from lines 439..536.
    }

    if found_colon && text.contains("://") {
        todo!("URL macro");
        // Port Ruby Asciidoctor's implementation from lines 538..634.
    }

    if found_macroish && (text.contains("link:") || text.contains("ilto:")) {
        todo!("Link macro");
        // Port Ruby Asciidoctor's implementation from lines 636..704.
    }

    if text.contains('@') {
        todo!("Maybe found email macro");
        // Port Ruby Asciidoctor's implementation from lines 706..717.
    }

    if found_square_bracket && false {
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
        let attrlist = Attrlist::parse(span).item.item;

        let default_alt = basename(&target.replace('_', " ").replace('-', " "));
        // IMPORTANT: Implementations of `render_icon` and `render_image` need to
        // remember to use `default_alt` when attrlist doesn't contain a value for
        // `alt`.

        if caps[0].starts_with("image:") {
            // TO DO: Register image with parser?
            // IMPORTANT: May require interior mutability on Parser because it looks like we
            // can't pass mutable references to Parser in a recursive Regex replacement.
            if false {
                todo!("Port this: {}", "doc.register :images, target");
            }

            let params = ImageRenderParams {
                target,
                alt: attrlist
                    .named_or_positional_attribute("alt", 1)
                    .map_or(default_alt, |a| a.raw_value().to_string()),
                width: attrlist
                    .named_or_positional_attribute("width", 2)
                    .map(|a| a.raw_value().data()),
                height: attrlist
                    .named_or_positional_attribute("height", 3)
                    .map(|a| a.raw_value().data()),
                attrlist: &attrlist,
                parser: self.0,
            };

            self.0.renderer.render_image(&params, dest);
        } else {
            todo!(
                "Port this: {}",
                r#"
        type, posattrs = 'image', %w(alt width height)
        target = $1
        attrs = parse_attributes $2, posattrs, unescape_input: true
        attrs['alt'] ||= (attrs['default-alt'] = (Helpers.basename target, true).tr '_-', ' ')
        Inline.new(self, :image, nil, type: type, target: target, attributes: attrs).convert
        "#
            );
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
