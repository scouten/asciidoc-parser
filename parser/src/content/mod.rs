//! Describes the content of a non-compound block after any relevant
//! [substitutions] have been performed.
//!
//! [substitutions]: https://docs.asciidoctor.org/asciidoc/latest/subs/

mod content;
pub use content::Content;

mod macros;

pub(crate) mod passthroughs;
pub(crate) use passthroughs::Passthroughs;

mod substitution_group;
pub(crate) use substitution_group::SubstitutionGroup;

mod substitution_step;
pub(crate) use substitution_step::SubstitutionStep;
