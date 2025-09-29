mod impl_debug {
    use pretty_assertions_sorted::assert_eq;

    use crate::document::InterpretedValue;

    #[test]
    fn value_empty_string() {
        let interpreted_value = InterpretedValue::Value("".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(debug_output, "InterpretedValue::Value(\"\")");
    }

    #[test]
    fn value_simple_string() {
        let interpreted_value = InterpretedValue::Value("hello".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(debug_output, "InterpretedValue::Value(\"hello\")");
    }

    #[test]
    fn value_string_with_spaces() {
        let interpreted_value = InterpretedValue::Value("hello world".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(debug_output, "InterpretedValue::Value(\"hello world\")");
    }

    #[test]
    fn value_string_with_special_chars() {
        let interpreted_value = InterpretedValue::Value("test!@#$%^&*()".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(debug_output, "InterpretedValue::Value(\"test!@#$%^&*()\")");
    }

    #[test]
    fn value_string_with_quotes() {
        let interpreted_value = InterpretedValue::Value("value\"with'quotes".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(
            debug_output,
            "InterpretedValue::Value(\"value\\\"with'quotes\")"
        );
    }

    #[test]
    fn value_string_with_newlines() {
        let interpreted_value = InterpretedValue::Value("line1\nline2\nline3".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(
            debug_output,
            "InterpretedValue::Value(\"line1\\nline2\\nline3\")"
        );
    }

    #[test]
    fn value_string_with_backslashes() {
        let interpreted_value = InterpretedValue::Value("path\\to\\file".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(
            debug_output,
            "InterpretedValue::Value(\"path\\\\to\\\\file\")"
        );
    }

    #[test]
    fn value_string_with_unicode() {
        let interpreted_value = InterpretedValue::Value("café 🚀 ñoño".to_string());
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(debug_output, "InterpretedValue::Value(\"café 🚀 ñoño\")");
    }

    #[test]
    fn set() {
        let interpreted_value = InterpretedValue::Set;
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(debug_output, "InterpretedValue::Set");
    }

    #[test]
    fn unset() {
        let interpreted_value = InterpretedValue::Unset;
        let debug_output = format!("{:?}", interpreted_value);
        assert_eq!(debug_output, "InterpretedValue::Unset");
    }
}
