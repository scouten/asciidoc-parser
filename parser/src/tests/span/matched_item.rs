mod impl_default {
    use crate::span::MatchedItem;

    #[test]
    fn default() {
        let mi: MatchedItem<'_, String> = MatchedItem::default();
        assert_eq!(mi.item, "".to_owned());
    }
}
