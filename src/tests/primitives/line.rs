mod fn_line {
    use crate::{input::Input, primitives::line};

    #[test]
    fn empty_source() {
        let (rem, line) = line(Input::new("", true)).unwrap();

        assert_eq!(rem, Input::new("", true));
        assert_eq!(line, Input::new("", true));
    }

    #[test]
    fn simple_line() {
        let (rem, line) = line(Input::new("abc", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 4);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn trailing_space() {
        let (rem, line) = line(Input::new("abc ", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 5);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc ");
    }

    #[test]
    fn consumes_lf() {
        // Should consume but not return \n.

        let (rem, line) = line(Input::new("abc\ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn consumes_crlf() {
        // Should consume but not return \r\n.

        let (rem, line) = line(Input::new("abc\r\ndef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "def");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn doesnt_consume_lfcr() {
        // Should consume \n but not a subsequent \r.
        
        let (rem, line) = line(Input::new("abc\n\rdef", true)).unwrap();

        assert_eq!(rem.line(), 2);
        assert_eq!(rem.col(), 1);
        assert_eq!(*rem.data(), "\rdef");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc");
    }

    #[test]
    fn standalone_cr_doesnt_end_line() {
        // Shouldn't terminate line at \r without \n.

        let (rem, line) = line(Input::new("abc\rdef", true)).unwrap();

        assert_eq!(rem.line(), 1);
        assert_eq!(rem.col(), 8);
        assert_eq!(*rem.data(), "");

        assert_eq!(line.line(), 1);
        assert_eq!(line.col(), 1);
        assert_eq!(*line.data(), "abc\rdef");
    }
}

// mod normalized_line {
//     use crate::primitives::normalized_line;

//     #[test]
//     fn empty_source() {
//         assert_eq!(normalized_line(""), Ok(("", "")));
//     }

//     #[test]
//     fn simple_line() {
//         assert_eq!(normalized_line("abc"), Ok(("", "abc")));
//     }

//     #[test]
//     fn trailing_space() {
//         assert_eq!(normalized_line("abc "), Ok(("", "abc")));
//     }

//     #[test]
//     fn trailing_spaces() {
//         assert_eq!(normalized_line("abc   "), Ok(("", "abc")));
//     }

//     #[test]
//     fn consumes_lf() {
//         // Should consume but not return \n.
//         assert_eq!(normalized_line("abc  \ndef"), Ok(("def", "abc")));
//     }

//     #[test]
//     fn consumes_crlf() {
//         // Should consume but not return \r\n.
//         assert_eq!(normalized_line("abc\r\ndef"), Ok(("def", "abc")));
//     }

//     #[test]
//     fn doesnt_consume_lfcr() {
//         // Should consume \n but not a subsequent \r.
//         assert_eq!(normalized_line("abc\n\rdef"), Ok(("\rdef", "abc")));
//     }

//     #[test]
//     fn doesnt_consume_standalone_cr() {
//         // Shouldn't terminate normalized_line at \r without \n.
//         assert_eq!(normalized_line("abc\rdef"), Ok(("", "abc\rdef")));
//     }
// }

// mod non_empty_line {
//     use nom::{
//         error::{Error, ErrorKind},
//         Err,
//     };

//     use crate::primitives::non_empty_line;

//     #[test]
//     fn empty_source() {
//         let expected_err: Err<Error<&str>> = Err::Error(Error::new("",
// ErrorKind::TakeTill1));

//         let actual_err = non_empty_line("").unwrap_err();

//         assert_eq!(expected_err, actual_err);
//     }

//     #[test]
//     fn only_spaces() {
//         let expected_err: Err<Error<&str>> = Err::Error(Error::new("   ",
// ErrorKind::TakeTill1));

//         let actual_err = non_empty_line("   ").unwrap_err();

//         assert_eq!(expected_err, actual_err);
//     }

//     #[test]
//     fn simple_line() {
//         assert_eq!(non_empty_line("abc"), Ok(("", "abc")));
//     }

//     #[test]
//     fn trailing_space() {
//         assert_eq!(non_empty_line("abc "), Ok(("", "abc")));
//     }

//     #[test]
//     fn trailing_spaces() {
//         assert_eq!(non_empty_line("abc   "), Ok(("", "abc")));
//     }

//     #[test]
//     fn consumes_lf() {
//         // Should consume but not return \n.
//         assert_eq!(non_empty_line("abc  \ndef"), Ok(("def", "abc")));
//     }

//     #[test]
//     fn consumes_crlf() {
//         // Should consume but not return \r\n.
//         assert_eq!(non_empty_line("abc\r\ndef"), Ok(("def", "abc")));
//     }

//     #[test]
//     fn doesnt_consume_lfcr() {
//         // Should consume \n but not a subsequent \r.
//         assert_eq!(non_empty_line("abc\n\rdef"), Ok(("\rdef", "abc")));
//     }

//     #[test]
//     fn doesnt_consume_standalone_cr() {
//         // Shouldn't terminate non_empty_line at \r without \n.
//         assert_eq!(non_empty_line("abc\rdef"), Ok(("", "abc\rdef")));
//     }
// }
