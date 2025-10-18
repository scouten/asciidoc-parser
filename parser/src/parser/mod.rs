//! The [`Parser`] struct and its related structs allow a caller to configure
//! how AsciiDoc parsing occurs and then to initiate the parsing process.

mod attribute_value;
pub(crate) use attribute_value::AttributeValue;
pub use attribute_value::{AllowableValue, ModificationContext};

mod built_in_attrs;

mod include_file_handler;
pub use include_file_handler::IncludeFileHandler;

mod inline_substitution_renderer;
pub use inline_substitution_renderer::{
    CharacterReplacementType, HtmlSubstitutionRenderer, IconRenderParams, ImageRenderParams,
    InlineSubstitutionRenderer, LinkRenderParams, LinkRenderType, QuoteScope, QuoteType,
    SpecialCharacter,
};

mod parser;
pub use parser::Parser;

mod path_resolver;
pub use path_resolver::PathResolver;

pub(crate) mod preprocessor;

mod source_map;
pub use source_map::{SourceLine, SourceMap};
