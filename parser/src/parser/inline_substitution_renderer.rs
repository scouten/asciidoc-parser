use std::{fmt::Debug, sync::LazyLock};

use regex::Regex;

use crate::{Parser, attributes::Attrlist};

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

    /// Renders the content of a [quote substitution].
    ///
    /// The renderer should write the appropriate rendering to `dest`.
    ///
    /// [quote substitution]: https://docs.asciidoctor.org/asciidoc/latest/subs/quotes/
    fn render_quoted_substitition(
        &self,
        type_: QuoteType,
        scope: QuoteScope,
        attrlist: Option<Attrlist<'_>>,
        id: Option<String>,
        body: &str,
        dest: &mut String,
    );

    /// Renders the content of a [character replacement].
    ///
    /// The renderer should write the appropriate rendering to `dest`.
    ///
    /// [character replacement]: https://docs.asciidoctor.org/asciidoc/latest/subs/replacements/
    fn render_character_replacement(&self, type_: CharacterReplacementType, dest: &mut String);

    /// Renders a line break.
    ///
    /// The renderer should write an appropriate rendering of line break to
    /// `dest`.
    ///
    /// This is used in the implementation of [post-replacement substitutions].
    ///
    /// [post-replacement substitutions]: https://docs.asciidoctor.org/asciidoc/latest/subs/post-replacements/
    fn render_line_break(&self, dest: &mut String);

    /// Renders an image.
    ///
    /// The renderer should write an appropriate rendering of the specified
    /// image to `dest`.
    fn render_image(&self, params: &ImageRenderParams, dest: &mut String);

    /// Construct a URI reference or data URI to the target image.
    ///
    /// If the `target_image_path` is a URI reference, then leave it untouched.
    ///
    /// The `target_image_path` is resolved relative to the directory retrieved
    /// from the specified document-scoped attribute key, if provided.
    ///
    /// NOT YET IMPLEMENTED:
    /// If the `data-uri` attribute is set on the document, and the safe mode
    /// level is less than `SafeMode::SECURE`, the image will be safely
    /// converted to a data URI by reading it from the same directory. If
    /// neither of these conditions are satisfied, a relative path (i.e., URL)
    /// will be returned.
    ///
    /// ## Parameters
    ///
    /// * `target_image_path`: path to the target image
    /// * `parser`: Current document parser state
    /// * `asset_dir_key`: If provided, the attribute key used to look up the
    ///   directory where the image is located. If not provided, `imagesdir` is
    ///   used.
    ///
    /// ## Return
    ///
    /// Returns a string reference or data URI for the target image that can be
    /// safely used in an image tag.
    fn image_uri(
        &self,
        target_image_path: &str,
        parser: &Parser,
        asset_dir_key: Option<&str>,
    ) -> String;
}

/// Specifies which special character is being replaced in a call to
/// [`InlineSubstitutionRenderer::render_special_character`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpecialCharacter {
    /// Replace `<` character.
    Lt,

    /// Replace `>` character.
    Gt,

    /// Replace `&` character.
    Ampersand,
}

/// Specifies which [quote type] is being rendered.
///
/// [quote type]: https://docs.asciidoctor.org/asciidoc/latest/subs/quotes/
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuoteType {
    /// Strong (often bold) formatting.
    Strong,

    /// Word(s) surrounded by smart double quotes.
    DoubleQuote,

    /// Word(s) surrounded by smart single quotes.
    SingleQuote,

    /// Monospace (code) formatting.
    Monospaced,

    /// Emphasis (often italic) formatting.
    Emphasis,

    /// Text range (span) formatted with zero or more styles.
    Mark,

    /// Superscript formatting.
    Superscript,

    /// Subscript formatting.
    Subscript,

    /// Surrounds a block of text that may need a `<span>` or similar tag.
    Unquoted,
}

/// Specifies whether the block is aligned to word boundaries or not.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuoteScope {
    /// The quoted section was aligned to word boundaries.
    Constrained,

    /// The quoted section may not have been aligned to word boundaries.
    Unconstrained,
}

