// Tests are grouped under this module so as to avoid
// having the test code itself included in coverage numbers.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn it_works() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}

mod fixtures;
mod strings;
