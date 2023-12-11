mod fn_line {
    use crate::primitives::line;

    #[test]
    fn empty_source() {
        assert_eq!(line(""), Ok(("", "")));
    }

    #[test]
    fn simple_line() {
        assert_eq!(line("abc"), Ok(("", "abc")));
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.
        assert_eq!(line("abc\ndef"), Ok(("def", "abc")));
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.
        assert_eq!(line("abc\r\ndef"), Ok(("def", "abc")));
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.
        assert_eq!(line("abc\n\rdef"), Ok(("\rdef", "abc")));
    }

    #[test]
    fn doesnt_consume_standalone_cr() {
        // Shouldn't terminate line at \r without \n.
        assert_eq!(line("abc\rdef"), Ok(("", "abc\rdef")));
    }
}
