use std::fmt::Debug;

use crate::{attributes::Attrlist, Parser};

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
                .map(|title| format!(r#" title="{}""#, *title.raw_value()))
                .unwrap_or_default()
        );

        let format = params
            .attrlist
            .named_attribute("format")
            .map(|format| *format.raw_value());

        // TO DO: Enforce non-safe mode. Add this contraint to following `if` clause:
        // `&& node.document.safe < SafeMode::SECURE`

        let img = if format == Some("svg") || params.target.contains(".svg") {
            todo!(
                "Port this: {}",
                r##"
                    if node.option? 'inline'
                        img = (read_svg_contents node, target) || %(<span class="alt">#{node.alt}</span>)
                    elsif node.option? 'interactive'
                        fallback = (node.attr? 'fallback') ? %(<img src="#{node.image_uri node.attr 'fallback'}" alt="#{encode_attribute_value node.alt}"#{attrs}#{@void_element_slash}>) : %(<span class="alt">#{node.alt}</span>)
                        img = %(<object type="image/svg+xml" data="#{src = node.image_uri target}"#{attrs}>#{fallback}</object>)
                    else
                        img = %(<img src="#{src = node.image_uri target}" alt="#{encode_attribute_value node.alt}"#{attrs}#{@void_element_slash}>)
                    end
            "##
            );
        } else {
            format!(
                r#"<img src="{src}" alt="{alt}"{attrs}{void_element_slash}>"#,
                src = params.target,
                alt = params.alt,
                attrs = attrs,
                void_element_slash = "",
                // img = %(<img src="#{src = node.image_uri target}" alt="#{encode_attribute_value
                // node.alt}"#{attrs}#{@void_element_slash}>)
            )
        };

        render_icon_or_image(params, &img, "image", dest);
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
