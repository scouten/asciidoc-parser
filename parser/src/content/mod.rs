use crate::Span;

/// Describes the annotated content of a [`Span`] after any relevant
/// [substitutions] have been performed.
///
/// This is typically used to represent the main body of block types that don't
/// contain other blocks, such as [`SimpleBlock`] or [`RawDelimitedBlock`].
///
/// [substitutions]: https://docs.asciidoctor.org/asciidoc/latest/subs/
/// [`SimpleBlock`]: crate::blocks::SimpleBlock
/// [`RawDelimitedBlock`]: crate::blocks::RawDelimitedBlock
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Content<'src> {
    /// The content of this [`Span`] should be passed through without further
    /// interpretation.
    Passthrough(Span<'src>),

    /// Represents a series of [`Content`] items of varying types.
    Sequence {
        /// The source for the overall sequence of [`Content`] items.
        source: Span<'src>,

        /// The sequence of [`Content`] items.
        children: Vec<Content<'src>>,
    },

    /// The less-than symbol, `<`, is replaced (in HTML output) with the named
    /// character reference `&lt;`.
    NamedCharacterLt(Span<'src>),

    /// The greater-than symbol, `>`, is replaced (in HTML output) with the
    /// named character reference `&gt;`.
    NamedCharacterGt(Span<'src>),

    /// An ampersand, `&`, is replaced with the named character reference
    /// `&amp;`.
    NamedCharacterAmp(Span<'src>),

    /// Content wrapped with underscores (e.g., `_word_`) are wrapped (in HTML
    /// output) in an `<em>` tag.
    Emphasis {
        /// The source for the overall quote sequence.
        source: Span<'src>,

        /// Content to be emphasized.
        content: Box<Content<'src>>,
    },

    /// Content wrapped with asterisks (e.g., `*word*`) are wrapped (in HTML
    /// output) in a `<strong>` tag.
    Strong {
        /// The source for the overall quote sequence.
        source: Span<'src>,

        /// Content to be emphasized.
        content: Box<Content<'src>>,
    },

    /// Content wrapped with backticks (e.g., `` `word` ``) are wrapped (in HTML
    /// output) in a `<code>` tag.
    Monospace {
        /// The source for the overall quote sequence.
        source: Span<'src>,

        /// Content to be monospaced.
        content: Box<Content<'src>>,
    },

    /// Content wrapped with up arrows (e.g., `^word^`) are wrapped (in HTML
    /// output) in a `<sup>` tag.
    Superscript {
        /// The source for the overall quote sequence.
        source: Span<'src>,

        /// Content to be superscripted.
        content: Box<Content<'src>>,
    },

    /// Content wrapped with tildes (e.g., `~word~`) are wrapped (in HTML
    /// output) in a `<sub>` tag.
    Subscript {
        /// The source for the overall quote sequence.
        source: Span<'src>,

        /// Content to be subscripted.
        content: Box<Content<'src>>,
    },

    /// Content wrapped with a double quote and backtick (e.g., ``"`word`"``)
    /// are wrapped (in HTML output) in a double curved quote pair (i.e.
    /// `&#8220;word&#8221;`).
    DoubleCurvedQuotes {
        /// The source for the overall quote sequence.
        source: Span<'src>,

        /// Content to be quote-wrapped.
        content: Box<Content<'src>>,
    },

    /// Content wrapped with a single quote and backtick (e.g., ``'`word`'``)
    /// are wrapped (in HTML output) in single curved quote pair (i.e.
    /// `&#8216;word&#8217;`).
    SingleCurvedQuotes {
        /// The source for the overall quote sequence.
        source: Span<'src>,

        /// Content to be quote-wrapped.
        content: Box<Content<'src>>,
    },

    /// Attribute references are replaced with the values of the attribute they
    /// reference when processed by the `attributes` substitution step. Such
    /// values may be subject to further substitutions.
    AttributeValue {
        /// The source for the attribute substitution sequence.
        source: Span<'src>,

        /// Content of the attribute value after any subsequent substitutions.
        value: Box<Content<'src>>,
    },

    /// The character sequence `(C)` is replaced (in HTML output) with the named
    /// character reference `&#169;`.
    TextSymbolCopyright(Span<'src>),

    /// The character sequence `(R)` is replaced (in HTML output) with the named
    /// character reference `&#174;`.
    TextSymbolRegistered(Span<'src>),

    /// The character sequence `(TM)` is replaced (in HTML output) with the
    /// named character reference `&#8482;`.
    TextSymbolTrademark(Span<'src>),

    /// The character sequence `--` is replaced (in HTML output) with the named
    /// character reference `&#8212;`, but only if between two word characters,
    /// between a word character and a line boundary, or flanked by spaces. When
    /// flanked by space characters (e.g., `a -- b`), the normal spaces are
    /// replaced by thin spaces (`&#8201;`). Otherwise, the em dash is followed
    /// by a zero-width space (`&#8203;`) to provide a break opportunity.
    TextSymbolEmDash(Span<'src>),

    /// The character sequence `...` is replaced (in HTML output) with the named
    /// character reference `&#8430;`. The ellipsis is followed by a zero-width
    /// space (`&#8203;`) to provide a break opportunity.
    TextSymbolEllipsis(Span<'src>),

    /// The character sequence `->` is replaced (in HTML output) with the named
    /// character reference `&#8594;`.
    TextSymbolSingleRightArrow(Span<'src>),

    /// The character sequence `=>` is replaced (in HTML output) with the named
    /// character reference `&#8658;`.
    TextSymbolDoubleRightArrow(Span<'src>),

    /// The character sequence `<-` is replaced (in HTML output) with the named
    /// character reference `&#8592;`.
    TextSymbolSingleLeftArrow(Span<'src>),

    /// The character sequence `<=` is replaced (in HTML output) with the named
    /// character reference `&#8656;`.
    TextSymbolDoubleLeftArrow(Span<'src>),

    /// The single typewriter apostrophe `'` is replaced (in HTML output) with
    /// the named character reference `&#8217;`.
    TextSymbolTypographicApostrophe(Span<'src>),

    /// HTML and XML character references and decimal and hexadecimal Unicode
    /// code points are replaced with their corresponding decimal form Unicode
    /// code point.
    CharacterReference {
        /// The source for the original character reference.
        source: Span<'src>,

        /// The Unicode character to insert.
        value: char,
    },

    /// The content of any inline and block macro are replaced with the
    /// appropriate built-in and user-defined configuration.
    Macro {
        /// The source for the original macro invocation.
        source: Span<'src>,

        /// Replacement content provided by the macro definition.
        content: Box<Content<'src>>,
    },

    /// The line break character, `+`, is replaced with (what?).
    LineBreak(Span<'src>),
}

impl<'src> From<&Span<'src>> for Content<'src> {
    fn from(span: &Span<'src>) -> Self {
        Self::Passthrough(*span)
    }
}
