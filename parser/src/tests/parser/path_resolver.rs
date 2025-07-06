mod posixify {
    use crate::parser::PathResolver;

    #[test]
    fn replaces_backslashes_if_windowsish() {
        let mut pr = PathResolver::default();
        pr.file_separator = '\\';
        assert_eq!(pr.posixify("abc/def\\ghi"), "abc/def/ghi");
    }

    #[test]
    fn doesnt_replace_backslashes_if_posixish() {
        let mut pr = PathResolver::default();
        pr.file_separator = '/';
        assert_eq!(pr.posixify("abc/def\\ghi"), "abc/def\\ghi");
    }

    #[test]
    fn doesnt_replace_backslashes_if_none_exist() {
        let mut pr = PathResolver::default();
        pr.file_separator = '\\';
        assert_eq!(pr.posixify("abc/def"), "abc/def");
    }
}