/// Specifies which [character replacement] is being rendered.
///
/// [character replacement]: https://docs.asciidoctor.org/asciidoc/latest/subs/replacements/
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CharacterReplacementType {
    /// Copyright `(C)`.
    Copyright,

    /// Registered `(R)`.
    Registered,

    /// Trademark `(TM)`.
    Trademark,

    /// Em-dash surrounded by spaces ` -- `.
    EmDashSurroundedBySpaces,

    /// Em-dash without space `--`.
    EmDashWithoutSpace,

    /// Ellipsis `...`.
    Ellipsis,

    /// Single right arrow `->`.
    SingleRightArrow,

    /// Double right arrow `=>`.
    DoubleRightArrow,

    /// Single left arrow `<-`.
    SingleLeftArrow,

    /// Double left arrow `<=`.
    DoubleLeftArrow,

    /// Typographic apostrophe `'` within a word.
    TypographicApostrophe,

    /// Character reference `&___;`.
    CharacterReference(String),
}

/// Provides parsed parameters for an image to be rendered.
#[derive(Clone, Debug)]
pub struct ImageRenderParams<'a> {
    /// Target (the reference to the image).
    pub target: &'a str,

    /// Alt text (either explicitly set or defaulted).
    pub alt: String,

    /// Width. The data type is not checked; this may be any string.
    pub width: Option<&'a str>,

    /// Height. The data type is not checked; this may be any string.
    pub height: Option<&'a str>,

    /// Attribute list.
    pub attrlist: &'a Attrlist<'a>,

    /// Parser. The rendered may find document settings (such as an image
    /// directory) in the parser's document attributes.
    pub parser: &'a Parser<'a>,
}

