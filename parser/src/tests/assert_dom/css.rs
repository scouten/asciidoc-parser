//! CSS-like query support for Virtual DOM nodes.
//!
//! This module provides a minimal CSS query engine for testing purposes.
//! It supports a subset of CSS syntax used in test assertions.

use crate::tests::assert_dom::virtual_dom::VirtualNode;

/// Queries a virtual DOM tree using an CSS-like selector.
///
/// Supports the following patterns:
/// - `tag` - Find all elements with the given tag anywhere in the tree
/// - `tag child` - Find child elements as direct children of tag elements
///
/// # Example
///
/// ```ignore
/// let doc = Parser::default().parse("* item 1\n* item 2");
/// let vdom = doc.to_virtual_dom();
/// let items = query_css(&vdom, "ul li");
/// assert_eq!(items.len(), 2);
/// ```
pub(crate) fn query_css<'a>(root: &'a VirtualNode, selector: &str) -> Vec<&'a VirtualNode> {
    let selector = selector.trim();

    // So far, only descendant-or-self pattern is supported: tag
    query_descendant_or_self(root, selector)
}

/// Queries for descendants or self matching the pattern.
fn query_descendant_or_self<'a>(node: &'a VirtualNode, pattern: &str) -> Vec<&'a VirtualNode> {
    // Split on first ' ' to handle paths like "ul li".
    if let Some((first, rest)) = pattern.split_once(' ') {
        let first = first.trim();
        let rest = rest.trim();

        // Find all nodes matching first part.
        let mut results = Vec::new();
        collect_descendants_matching(node, first, &mut results);

        // For each matching node, query its children with the rest of the path.
        let mut final_results = Vec::new();
        for matched_node in results {
            for child in &matched_node.children {
                let descendants = query_descendant_or_self(child, rest.trim());
                final_results.extend(descendants);
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
/// - Wildcard: `*` (matches any element)
/// - Class selector: `[@class="ulist"]` or `.ulist`
/// - ID selector: `[@id="foo"]` or `#foo`
/// - Text content: `[text()="value"]`
/// - Index: `[1]`, `[2]`, etc. (handled by `apply_numeric_predicate`)
fn matches_selector(node: &VirtualNode, selector: &str) -> bool {
    let selector = selector.trim();

    // Handle index predicates [N] by stripping them off.
    // (Caller should handle filtering by index.)
    let (base_selector, predicate) = if let Some(bracket_pos) = selector.find('[') {
        (&selector[..bracket_pos], Some(&selector[bracket_pos..]))
    } else {
        (selector, None)
    };

    // Wildcard selector: matches any element.
    if base_selector == "*" {
        if let Some(predicate) = predicate {
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
    if let Some(predicate) = predicate {
        return matches_predicate(node, predicate);
    }

    true
}

/// Checks if a node matches a predicate like `[@class="value"]` or
/// `[text()="value"]`.
/// Can handle multiple predicates like `[@class="value"][text()="text"]`.
fn matches_predicate(node: &VirtualNode, predicate: &str) -> bool {
    let mut predicate = predicate.trim();

    // Handle multiple predicates by checking each one.
    while !predicate.is_empty() {
        // Find the next closing bracket.
        if let Some(bracket_start) = predicate.find('[') {
            if let Some(bracket_end) = predicate[bracket_start..].find(']') {
                let bracket_end = bracket_start + bracket_end;
                let single_pred = &predicate[bracket_start + 1..bracket_end];

                // Check this single predicate.
                if !matches_single_predicate(node, single_pred) {
                    return false;
                }

                // Move to the next predicate.
                predicate = predicate[bracket_end + 1..].trim();
            } else {
                // Malformed predicate.
                return false;
            }
        } else {
            // No more predicates.
            break;
        }
    }

    true
}

/// Checks if a node matches a single predicate.
fn matches_single_predicate(node: &VirtualNode, predicate: &str) -> bool {
    let predicate = predicate.trim();

    // Check for `text()` predicate.
    if let Some(rest) = predicate.strip_prefix("text()") {
        let rest = rest.trim();

        // Handle text() = 'value' (single quotes).
        if let Some(value_part) = rest.strip_prefix('=').map(|s| s.trim()) {
            // Try single-quoted string first.
            if let Some(value) = value_part.strip_prefix('\'') {
                if let Some(value) = value.strip_suffix('\'') {
                    let unescaped = unescape_css_string(value);
                    return node.text.as_deref() == Some(&unescaped);
                }
            }
            // Try double-quoted string.
            else if let Some(value) = value_part.strip_prefix('"') {
                if let Some(value) = value.strip_suffix('"') {
                    let unescaped = unescape_css_string(value);
                    return node.text.as_deref() == Some(&unescaped);
                }
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

/// Unescapes CSS string literals.
/// Handles escape sequences like `\n` (newline), `\'` (single quote), `\\`
/// (backslash).
fn unescape_css_string(s: &str) -> String {
    let mut result = String::new();
    let mut chars = s.chars();

    while let Some(ch) = chars.next() {
        if ch == '\\' {
            // Handle escape sequence.
            if let Some(next) = chars.next() {
                match next {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    '\\' => result.push('\\'),
                    '\'' => result.push('\''),
                    '"' => result.push('"'),
                    _ => {
                        // Unknown escape - keep as is.
                        result.push('\\');
                        result.push(next);
                    }
                }
            } else {
                result.push('\\');
            }
        } else {
            result.push(ch);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Parser, tests::assert_dom::virtual_dom::ToVirtualDom};

    #[test]
    fn query_all_paragraphs() {
        let doc = Parser::default().parse("Para 1\n\nPara 2\n\nPara 3");
        let vdom = doc.to_virtual_dom();
        let paras = query_css(&vdom, "p");
        assert_eq!(paras.len(), 3);
    }

    #[test]
    fn query_list_items() {
        let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
        let vdom = doc.to_virtual_dom();

        // Find all `ul` elements.
        let uls = query_css(&vdom, "ul");
        assert_eq!(uls.len(), 1);

        // Find all `li` elements.
        let lis = query_css(&vdom, "li");
        assert_eq!(lis.len(), 3);

        // Find `li` as children of `ul`.
        let ul_lis = query_css(&vdom, "ul li");
        assert_eq!(ul_lis.len(), 3);
    }
}
