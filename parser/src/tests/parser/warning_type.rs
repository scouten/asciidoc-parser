mod impl_debug {
    use pretty_assertions_sorted::assert_eq;

    use crate::warnings::WarningType;

    #[test]
    fn attribute_value_missing_terminating_quote() {
        let warning = WarningType::AttributeValueMissingTerminatingQuote;
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::AttributeValueMissingTerminatingQuote"
        );
    }

    #[test]
    fn document_header_not_terminated() {
        let warning = WarningType::DocumentHeaderNotTerminated;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::DocumentHeaderNotTerminated");
    }

    #[test]
    fn empty_attribute_value() {
        let warning = WarningType::EmptyAttributeValue;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::EmptyAttributeValue");
    }

    #[test]
    fn empty_shorthand_item() {
        let warning = WarningType::EmptyShorthandItem;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::EmptyShorthandItem");
    }

    #[test]
    fn invalid_macro_name() {
        let warning = WarningType::InvalidMacroName;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::InvalidMacroName");
    }

    #[test]
    fn media_macro_missing_target() {
        let warning = WarningType::MediaMacroMissingTarget;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::MediaMacroMissingTarget");
    }

    #[test]
    fn macro_missing_attribute_list() {
        let warning = WarningType::MacroMissingAttributeList;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::MacroMissingAttributeList");
    }

    #[test]
    fn macro_missing_double_colon() {
        let warning = WarningType::MacroMissingDoubleColon;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::MacroMissingDoubleColon");
    }

    #[test]
    fn missing_comma_after_quoted_attribute_value() {
        let warning = WarningType::MissingCommaAfterQuotedAttributeValue;
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::MissingCommaAfterQuotedAttributeValue"
        );
    }

    #[test]
    fn unterminated_delimited_block() {
        let warning = WarningType::UnterminatedDelimitedBlock;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::UnterminatedDelimitedBlock");
    }

    #[test]
    fn missing_block_after_title_or_attribute_list() {
        let warning = WarningType::MissingBlockAfterTitleOrAttributeList;
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::MissingBlockAfterTitleOrAttributeList"
        );
    }

    #[test]
    fn empty_block_anchor_name() {
        let warning = WarningType::EmptyBlockAnchorName;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::EmptyBlockAnchorName");
    }

    #[test]
    fn invalid_block_anchor_name() {
        let warning = WarningType::InvalidBlockAnchorName;
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::InvalidBlockAnchorName");
    }

    #[test]
    fn attribute_value_is_locked_simple_string() {
        let warning = WarningType::AttributeValueIsLocked("test-attribute".to_string());
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::AttributeValueIsLocked(\"test-attribute\")"
        );
    }

    #[test]
    fn attribute_value_is_locked_empty_string() {
        let warning = WarningType::AttributeValueIsLocked("".to_string());
        let debug_output = format!("{:?}", warning);
        assert_eq!(debug_output, "WarningType::AttributeValueIsLocked(\"\")");
    }

    #[test]
    fn attribute_value_is_locked_string_with_special_chars() {
        let warning =
            WarningType::AttributeValueIsLocked("attr-with-special!@#$%^&*()".to_string());
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::AttributeValueIsLocked(\"attr-with-special!@#$%^&*()\")"
        );
    }

    #[test]
    fn attribute_value_is_locked_string_with_quotes() {
        let warning = WarningType::AttributeValueIsLocked("attr\"with'quotes".to_string());
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::AttributeValueIsLocked(\"attr\\\"with'quotes\")"
        );
    }

    #[test]
    fn attribute_value_is_locked_string_with_newlines() {
        let warning = WarningType::AttributeValueIsLocked("attr\nwith\nnewlines".to_string());
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::AttributeValueIsLocked(\"attr\\nwith\\nnewlines\")"
        );
    }

    #[test]
    fn level0_section_heading_not_supported() {
        let warning = WarningType::Level0SectionHeadingNotSupported;
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::Level0SectionHeadingNotSupported"
        );
    }

    #[test]
    fn section_heading_level_skipped() {
        let warning = WarningType::SectionHeadingLevelSkipped(2, 4);
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::SectionHeadingLevelSkipped(2, 4)"
        );
    }

    #[test]
    fn section_heading_level_exceeds_maximum() {
        let warning = WarningType::SectionHeadingLevelExceedsMaximum(6);
        let debug_output = format!("{:?}", warning);
        assert_eq!(
            debug_output,
            "WarningType::SectionHeadingLevelExceedsMaximum(6)"
        );
    }
}