/// Implementation of [`InlineSubstitutionRenderer`] that renders substitutions
/// for common HTML-based applications.
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
        mut id: Option<String>,
        body: &str,
        dest: &mut String,
    ) {
        let mut roles: Vec<&str> = attrlist.as_ref().map(|a| a.roles()).unwrap_or_default();

        if let Some(block_style) = attrlist
            .as_ref()
            .and_then(|a| a.nth_attribute(1))
            .and_then(|attr1| attr1.block_style())
        {
            roles.insert(0, block_style);
        }

        if id.is_none() {
            id = attrlist
                .as_ref()
                .and_then(|a| a.nth_attribute(1))
                .and_then(|attr1| attr1.id())
                .map(|id| id.to_owned())
        }

        match type_ {
            QuoteType::Strong => {
                wrap_body_in_html_tag(attrlist.as_ref(), "strong", id, roles, body, dest);
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
                wrap_body_in_html_tag(attrlist.as_ref(), "code", id, roles, body, dest);
            }

            QuoteType::Emphasis => {
                wrap_body_in_html_tag(attrlist.as_ref(), "em", id, roles, body, dest);
            }

            QuoteType::Mark => {
                if roles.is_empty() && id.is_none() {
                    wrap_body_in_html_tag(attrlist.as_ref(), "mark", id, roles, body, dest);
                } else {
                    wrap_body_in_html_tag(attrlist.as_ref(), "span", id, roles, body, dest);
                }
            }

            QuoteType::Superscript => {
                wrap_body_in_html_tag(attrlist.as_ref(), "sup", id, roles, body, dest);
            }

            QuoteType::Subscript => {
                wrap_body_in_html_tag(attrlist.as_ref(), "sub", id, roles, body, dest);
            }

            QuoteType::Unquoted => {
                if roles.is_empty() && id.is_none() {
                    dest.push_str(body);
                } else {
                    wrap_body_in_html_tag(attrlist.as_ref(), "span", id, roles, body, dest);
                }
            }
        }
    }

    fn render_character_replacement(&self, type_: CharacterReplacementType, dest: &mut String) {
        match type_ {
            CharacterReplacementType::Copyright => {
                dest.push_str("&#169;");
            }

            CharacterReplacementType::Registered => {
                dest.push_str("&#174;");
            }

            CharacterReplacementType::Trademark => {
                dest.push_str("&#8482;");
            }

            CharacterReplacementType::EmDashSurroundedBySpaces => {
                dest.push_str("&#8201;&#8212;&#8201;");
            }

            CharacterReplacementType::EmDashWithoutSpace => {
                dest.push_str("&#8212;&#8203;");
            }

            CharacterReplacementType::Ellipsis => {
                dest.push_str("&#8230;&#8203;");
            }

            CharacterReplacementType::SingleLeftArrow => {
                dest.push_str("&#8592;");
            }

            CharacterReplacementType::DoubleLeftArrow => {
                dest.push_str("&#8656;");
            }

            CharacterReplacementType::SingleRightArrow => {
                dest.push_str("&#8594;");
            }

            CharacterReplacementType::DoubleRightArrow => {
                dest.push_str("&#8658;");
            }

            CharacterReplacementType::TypographicApostrophe => {
                dest.push_str("&#8217;");
            }

            CharacterReplacementType::CharacterReference(name) => {
                dest.push('&');
                dest.push_str(&name);
                dest.push(';');
            }
        }
    }

    fn render_line_break(&self, dest: &mut String) {
        dest.push_str("<br>");
    }

    fn render_image(&self, params: &ImageRenderParams, dest: &mut String) {
        let attrs = format!(
            "{width}{height}{title}",
            width = params
                .width
                .map(|width| format!(r#" width="{width}""#))
                .unwrap_or_default(),
            height = params
                .height
                .map(|height| format!(r#" height="{height}""#))
                .unwrap_or_default(),
            title = params
                .attrlist
                .named_attribute("title")
                .map(|title| format!(r#" title="{}""#, title.value()))
                .map(encode_attribute_value)
                .unwrap_or_default()
        );

        let format = params
            .attrlist
            .named_attribute("format")
            .map(|format| format.value());

        // TO DO: Enforce non-safe mode. Add this contraint to following `if` clause:
        // `&& node.document.safe < SafeMode::SECURE`

        let (mut img, src) = if format == Some("svg") || params.target.contains(".svg") {
            if params.attrlist.has_option("inline") {
                todo!(
                    "Port this: {}",
                    r#"img = (read_svg_contents node, target) || %(<span class="alt">#{node.alt}</span>)"#
                );
            } else if params.attrlist.has_option("interactive") {
                todo!(
                    "Port this: {}",
                    r##"
                        fallback = (node.attr? 'fallback') ? %(<img src="#{node.image_uri node.attr 'fallback'}" alt="#{encode_attribute_value node.alt}"#{attrs}#{@void_element_slash}>) : %(<span class="alt">#{node.alt}</span>)
                        img = %(<object type="image/svg+xml" data="#{src = node.image_uri target}"#{attrs}>#{fallback}</object>)
                    "##
                );
            } else {
                let src = self.image_uri(params.target, params.parser, None);

                (
                    format!(
                        r#"<img src="{src}" alt="{alt}"{attrs}{void_element_slash}>"#,
                        alt = encode_attribute_value(params.alt.to_string()),
                        attrs = attrs,
                        void_element_slash = "",
                        // img = %(<img src="#{src = node.image_uri target}"
                        // alt="#{encode_attribute_value node.alt}"#{attrs}#{@
                        // void_element_slash}>)
                    ),
                    src,
                )
            }
        } else {
            let src = self.image_uri(params.target, params.parser, None);

            (
                format!(
                    r#"<img src="{src}" alt="{alt}"{attrs}{void_element_slash}>"#,
                    alt = encode_attribute_value(params.alt.to_string()),
                    attrs = attrs,
                    void_element_slash = "",
                    // img = %(<img src="#{src = node.image_uri target}"
                    // alt="#{encode_attribute_value node.alt}"#{attrs}#{@
                    // void_element_slash}>)
                ),
                src,
            )
        };

        let link = params.attrlist.named_attribute("link").map(|link| {
            if link.value() == "self" {
                src
            } else {
                link.value().to_string()
            }
        });

        if let Some(link) = link {
            img = format!(
                r#"<a class="image" href="{link}"{link_constraint_attrs}>{img}</a>"#,
                link_constraint_attrs = "" /* link_constraint_attrs =
                                            * (append_link_constraint_attrs node).join} */
            );
        }

        render_icon_or_image(params, &img, "image", dest);
    }

    fn image_uri(
        &self,
        target_image_path: &str,
        parser: &Parser,
        asset_dir_key: Option<&str>,
    ) -> String {
        let asset_dir_key = asset_dir_key.unwrap_or("imagesdir");

        if false {
            todo!(
                "Port this: {}",
                r#"
				if (doc = @document).safe < SafeMode::SECURE && (doc.attr? 'data-uri')
				  if ((Helpers.uriish? target_image) && (target_image = Helpers.encode_spaces_in_uri target_image)) ||
					  (asset_dir_key && (images_base = doc.attr asset_dir_key) && (Helpers.uriish? images_base) &&
					  (target_image = normalize_web_path target_image, images_base, false))
					(doc.attr? 'allow-uri-read') ? (generate_data_uri_from_uri target_image, (doc.attr? 'cache-uri')) : target_image
				  else
					generate_data_uri target_image, asset_dir_key
				  end
				else
				  normalize_web_path target_image, (asset_dir_key ? (doc.attr asset_dir_key) : nil)
				end
            "#
            );
        } else {
            dbg!(&asset_dir_key);

            let asset_dir = parser
                .attribute_value(asset_dir_key)
                .as_maybe_str()
                .map(|s| s.to_string());

            normalize_web_path(target_image_path, parser, asset_dir.as_deref(), false)
        }
    }
}

fn wrap_body_in_html_tag(
    _attrlist: Option<&Attrlist<'_>>,
    tag: &'static str,
    id: Option<String>,
    roles: Vec<&str>,
    body: &str,
    dest: &mut String,
) {
    dest.push('<');
    dest.push_str(tag);

    if let Some(id) = id.as_ref() {
        dest.push_str(" id=\"");
        dest.push_str(id);
        dest.push('"');
    }

    if !roles.is_empty() {
        let roles = roles.join(" ");
        dest.push_str(" class=\"");
        dest.push_str(&roles);
        dest.push('"');
    }

    dest.push('>');
    dest.push_str(body);
    dest.push_str("</");
    dest.push_str(tag);
    dest.push('>');
}

fn render_icon_or_image(
    _params: &ImageRenderParams,
    img: &str,
    type_: &'static str,
    dest: &mut String,
) {
    let class_attr_val = type_;

    if false {
        // Handle the edge cases within.
        todo!(
            "Port this: {}",
            r##"
            if (node.attr? 'link') && ((href_attr_val = node.attr 'link') != 'self' || (href_attr_val = src))
                img = %(<a class="image" href="#{href_attr_val}"#{(append_link_constraint_attrs node).join}>#{img}</a>)
            end
            if (role = node.role)
                class_attr_val = (node.attr? 'float') ? %(#{class_attr_val} #{node.attr 'float'} #{role}) : %(#{class_attr_val} #{role})
            elsif node.attr? 'float'
                class_attr_val = %(#{class_attr_val} #{node.attr 'float'})
            end
        "##
        );
    }

    dest.push_str(r#"<span class=""#);
    dest.push_str(class_attr_val);
    dest.push_str(r#"">"#);
    dest.push_str(img);
    dest.push_str("</span>");
}

fn encode_attribute_value(value: String) -> String {
    value.replace('"', "&quot;")
}

fn normalize_web_path(
    target: &str,
    parser: &Parser,
    start: Option<&str>,
    preserve_uri_target: bool,
) -> String {
    if preserve_uri_target && is_uri_ish(target) {
        todo!("Helpers.encode_spaces_in_uri target");
    } else {
        parser.path_resolver.web_path(target, start)
    }
}

fn is_uri_ish(path: &str) -> bool {
    path.contains(':') && URI_SNIFF.is_match(path)
}

/// Detects strings that resemble URIs.
///
/// ## Examples
///
/// * `http://domain`
/// * `https://domain`
/// * `file:///path`
/// * `data:info`
///
/// ## Counter-examples (do not match)
///
/// * `c:/sample.adoc`
/// * `c:\sample.adoc`
static URI_SNIFF: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)
        \A                             # Anchor to start of string
        \p{Alphabetic}                 # First character must be a letter
        [\p{Alphabetic}\p{Nd}.+-]+     # Followed by one or more alphanum or . + -
        :                              # Literal colon
        /{0,2}                         # Zero to two slashes
    "#,
    )
    .unwrap()
});
