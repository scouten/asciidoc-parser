mod impl_debug {
    use pretty_assertions_sorted::assert_eq;

    use crate::blocks::ContentModel;

    #[test]
    fn compound() {
        let content_model = ContentModel::Compound;
        let debug_output = format!("{:?}", content_model);
        assert_eq!(debug_output, "ContentModel::Compound");
    }

    #[test]
    fn simple() {
        let content_model = ContentModel::Simple;
        let debug_output = format!("{:?}", content_model);
        assert_eq!(debug_output, "ContentModel::Simple");
    }

    #[test]
    fn verbatim() {
        let content_model = ContentModel::Verbatim;
        let debug_output = format!("{:?}", content_model);
        assert_eq!(debug_output, "ContentModel::Verbatim");
    }

    #[test]
    fn raw() {
        let content_model = ContentModel::Raw;
        let debug_output = format!("{:?}", content_model);
        assert_eq!(debug_output, "ContentModel::Raw");
    }

    #[test]
    fn empty() {
        let content_model = ContentModel::Empty;
        let debug_output = format!("{:?}", content_model);
        assert_eq!(debug_output, "ContentModel::Empty");
    }

    #[test]
    fn table() {
        let content_model = ContentModel::Table;
        let debug_output = format!("{:?}", content_model);
        assert_eq!(debug_output, "ContentModel::Table");
    }
}
