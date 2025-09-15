mod impl_default {
    use crate::span::MatchedItem;

    #[test]
    fn default() {
        let mi: MatchedItem<'_, String> = MatchedItem::default();
        assert_eq!(mi.item, "".to_owned());
    }
}

mod unwrap_item_or_default {
    use crate::{Span, span::MatchedItem};

    #[test]
    fn some() {
        let mi = MatchedItem {
            item: Some("foo".to_string()),
            after: Span::default(),
        };

        assert_eq!(mi.unwrap_item_or_default(), "foo".to_owned());
    }

    #[test]
    fn none() {
        let mi: MatchedItem<'_, Option<String>> = MatchedItem {
            item: None,
            after: Span::default(),
        };

        assert_eq!(mi.unwrap_item_or_default(), "".to_owned());
    }
}
