use crate::parser::PathResolver;

mod posixify {
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

        // #     resolver.web_path('./images/../assets/images')
        // #     => './assets/images'
        // #
        // #     resolver.web_path('/../images')
        // #     => '/images'
        // #
        // #     resolver.web_path('images', 'assets')
        // #     => 'assets/images'
        // #
        // #     resolver.web_path('tiger.png', '../assets/images')
        // #     => '../assets/images/tiger.png'
    }
}

#[test]
fn is_web_root() {
    let pr = PathResolver::default();
    assert!(pr.is_web_root("/blah"));
    assert!(!pr.is_web_root(""));
    assert!(!pr.is_web_root("./blah"));
}
