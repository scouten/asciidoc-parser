mod impl_debug {
    use pretty_assertions_sorted::assert_eq;

    use crate::blocks::MediaType;

    #[test]
    fn image() {
        let media_type = MediaType::Image;
        let debug_output = format!("{:?}", media_type);
        assert_eq!(debug_output, "MediaType::Image");
    }

    #[test]
    fn video() {
        let media_type = MediaType::Video;
        let debug_output = format!("{:?}", media_type);
        assert_eq!(debug_output, "MediaType::Video");
    }

    #[test]
    fn audio() {
        let media_type = MediaType::Audio;
        let debug_output = format!("{:?}", media_type);
        assert_eq!(debug_output, "MediaType::Audio");
    }
}
