//! DOM assertion utilities for testing.
//!
//! This module provides assertion functions that can be used in tests
//! to verify the structure of parsed AsciiDoc documents.

use crate::Document;

/// Asserts that an XPath query matches exactly the expected number of nodes.
///
/// # Panics
///
/// Panics if the number of matches doesn't equal `expected_count`.
///
/// # Examples
///
/// ```ignore
/// let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
/// assert_xpath(&doc, "//ul", 1);
/// assert_xpath(&doc, "//li", 3);
/// ```
#[track_caller]
pub(crate) fn assert_xpath(doc: &Document, xpath: &str, expected_count: usize) {
    let vdom = doc.to_virtual_dom();
    let matches = query_xpath(&vdom, xpath);
    assert_eq!(
        matches.len(),
        expected_count,
        "XPath query '{}' expected {} matches, found {}",
        xpath,
        expected_count,
        matches.len()
    );
}

/// Asserts that a CSS selector matches exactly the expected number of nodes.
///
/// Currently this is an alias for `assert_xpath` since our query engine
/// supports both XPath and CSS-style selectors.
///
/// # Panics
///
/// Panics if the number of matches doesn't equal `expected_count`.
///
/// # Examples
///
/// ```ignore
/// let doc = Parser::default().parse("* item");
/// assert_css(&doc, ".ulist", 1);
/// assert_css(&doc, "ul li", 1);
/// ```
#[track_caller]
pub(crate) fn assert_css(doc: &Document, selector: &str, expected_count: usize) {
    // For now, treat CSS selectors the same as XPath queries.
    // We support CSS-style syntax like .class and #id already.
    assert_xpath(doc, selector, expected_count);
}

mod virtual_dom;
use virtual_dom::ToVirtualDom;

mod xpath;
use xpath::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn assert_xpath_success() {
        let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
        assert_xpath(&doc, "//ul", 1);
        assert_xpath(&doc, "//li", 3);
    }

    #[test]
    #[should_panic(expected = "XPath query '//ul' expected 2 matches, found 1")]
    fn assert_xpath_failure() {
        let doc = Parser::default().parse("* item");
        assert_xpath(&doc, "//ul", 2); // Should panic
    }

    #[test]
    fn assert_css_with_class_selector() {
        let doc = Parser::default().parse("* item");
        assert_css(&doc, ".ulist", 1);
    }

    #[test]
    fn assert_xpath_with_path() {
        let doc = Parser::default().parse("== Section\n\n* item 1\n* item 2");
        assert_xpath(&doc, "//div/ul/li", 2);
    }

    #[test]
    fn assert_xpath_with_predicate() {
        let doc = Parser::default().parse("Hello\n\nWorld");
        assert_xpath(&doc, "//p[text()=\"Hello\"]", 1);
    }
}
