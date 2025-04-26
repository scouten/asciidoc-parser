use std::cmp::PartialEq;

use crate::{tests::fixtures::TSpan, Content};

#[allow(unused)] // TEMPORARY while building
#[derive(Debug, Eq, PartialEq)]
pub(crate) enum TContent {
    Basic(TSpan),

    Sequence {
        source: TSpan,
        children: Vec<TContent>,
    },

    NamedCharacterLt(TSpan),
    NamedCharacterGt(TSpan),
    NamedCharacterAmp(TSpan),

    Emphasis {
        source: TSpan,
        content: Box<TContent>,
    },

    Strong {
        source: TSpan,
        content: Box<TContent>,
    },

    Monospace {
        source: TSpan,
        content: Box<TContent>,
    },

    Superscript {
        source: TSpan,
        content: Box<TContent>,
    },

    Subscript {
        source: TSpan,
        content: Box<TContent>,
    },

    DoubleCurvedQuotes {
        source: TSpan,
        content: Box<TContent>,
    },

    SingleCurvedQuotes {
        source: TSpan,
        content: Box<TContent>,
    },

    AttributeValue {
        source: TSpan,
        value: Box<TContent>,
    },

    TextSymbolCopyright(TSpan),
    TextSymbolRegistered(TSpan),
    TextSymbolTrademark(TSpan),
    TextSymbolEmDash(TSpan),
    TextSymbolEllipsis(TSpan),
    TextSymbolSingleRightArrow(TSpan),
    TextSymbolDoubleRightArrow(TSpan),
    TextSymbolSingleLeftArrow(TSpan),
    TextSymbolDoubleLeftArrow(TSpan),
    TextSymbolTypographicApostrophe(TSpan),

    CharacterReference {
        source: TSpan,
        value: char,
    },

    Macro {
        source: TSpan,
        content: Box<TContent>,
    },

    LineBreak(TSpan),
}

impl<'src> PartialEq<Content<'src>> for TContent {
    fn eq(&self, other: &Content<'src>) -> bool {
        fixture_eq_observed(self, other)
    }
}

impl PartialEq<TContent> for Content<'_> {
    fn eq(&self, other: &TContent) -> bool {
        fixture_eq_observed(other, self)
    }
}

impl PartialEq<TContent> for &Content<'_> {
    fn eq(&self, other: &TContent) -> bool {
        fixture_eq_observed(other, self)
    }
}

fn fixture_eq_observed(fixture: &TContent, observed: &Content) -> bool {
    match fixture {
        TContent::Basic(f_span) => match observed {
            Content::Basic(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::Sequence {
            source: f_source,
            children: f_children,
        } => match observed {
            Content::Sequence {
                source: o_source,
                children: ref o_children,
            } => f_source == o_source && f_children == o_children,
            _ => false,
        },

        TContent::NamedCharacterLt(f_span) => match observed {
            Content::NamedCharacterLt(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::NamedCharacterGt(f_span) => match observed {
            Content::NamedCharacterGt(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::NamedCharacterAmp(f_span) => match observed {
            Content::NamedCharacterAmp(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::Emphasis {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::Emphasis {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::Strong {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::Strong {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::Monospace {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::Monospace {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::Superscript {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::Superscript {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::Subscript {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::Subscript {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::DoubleCurvedQuotes {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::DoubleCurvedQuotes {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::SingleCurvedQuotes {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::SingleCurvedQuotes {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::AttributeValue {
            source: f_source,
            value: f_value,
        } => match observed {
            Content::AttributeValue {
                source: o_source,
                value: o_value,
            } => f_source == o_source && f_value.as_ref() == o_value.as_ref(),
            _ => false,
        },

        TContent::TextSymbolCopyright(f_span) => match observed {
            Content::TextSymbolCopyright(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolRegistered(f_span) => match observed {
            Content::TextSymbolRegistered(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolTrademark(f_span) => match observed {
            Content::TextSymbolTrademark(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolEmDash(f_span) => match observed {
            Content::TextSymbolEmDash(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolEllipsis(f_span) => match observed {
            Content::TextSymbolEllipsis(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolSingleRightArrow(f_span) => match observed {
            Content::TextSymbolSingleRightArrow(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolDoubleRightArrow(f_span) => match observed {
            Content::TextSymbolDoubleRightArrow(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolSingleLeftArrow(f_span) => match observed {
            Content::TextSymbolSingleLeftArrow(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolDoubleLeftArrow(f_span) => match observed {
            Content::TextSymbolDoubleLeftArrow(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::TextSymbolTypographicApostrophe(f_span) => match observed {
            Content::TextSymbolTypographicApostrophe(ref o_span) => f_span == o_span,
            _ => false,
        },

        TContent::CharacterReference {
            source: f_source,
            value: f_value,
        } => match observed {
            Content::CharacterReference {
                source: o_source,
                value: o_value,
            } => f_source == o_source && f_value == o_value,
            _ => false,
        },

        TContent::Macro {
            source: f_source,
            content: f_content,
        } => match observed {
            Content::Macro {
                source: o_source,
                content: o_content,
            } => f_source == o_source && f_content.as_ref() == o_content.as_ref(),
            _ => false,
        },

        TContent::LineBreak(f_span) => match observed {
            Content::LineBreak(ref o_span) => f_span == o_span,
            _ => false,
        },
    }
}
