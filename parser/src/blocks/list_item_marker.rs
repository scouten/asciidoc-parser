#![allow(unused)] // TEMPORARY while building

use std::{slice::Iter, sync::LazyLock};

use regex::Regex;

use crate::{
    HasSpan, Parser, Span,
    attributes::Attrlist,
    blocks::{
        Block, ContentModel, IsBlock, metadata::BlockMetadata, parse_utils::parse_blocks_until,
    },
    internal::debug::DebugSliceReference,
    span::MatchedItem,
    strings::CowStr,
    warnings::Warning,
};

/// A list item is signaled by one of several designeted marker sequences.
#[derive(Clone, Eq, PartialEq)]
pub enum ListItemMarker<'src> {
    /// Unordered list (hyphen).
    Hyphen(Span<'src>),

    /// Unordered list (asterisks).
    Asterisks(Span<'src>),
}

impl<'src> ListItemMarker<'src> {
    pub(crate) fn parse(source: Span<'src>) -> Option<MatchedItem<'src, Self>> {
        let source = source.discard_whitespace();

        let captures = LIST_ITEM_MARKER.captures(source.data())?;
        let marker = source.slice(0..captures[1].len());
        let marker_str = marker.data();
        let after = source.slice_from(captures[1].len()..).discard_whitespace();

        let item = if marker_str == "-" {
            Self::Hyphen(marker)
        } else if marker_str.starts_with('*') {
            Self::Asterisks(marker)
        } else {
            todo!("Not handled yet");
        };

        Some(MatchedItem { item, after })
    }
}

impl<'src> HasSpan<'src> for ListItemMarker<'src> {
    fn span(&self) -> Span<'src> {
        match self {
            Self::Hyphen(x) => *x,
            Self::Asterisks(x) => *x,
        }
    }
}

impl std::fmt::Debug for ListItemMarker<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Hyphen(x) => f.debug_tuple("ListItemMarker::Hyphen").field(x).finish(),

            Self::Asterisks(x) => f.debug_tuple("ListItemMarker::Asterisks").field(x).finish(),
        }
    }
}

static LIST_ITEM_MARKER: LazyLock<Regex> = LazyLock::new(|| {
    #[allow(clippy::unwrap_used)]
    Regex::new(
        r#"(?x)    
            ^(                      # Capture group for list marker
                -                       # Hyphen (unordered list)
                |\*+                    # One or more asterisks (unordered list, up to 5 levels)
                |\.+                    # One or more dots (ordered list, up to 5 levels)
                |\u{2022}               # Bullet character • (unordered list)
                |\d+\.                  # Digits followed by dot (numbered list)
                |[a-zA-Z]\.             # Letter followed by dot (alpha list)
                |[IVXivx]+\)            # Roman numerals followed by ) (Roman list)
            )
            [\ \t]                  # Required whitespace after marker
        "#,
    )
    .unwrap()
});

#[cfg(test)]
mod tests {
    #![allow(clippy::panic)]
    #![allow(clippy::unwrap_used)]

    use pretty_assertions_sorted::assert_eq;

    use crate::{HasSpan, span::MatchedItem, tests::prelude::*};

    fn lim_parse<'a>(
        source: &'a str,
    ) -> Option<MatchedItem<'a, crate::blocks::ListItemMarker<'a>>> {
        crate::blocks::ListItemMarker::parse(crate::Span::new(source))
    }

    #[test]
    fn hyphen() {
        assert!(lim_parse("-").is_none());
        assert!(lim_parse("-- x").is_none());

        let lim = lim_parse("- blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Hyphen(Span {
                data: "-",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "-",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Hyphen(\n    Span {\n        data: \"-\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );
    }

    #[test]
    fn asterisks() {
        assert!(lim_parse("*").is_none());
        assert!(lim_parse("*- x").is_none());

        let lim = lim_parse("* blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Asterisks(Span {
                data: "*",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 3,
                offset: 2,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "*",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Asterisks(\n    Span {\n        data: \"*\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );

        let lim = lim_parse("***** blah").unwrap();

        assert_eq!(
            lim.item,
            ListItemMarker::Asterisks(Span {
                data: "*****",
                line: 1,
                col: 1,
                offset: 0,
            },)
        );

        assert_eq!(
            lim.after,
            Span {
                data: "blah",
                line: 1,
                col: 7,
                offset: 6,
            }
        );

        assert_eq!(
            lim.item.span(),
            Span {
                data: "*****",
                line: 1,
                col: 1,
                offset: 0,
            }
        );

        assert_eq!(
            format!("{lim:#?}", lim = lim.item),
            "ListItemMarker::Asterisks(\n    Span {\n        data: \"*****\",\n        line: 1,\n        col: 1,\n        offset: 0,\n    },\n)"
        );
    }
}
