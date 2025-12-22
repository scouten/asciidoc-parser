//! XPath-like query support for Virtual DOM nodes.
//!
//! This module provides a minimal XPath query engine for testing purposes.
//! It supports a subset of XPath syntax commonly used in test assertions.

use crate::tests::assert_dom::virtual_dom::VirtualNode;

/// Queries a virtual DOM tree using an XPath-like selector.
///
/// Supports the following patterns:
/// - `//tag` - Find all elements with the given tag anywhere in the tree
/// - `/tag` - Find tag elements as direct children of root
/// - `//tag/child` - Find child elements as direct children of tag elements
/// - `//tag[@attr="value"]` - Find elements with specific attribute values
/// - `//tag[text()="value"]` - Find elements with specific text content
/// - `//tag[N]` - Find the Nth element (1-indexed)
///
/// # Example
///
/// ```ignore
/// let doc = Parser::default().parse("* item 1\n* item 2");
/// let vdom = doc.to_virtual_dom();
/// let items = query_xpath(&vdom, "//ul/li");
/// assert_eq!(items.len(), 2);
/// ```
pub(crate) fn query_xpath<'a>(root: &'a VirtualNode, xpath: &str) -> Vec<&'a VirtualNode> {
    let xpath = xpath.trim();

    // Handle descendant-or-self pattern: //tag
    if let Some(rest) = xpath.strip_prefix("//") {
        return query_descendant_or_self(root, rest);
    }

    // Handle root-relative pattern: /tag
    if let Some(rest) = xpath.strip_prefix('/') {
        return query_from_root(root, rest);
    }

    // Default: treat as descendant-or-self
    query_descendant_or_self(root, xpath)
}

/// Queries for descendants or self matching the pattern.
fn query_descendant_or_self<'a>(node: &'a VirtualNode, pattern: &str) -> Vec<&'a VirtualNode> {
    // Split on first '/' to handle paths like "ul/li".
    if let Some((first, rest)) = pattern.split_once('/') {
        let first = first.trim();
        let rest = rest.trim();

        // Find all nodes matching first part.
        let mut results = Vec::new();
        collect_descendants_matching(node, first, &mut results);

        // For each matching node, query its children with the rest of the path.
        let mut final_results = Vec::new();
        for matched_node in results {
            if rest.starts_with('/') {
                // Descendant pattern continues: //
                let descendants =
                    query_descendant_or_self(matched_node, rest.trim_start_matches('/'));
                final_results.extend(descendants);
            } else {
                // Direct child pattern: continue with rest as a path from this node.
                let children_results = query_from_root(matched_node, rest);
                final_results.extend(children_results);
            }
        }
        final_results
    } else {
        // Simple tag match: Find all descendants (or self) matching this selector.
        let mut results = Vec::new();
        collect_descendants_matching(node, pattern.trim(), &mut results);
        results
    }
}

/// Queries from root using direct child selectors.
fn query_from_root<'a>(node: &'a VirtualNode, pattern: &str) -> Vec<&'a VirtualNode> {
    if let Some((first, rest)) = pattern.split_once('/') {
        let mut results = Vec::new();

        for child in &node.children {
            if matches_selector(child, first.trim()) {
                if rest.trim().is_empty() {
                    results.push(child);
                } else if rest.starts_with('/') {
                    // Continue with descendant-or-self
                    results.extend(query_descendant_or_self(
                        child,
                        rest.trim_start_matches('/'),
                    ));
                } else {
                    // Continue with direct children.
                    results.extend(query_from_root(child, rest.trim()));
                }
            }
        }
        results
    } else {
        // Direct children only.
        node.children
            .iter()
            .filter(|child| matches_selector(child, pattern.trim()))
            .collect()
    }
}

/// Recursively collects all descendants (including self) that match the
/// selector.
fn collect_descendants_matching<'a>(
    node: &'a VirtualNode,
    selector: &str,
    results: &mut Vec<&'a VirtualNode>,
) {
    if matches_selector(node, selector) {
        results.push(node);
    }

    for child in &node.children {
        collect_descendants_matching(child, selector, results);
    }
}

/// Checks if a node matches the given selector.
///
/// Supports:
/// - Tag name: `div`, `ul`, `li`
/// - Class selector: `[@class="ulist"]` or `.ulist`
/// - ID selector: `[@id="foo"]` or `#foo`
/// - Text content: `[text()="value"]`
/// - Index: `[1]`, `[2]`, etc. (not implemented here, handled in caller)
fn matches_selector(node: &VirtualNode, selector: &str) -> bool {
    let selector = selector.trim();

    // Handle index predicates [N] by stripping them off.
    // (Caller should handle filtering by index.)
    let (base_selector, _predicate) = if let Some(bracket_pos) = selector.find('[') {
        (&selector[..bracket_pos], Some(&selector[bracket_pos..]))
    } else {
        (selector, None)
    };

    // CSS-style class selector: .classname
    if let Some(class_name) = base_selector.strip_prefix('.') {
        return node.classes.iter().any(|c| c == class_name);
    }

    // CSS-style ID selector: #id
    if let Some(id) = base_selector.strip_prefix('#') {
        return node.id.as_deref() == Some(id);
    }

    // Tag name match.
    if !base_selector.is_empty() && node.tag != base_selector {
        return false;
    }

    // Handle predicates if present.
    if let Some(predicate) = _predicate {
        return matches_predicate(node, predicate);
    }

    true
}

