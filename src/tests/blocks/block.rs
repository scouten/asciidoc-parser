mod simple {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };

    use crate::{
        blocks::{Block, SimpleBlock},
        tests::fixtures::{
            blocks::{TBlock, TSimpleBlock},
            TSpan,
        },
        Span,
    };

    #[test]
    fn empty_source() {
        let expected_err = Err::Error(Error::new(Span::new("", true), ErrorKind::TakeTill1));

        let actual_err = Block::parse(Span::new("", true)).unwrap_err();

        assert_eq!(expected_err, actual_err);
    }

    #[test]
    fn only_spaces() {
        let err = Block::parse(Span::new("    ", true)).unwrap_err();

        let Err::Error(e) = err else {
            panic!("Expected Err::Error: {err:#?}");
        };

        assert_eq!(e.code, ErrorKind::TakeTill1);

        assert_eq!(
            e.input,
            TSpan {
                data: "",
                line: 1,
                col: 5,
                offset: 4
            }
        );
    }

    #[test]
    fn single_line() {
        let expected = Block::Simple(SimpleBlock {
            inlines: vec![Span::new("abc", true)],
        });

        let (rem, block) = Block::parse(Span::new("abc", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 1,
                col: 4,
                offset: 3
            }
        );

        assert_eq!(
            block,
            TBlock::Simple(TSimpleBlock {
                inlines: vec![TSpan {
                    data: "abc",
                    line: 1,
                    col: 1,
                    offset: 0,
                }]
            })
        );

        assert_eq!(block, expected);
    }

    #[test]
    fn multiple_lines() {
        let (rem, block) = Block::parse(Span::new("abc\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "",
                line: 2,
                col: 4,
                offset: 7
            }
        );

        let Block::Simple(block) = block;
        // else { // ADD THIS ONCE WE HAVE OTHER BLOCK TYPES
        //panic!("Expected a SimpleBlock: {block:#?}");
        //};

        assert_eq!(block.inlines.len(), 2);

        assert_eq!(
            block.inlines[0],
            TSpan {
                data: "abc",
                line: 1,
                col: 1,
                offset: 0
            }
        );

        assert_eq!(
            block.inlines[1],
            TSpan {
                data: "def",
                line: 2,
                col: 1,
                offset: 4
            }
        );
    }

    #[test]
    fn consumes_blank_lines_after() {
        let expected = SimpleBlock {
            inlines: vec![Span::new("abc", true)],
        };

        let (rem, block) = Block::parse(Span::new("abc\n\ndef", true)).unwrap();

        assert_eq!(
            rem,
            TSpan {
                data: "def",
                line: 3,
                col: 1,
                offset: 5
            }
        );

        let Block::Simple(block) = block;
        // else { // ADD THIS ONCE WE HAVE OTHER BLOCK TYPES
        //panic!("Expected a SimpleBlock: {block:#?}");
        //};

        assert_eq!(block, expected);
    }
}
