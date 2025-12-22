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
/// - `//tag[N]/*` - Find all children of the Nth element
/// - `//*` or `*` - Match any element
/// - `(//tag)[N]/child` - Apply predicate to subquery results
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

    // Handle parenthesized subqueries: (//tag)[N]/rest
    if xpath.starts_with('(') {
        return query_parenthesized(root, xpath);
    }

    // Handle descendant-or-self pattern: //tag
    if let Some(rest) = xpath.strip_prefix("//") {
        return query_descendant_or_self(root, rest);
    }

    // Handle root-relative pattern: /tag
    if let Some(rest) = xpath.strip_prefix('/') {
        return query_from_root(root, rest);
    }

    // Default: treat as descendant-or-self.
    query_descendant_or_self(root, xpath)
}

/// Handles parenthesized subqueries like `(//ul)[1]/li`.
fn query_parenthesized<'a>(root: &'a VirtualNode, xpath: &str) -> Vec<&'a VirtualNode> {
    // Find the matching closing parenthesis.
    if let Some(close_paren) = xpath.find(')') {
        let subquery = &xpath[1..close_paren]; // Extract content between parentheses.
        let rest = &xpath[close_paren + 1..].trim_start(); // Everything after ).

        // Execute the subquery.
        let mut results = query_xpath(root, subquery);

        // Check if there's a predicate immediately after the closing paren.
        if let Some(rest) = rest.strip_prefix('[') {
            if let Some(bracket_end) = rest.find(']') {
                let predicate = &rest[..bracket_end];
                let remaining = &rest[bracket_end + 1..];

                // Apply numeric predicate.
                if let Ok(index) = predicate.trim().parse::<usize>() {
                    if index > 0 && index <= results.len() {
                        results = vec![results[index - 1]];
                    } else {
                        return vec![];
                    }
                }

                // Continue with the remaining path if any.
                if !remaining.is_empty() {
                    let mut final_results = Vec::new();
                    for node in results {
                        if remaining.starts_with('/') && !remaining.starts_with("//") {
                            // Direct child path.
                            final_results.extend(query_from_root(node, &remaining[1..]));
                        } else if remaining.starts_with("//") {
                            // Descendant path.
                            final_results.extend(query_descendant_or_self(node, &remaining[2..]));
                        }
                    }
                    return final_results;
                }
            }
        } else if !rest.is_empty() {
            // Continue with the remaining path without a predicate.
            let mut final_results = Vec::new();
            for node in results {
                if rest.starts_with('/') && !rest.starts_with("//") {
                    // Direct child path.
                    final_results.extend(query_from_root(node, &rest[1..]));
                } else if rest.starts_with("//") {
                    // Descendant path.
                    final_results.extend(query_descendant_or_self(node, &rest[2..]));
                }
            }
            return final_results;
        }

        return results;
    }

    // No valid parenthesized expression found.
    vec![]
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

        // Apply numeric predicate if present.
        results = apply_numeric_predicate(results, first);

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
        apply_numeric_predicate(results, pattern.trim())
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

        apply_numeric_predicate(results, first.trim())
    } else {
        // Direct children only.
        let results: Vec<&VirtualNode> = node
            .children
            .iter()
            .filter(|child| matches_selector(child, pattern.trim()))
            .collect();

        apply_numeric_predicate(results, pattern.trim())
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

/// Applies numeric predicate filtering (e.g., [1], [2]) to a results vector.
/// Returns the filtered results or the original results if no numeric predicate
/// is present.
fn apply_numeric_predicate<'a>(
    results: Vec<&'a VirtualNode>,
    selector: &str,
) -> Vec<&'a VirtualNode> {
    // Extract numeric predicate [N] from selector.
    if let Some(bracket_pos) = selector.find('[') {
        if let Some(predicate) = selector[bracket_pos..]
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
        {
            // Check if it's a numeric predicate.
            if let Ok(index) = predicate.trim().parse::<usize>() {
                // XPath uses 1-based indexing.
                if index > 0 && index <= results.len() {
                    return vec![results[index - 1]];
                } else {
                    return vec![];
                }
            }
        }
    }

    results
}