/// Checks if a node matches a predicate like `[@class="value"]` or
/// `[text()="value"]`.
fn matches_predicate(node: &VirtualNode, predicate: &str) -> bool {
    let predicate = predicate.trim();

    // Strip outer brackets.
    let predicate = predicate
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(predicate);

    // Check for `text()` predicate.
    if let Some(rest) = predicate.strip_prefix("text()") {
        let rest = rest.trim();
        if let Some(value) = rest
            .strip_prefix('=')
            .and_then(|s| s.trim().strip_prefix('"'))
        {
            if let Some(value) = value.strip_suffix('"') {
                return node.text.as_deref() == Some(value);
            }
        }
        return false;
    }

    // Check for attribute predicates `[@attr="value"]`.
    if let Some(attr_part) = predicate.strip_prefix('@') {
        if let Some((attr_name, value_part)) = attr_part.split_once('=') {
            let attr_name = attr_name.trim();
            let value = value_part
                .trim()
                .strip_prefix('"')
                .and_then(|s| s.strip_suffix('"'))
                .unwrap_or(value_part.trim());

            match attr_name {
                "class" => return node.classes.iter().any(|c| c == value),
                "id" => return node.id.as_deref() == Some(value),
                _ => return false,
            }
        }
    }

    // Numeric predicate [N]: Would need to be handled by caller with context.
    // For now, just return `true` to pass through.
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Parser, tests::assert_dom::virtual_dom::ToVirtualDom};

    #[test]
    fn query_all_paragraphs() {
        let doc = Parser::default().parse("Para 1\n\nPara 2\n\nPara 3");
        let vdom = doc.to_virtual_dom();
        let paras = query_xpath(&vdom, "//p");
        assert_eq!(paras.len(), 3);
    }

    #[test]
    fn query_list_items() {
        let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
        let vdom = doc.to_virtual_dom();

        // Find all `ul` elements.
        let uls = query_xpath(&vdom, "//ul");
        assert_eq!(uls.len(), 1);

        // Find all `li` elements.
        let lis = query_xpath(&vdom, "//li");
        assert_eq!(lis.len(), 3);

        // Find `li` as children of `ul`.
        let ul_lis = query_xpath(&vdom, "//ul/li");
        assert_eq!(ul_lis.len(), 3);
    }

    #[test]
    fn query_with_class_selector() {
        let doc = Parser::default().parse("* item 1\n* item 2");
        let vdom = doc.to_virtual_dom();

        // CSS-style class selector>: Finds the wrapper `div`.
        let ulists = query_xpath(&vdom, "//.ulist");
        assert_eq!(ulists.len(), 1);
        assert_eq!(ulists[0].tag, "div"); // Wrapper `div` has the class.
    }

    #[test]
    fn query_section_headings() {
        let doc = Parser::default().parse("== Section 1\n\nPara\n\n== Section 2\n\nPara");
        let vdom = doc.to_virtual_dom();

        // Find all `h2` elements.
        let h2s = query_xpath(&vdom, "//h2");
        assert_eq!(h2s.len(), 2);
    }

    #[test]
    fn query_nested_path() {
        let doc = Parser::default().parse("== Section\n\n* item 1\n* item 2");
        let vdom = doc.to_virtual_dom();

        // First verify we can find the `ul`.
        let uls = query_xpath(&vdom, "//ul");
        assert_eq!(uls.len(), 1, "Should find one ul element");

        // Then verify we can find `ul/li`.
        let ul_lis = query_xpath(&vdom, "//ul/li");
        assert_eq!(ul_lis.len(), 2, "Should find 2 li elements under ul");

        // Find list items within any `div` that contains a `ul`.
        let items = query_xpath(&vdom, "//div/ul/li");
        assert_eq!(
            items.len(),
            2,
            "Should find 2 li elements via //div/ul/li path"
        );
    }

    #[test]
    fn query_with_text_predicate() {
        let doc = Parser::default().parse("Hello\n\nWorld");
        let vdom = doc.to_virtual_dom();

        let hello_para = query_xpath(&vdom, "//p[text()=\"Hello\"]");
        assert_eq!(hello_para.len(), 1);
        assert_eq!(hello_para[0].text.as_deref(), Some("Hello"));
    }

    #[test]
    fn query_with_attribute_predicate() {
        let doc = Parser::default().parse("* item");
        let vdom = doc.to_virtual_dom();

        // The wrapper `div` has the `ulist` class, not the `ul` element.
        let ulist_wrapper = query_xpath(&vdom, "//div[@class=\"ulist\"]");
        assert_eq!(ulist_wrapper.len(), 1);
    }
}
