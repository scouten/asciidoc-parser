mod location {
    use nom::character::complete::alpha1;

    use crate::input::{Input, Location};

    #[test]
    fn empty_str() {
        let i = Input::new("", true);
        let l: Location = (&i).into();
        assert_eq!(l.line(), 1);
        assert_eq!(l.col(), 1);
    }

    #[test]
    fn take3() {
        let i1 = Input::new("abc456", true);
        let (i2, _) = alpha1::<Input, crate::Error>(i1).unwrap();
        let l2: Location = (&i2).into();
        assert_eq!(l2.line(), 1);
        assert_eq!(l2.col(), 4);
    }
}

mod span {
    use nom::character::complete::alpha1;

    use crate::input::{Input, Span};

    #[test]
    fn empty_str() {
        let i = Input::new("", true);
        let s = Span::from_start_and_after_end(&i, &i);

        assert_eq!(s.start().line(), 1);
        assert_eq!(s.start().col(), 1);
        assert_eq!(s.after_end().line(), 1);
        assert_eq!(s.after_end().col(), 1);
        assert!(s.is_empty());
    }

    #[test]
    fn take3() {
        let i1 = Input::new("abc456", true);
        let (i2, _) = alpha1::<Input, crate::Error>(i1).unwrap();
        let s = Span::from_start_and_after_end(&i1, &i2);

        assert_eq!(s.start().line(), 1);
        assert_eq!(s.start().col(), 1);
        assert_eq!(s.after_end().line(), 1);
        assert_eq!(s.after_end().col(), 4);
        assert!(!s.is_empty());
    }
}