/// Checks if a node matches the given selector.
///
/// Supports:
/// - Tag name: `div`, `ul`, `li`
/// - Wildcard: `*` (matches any element)
/// - Class selector: `[@class="ulist"]` or `.ulist`
/// - ID selector: `[@id="foo"]` or `#foo`
/// - Text content: `[text()="value"]`
/// - Index: `[1]`, `[2]`, etc. (handled by `apply_numeric_predicate`)
fn matches_selector(node: &VirtualNode, selector: &str) -> bool {
    let selector = selector.trim();

    // Handle index predicates [N] by stripping them off.
    // (Caller should handle filtering by index.)
    let (base_selector, _predicate) = if let Some(bracket_pos) = selector.find('[') {
        (&selector[..bracket_pos], Some(&selector[bracket_pos..]))
    } else {
        (selector, None)
    };

    // Wildcard selector: matches any element.
    if base_selector == "*" {
        if let Some(predicate) = _predicate {
            return matches_predicate(node, predicate);
        }
        return true;
    }

    // CSS-style class selector: `.classname`
    if let Some(class_name) = base_selector.strip_prefix('.') {
        return node.classes.iter().any(|c| c == class_name);
    }

    // CSS-style ID selector: `#id`
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

    #[test]
    fn query_with_numeric_predicate() {
        let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
        let vdom = doc.to_virtual_dom();

        let first_li = query_xpath(&vdom, "//ul/li[1]");
        assert_eq!(first_li.len(), 1);

        let second_li = query_xpath(&vdom, "//ul/li[2]");
        assert_eq!(second_li.len(), 1);

        let third_li = query_xpath(&vdom, "//ul/li[3]");
        assert_eq!(third_li.len(), 1);

        let fourth_li = query_xpath(&vdom, "//ul/li[4]");
        assert_eq!(fourth_li.len(), 0);
    }

    #[test]
    fn query_with_wildcard() {
        let doc = Parser::default().parse("* item 1\n* item 2");
        let vdom = doc.to_virtual_dom();

        let all_elements = query_xpath(&vdom, "//*");
        assert!(all_elements.len() > 0);

        let first_li_children = query_xpath(&vdom, "//ul/li[1]/*");
        assert_eq!(first_li_children.len(), 1);
        assert_eq!(first_li_children[0].tag, "p");
    }

    #[test]
    fn query_numeric_predicate_with_wildcard() {
        let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
        let vdom = doc.to_virtual_dom();

        let first_li_all_children = query_xpath(&vdom, "//ul/li[1]/*");
        assert_eq!(first_li_all_children.len(), 1);
        assert_eq!(first_li_all_children[0].tag, "p");

        let second_li_all_children = query_xpath(&vdom, "//ul/li[2]/*");
        assert_eq!(second_li_all_children.len(), 1);
        assert_eq!(second_li_all_children[0].tag, "p");
    }

    #[test]
    fn query_comprehensive_numeric_wildcard() {
        // This test demonstrates the full `//ul/li[1]/*` pattern.
        let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
        let vdom = doc.to_virtual_dom();

        // The pattern `//ul/li[1]/*` means:
        // 1. Find all <ul> elements (//ul)
        // 2. Get their <li> children (/li)
        // 3. Take only the first one ([1])
        // 4. Get all its children (/*)
        let result = query_xpath(&vdom, "//ul/li[1]/*");

        assert_eq!(result.len(), 1, "Should find exactly 1 child element");
        assert_eq!(result[0].tag, "p", "The child should be a paragraph");

        assert_eq!(
            result[0].text.as_deref(),
            Some("item 1"),
            "The paragraph should contain 'item 1'"
        );
    }

    #[test]
    fn query_parenthesized_with_predicate() {
        // Create a document with multiple lists separated by a comment.
        // This mimics the pattern from the actual test suite.
        let doc = Parser::default().parse("- Foo\n- Boo\n\n//\n\n- Blech\n");
        let vdom = doc.to_virtual_dom();

        let all_uls = query_xpath(&vdom, "//ul");
        assert_eq!(all_uls.len(), 2, "Should have 2 ul elements");

        let first_ul = query_xpath(&vdom, "(//ul)[1]");
        assert_eq!(first_ul.len(), 1, "Should find exactly 1 ul");

        let second_ul = query_xpath(&vdom, "(//ul)[2]");
        assert_eq!(second_ul.len(), 1, "Should find exactly 1 ul");

        let first_ul_lis = query_xpath(&vdom, "(//ul)[1]/li");
        assert_eq!(first_ul_lis.len(), 2, "First ul should have 2 li elements");

        let second_ul_lis = query_xpath(&vdom, "(//ul)[2]/li");
        assert_eq!(second_ul_lis.len(), 1, "Second ul should have 1 li element");
    }

    #[test]
    fn query_parenthesized_complex() {
        let doc = Parser::default().parse("- Foo\n- Boo\n\n.Also\n- Blech\n");
        let vdom = doc.to_virtual_dom();

        let all_uls = query_xpath(&vdom, "//ul");
        assert_eq!(all_uls.len(), 2);

        let result = query_xpath(&vdom, "(//ul)[1]/li[1]");
        assert_eq!(result.len(), 1);

        let result = query_xpath(&vdom, "(//ul)[2]/li");
        assert_eq!(result.len(), 1);

        let result = query_xpath(&vdom, "(//ul)[3]/li");
        assert_eq!(result.len(), 0, "Out of bounds should return empty");
    }

    #[test]
    fn query_parenthesized_complex_2() {
        let doc = Parser::default().parse("List\n====\n\n- Foo\n- Boo\n\n//\n\n- Blech\n");
        let vdom = doc.to_virtual_dom();

        assert_eq!(query_xpath(&vdom, "//ul").len(), 2);

        let first_ul_items = query_xpath(&vdom, "(//ul)[1]/li");
        assert_eq!(first_ul_items.len(), 2);

        let second_ul_items = query_xpath(&vdom, "(//ul)[2]/li");
        assert_eq!(second_ul_items.len(), 1);

        let all_items = query_xpath(&vdom, "//ul/li");
        assert_eq!(all_items.len(), 3);
    }
}
