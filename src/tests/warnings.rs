mod warning {
    use crate::{
        warnings::{Warning, WarningType},
        Span,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let w1 = Warning {
            source: Span::new("abc"),
            warning: WarningType::EmptyAttributeValue,
        };

        let w2 = w1.clone();
        assert_eq!(w1, w2);
    }
}

mod match_and_warnings {
    use crate::{
        warnings::{MatchAndWarnings, Warning, WarningType},
        Span,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let maw1 = MatchAndWarnings {
            item: "xyz",
            warnings: vec![Warning {
                source: Span::new("abc"),
                warning: WarningType::EmptyAttributeValue,
            }],
        };

        let maw2 = maw1.clone();
        assert_eq!(maw1, maw2);
    }

    #[test]
    fn unwrap_if_no_warnings() {
        let maw = MatchAndWarnings {
            item: "xyz",
            warnings: vec![],
        };

        let item = maw.unwrap_if_no_warnings();
        assert_eq!(item, "xyz");
    }

    #[test]
    #[should_panic]
    fn unwrap_if_no_warnings_panic() {
        let maw = MatchAndWarnings {
            item: "xyz",
            warnings: vec![Warning {
                source: Span::new("abc"),
                warning: WarningType::EmptyAttributeValue,
            }],
        };

        let _ = maw.unwrap_if_no_warnings();
        // There are warnings so this should panic.
    }
}
