#![allow(unused)]
// I'm generally not a fan of preludes, but for repetitive test infrastructure,
// I'm making an exception.

pub(crate) use crate::{
    blocks::SectionType,
    tests::{
        fixtures::{attributes::*, blocks::*, content::*, document::*, parser::*, warnings::*, *},
        sdd::*,
    },
};
