use crate::parser::PathResolver;

mod posixify {
    use pretty_assertions_sorted::assert_eq;

    use crate::parser::PathResolver;

    #[test]
    fn replaces_backslashes_if_windowsish() {
        let pr = PathResolver {
            file_separator: '\\',
        };

        assert_eq!(pr.posixify("abc/def\\ghi"), "abc/def/ghi");
    }

    #[test]
    fn doesnt_replace_backslashes_if_posixish() {
        let pr = PathResolver {
            file_separator: '/',
        };

        assert_eq!(pr.posixify("abc/def\\ghi"), "abc/def\\ghi");
    }

    #[test]
    fn doesnt_replace_backslashes_if_none_exist() {
        let pr = PathResolver {
            file_separator: '\\',
        };

        assert_eq!(pr.posixify("abc/def"), "abc/def");
    }
}

mod web_path {
    use pretty_assertions_sorted::assert_eq;

    use crate::parser::PathResolver;

    #[test]
    fn test_cases_from_asciidoctor_rb() {
        let pr = PathResolver::default();

        assert_eq!(pr.web_path("images", None), "images");
        assert_eq!(pr.web_path("./images", None), "./images");
        assert_eq!(pr.web_path("/images", None), "/images");

        assert_eq!(
            pr.web_path("./images/../assets/images", None),
            "./assets/images"
        );

        assert_eq!(pr.web_path("/../images", None), "/images");

        assert_eq!(pr.web_path("/../images", Some("assets")), "/images");
        assert_eq!(pr.web_path("../images", Some("./")), "./../images");
        assert_eq!(pr.web_path("../../images", Some("./")), "./../../images");

        assert_eq!(
            pr.web_path("tiger.png", Some("../assets/images")),
            "../assets/images/tiger.png"
        );

        // Basic relative path resolution.
        assert_eq!(
            pr.web_path("images/photo.jpg", Some("docs/guide")),
            "docs/guide/images/photo.jpg"
        );
        assert_eq!(pr.web_path("photo.jpg", Some("images")), "images/photo.jpg");
        assert_eq!(
            pr.web_path("../photo.jpg", Some("images/folder")),
            "images/photo.jpg"
        );
        assert_eq!(
            pr.web_path("../../photo.jpg", Some("docs/images/folder")),
            "docs/photo.jpg"
        );

        // URI-based scenarios (triggers `extract_uri_prefix`).
        assert_eq!(
            pr.web_path("images/photo.jpg", Some("http://example.com/base")),
            "http://example.com/base/images/photo.jpg"
        );
        assert_eq!(
            pr.web_path("../images/logo.png", Some("https://cdn.example.com/assets")),
            "https://cdn.example.com/images/logo.png"
        );
        assert_eq!(
            pr.web_path("docs/guide.pdf", Some("file:///Users/docs")),
            "file:///Users/docs/docs/guide.pdf"
        );
        assert_eq!(
            pr.web_path("assets/style.css", Some("ftp://files.example.com/web")),
            "ftp://files.example.com/web/assets/style.css"
        );

        // Web root scenarios (start parameter ignored).
        assert_eq!(
            pr.web_path("/absolute/path.jpg", Some("http://example.com/base")),
            "/absolute/path.jpg"
        );
        assert_eq!(
            pr.web_path("/images/photo.jpg", Some("docs/guide")),
            "/images/photo.jpg"
        );
        assert_eq!(pr.web_path("/", Some("any/path")), "/");

        // No start path scenarios.
        assert_eq!(pr.web_path("images/photo.jpg", None), "images/photo.jpg");
        assert_eq!(pr.web_path("../photo.jpg", None), "../photo.jpg");

        // Path normalization with dots.
        assert_eq!(
            pr.web_path("./photo.jpg", Some("images")),
            "images/photo.jpg"
        );
        assert_eq!(
            pr.web_path("folder/./photo.jpg", Some("images")),
            "images/folder/photo.jpg"
        );
        assert_eq!(
            pr.web_path("folder/../photo.jpg", Some("images")),
            "images/photo.jpg"
        );

        // Complex path resolution.
        assert_eq!(
            pr.web_path("../../../photo.jpg", Some("docs/images/folder/sub")),
            "docs/photo.jpg"
        );
        assert_eq!(
            pr.web_path("folder/../../photo.jpg", Some("docs/images")),
            "docs/photo.jpg"
        );
        assert_eq!(
            pr.web_path("./folder/../photo.jpg", Some("images")),
            "images/photo.jpg"
        );

        // Edge cases with trailing slashes.
        assert_eq!(
            pr.web_path("photo.jpg", Some("images/")),
            "images/photo.jpg"
        );
        assert_eq!(pr.web_path("photo.jpg", Some("images")), "images/photo.jpg");

        // URLs with paths and parent references.
        assert_eq!(
            pr.web_path("../styles/main.css", Some("https://example.com/assets/css")),
            "https://example.com/assets/styles/main.css"
        );
        assert_eq!(
            pr.web_path(
                "../../images/logo.png",
                Some("http://site.com/docs/guide/examples")
            ),
            "http://site.com/docs/images/logo.png"
        );

        // Space handling (gets URL encoded).
        assert_eq!(
            pr.web_path("my file.jpg", Some("images")),
            "images/my%20file.jpg"
        );
        assert_eq!(
            pr.web_path("folder with spaces/file.jpg", Some("docs")),
            "docs/folder%20with%20spaces/file.jpg"
        );

        // Protocol-less absolute paths.
        assert_eq!(
            pr.web_path(
                "//cdn.example.com/assets/image.jpg",
                Some("http://example.com")
            ),
            "//cdn.example.com/assets/image.jpg"
        );

        // Mixed scenarios.
        assert_eq!(pr.web_path("", Some("docs/images")), "docs/images/");
        assert_eq!(pr.web_path("", Some("")), "/");
        assert_eq!(pr.web_path("", None), "");

        // Complex URI scenarios.
        assert_eq!(
            pr.web_path("api/v1/data", Some("https://api.example.com:8080/base")),
            "https://api.example.com:8080/base/api/v1/data"
        );
        assert_eq!(
            pr.web_path("../v2/data", Some("https://api.example.com/api/v1")),
            "https://api.example.com/api/v2/data"
        );

        // File protocol variations.
        assert_eq!(
            pr.web_path("document.pdf", Some("file:///C:/Users/docs")),
            "file:///C:/Users/docs/document.pdf"
        );
        assert_eq!(
            pr.web_path("../shared/doc.pdf", Some("file:///home/user/documents")),
            "file:///home/user/shared/doc.pdf"
        );
    }
}

#[test]
fn is_web_root() {
    let pr = PathResolver::default();
    assert!(pr.is_web_root("/blah"));
    assert!(!pr.is_web_root(""));
    assert!(!pr.is_web_root("./blah"));
}
