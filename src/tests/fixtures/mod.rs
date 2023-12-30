use std::{env, path::PathBuf};

pub(crate) mod blocks;
pub(crate) mod document;

mod span;
pub(crate) use span::TSpan;

#[allow(dead_code)] // TEMPORARY while bootstrapping
pub(crate) fn fixture_path(name: &str) -> String {
    let root_dir = &env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut path = PathBuf::from(root_dir);
    path.push("src/tests/fixtures");
    path.push(name);

    assert!(path.exists());

    path.to_str().unwrap().to_string()
}
