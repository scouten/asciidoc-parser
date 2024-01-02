//! Block elements form the main structure of an AsciiDoc document, starting
//! with the document itself.
//!
//! A block element (aka block) is a discrete, line-oriented chunk of content in
//! an AsciiDoc document. Once parsed, that chunk of content becomes a block
//! element in the parsed document model. Certain blocks may contain other
//! blocks, so we say that blocks can be nested. The converter visits each block
//! in turn, in document order, converting it to a corresponding chunk of
//! output.

mod block;
pub use block::Block;

mod r#macro;
pub use r#macro::MacroBlock;

mod simple;
pub use simple::SimpleBlock;
