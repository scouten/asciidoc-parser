#![allow(unused)]
// I'm generally not a fan of preludes, but for repetitive test infrastructure,
// I'm making an exception.

pub(crate) use crate::tests::{
    fixtures::{attributes::*, blocks::*, content::*, document::*, warnings::*, *},
    sdd::*,
};
