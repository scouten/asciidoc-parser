#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::unwrap_used)]
#![deny(missing_docs)]
#![deny(warnings)]
#![doc = include_str!("../README.md")]

/// TEMPORARY demo function
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub mod strings;

#[cfg(test)]
mod tests;
