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

mod match_and_maybe_warning {
    use crate::{
        warnings::{MatchAndMaybeWarning, Warning, WarningType},
        Span,
    };

    #[test]
    fn impl_clone() {
        // Silly test to mark the #[derive(...)] line as covered.
        let mmw1 = MatchAndMaybeWarning {
            item: "xyz",
            warning: Some(Warning {
                source: Span::new("abc"),
                warning: WarningType::EmptyAttributeValue,
            }),
        };

        let mmw2 = mmw1.clone();
        assert_eq!(mmw1, mmw2);
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
}
