//! Virtual DOM representation of parsed AsciiDoc documents.
//!
//! This module provides a lightweight HTML-like representation of AsciiDoc
//! documents for testing purposes. It maps AsciiDoc block structures to their
//! HTML equivalents, enabling XPath-like queries for test assertions.

use crate::{
    Document,
    blocks::{
        Block, Break, CompoundDelimitedBlock, IsBlock, ListBlock, ListItem, ListItemMarker,
        ListType, MediaBlock, Preamble, RawDelimitedBlock, SectionBlock, SimpleBlock,
        SimpleBlockStyle,
    },
};

/// Decodes common HTML entities to their character equivalents.
///
/// This simulates what a browser would do when parsing HTML and accessing
/// text content via JavaScript's `textContent` or XPath's `text()`.
fn decode_html_entities(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
}

/// A virtual DOM node representing an HTML-like element.
///
/// This structure is built from a parsed `Document` and maps AsciiDoc blocks
/// to their HTML equivalents for testing purposes.
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualNode {
    /// HTML tag name (e.g., "ul", "li", "p", "div").
    pub tag: String,

    /// CSS classes applied to this element.
    pub classes: Vec<String>,

    /// Element ID attribute, if any.
    pub id: Option<String>,

    /// Text content of this element (for leaf nodes).
    pub text: Option<String>,

    /// Child elements.
    pub children: Vec<VirtualNode>,
}

#[allow(dead_code)] // TEMPORARY while building
impl VirtualNode {
    /// Creates a new virtual node with the specified tag.
    pub fn new(tag: impl Into<String>) -> Self {
        Self {
            tag: tag.into(),
            classes: Vec::new(),
            id: None,
            text: None,
            children: Vec::new(),
        }
    }

    /// Adds a CSS class to this node.
    pub fn with_class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Adds multiple CSS classes to this node.
    pub fn with_classes(mut self, classes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.classes.extend(classes.into_iter().map(Into::into));
        self
    }

    /// Sets the ID of this node.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the text content of this node.
    ///
    /// Decodes HTML entities to match what a browser's text content would show.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(decode_html_entities(&text.into()));
        self
    }

    /// Adds a child node.
    pub fn with_child(mut self, child: VirtualNode) -> Self {
        self.children.push(child);
        self
    }

    /// Adds multiple child nodes.
    pub fn with_children(mut self, children: impl IntoIterator<Item = VirtualNode>) -> Self {
        self.children.extend(children);
        self
    }
}

/// Trait for converting AsciiDoc structures to virtual DOM nodes.
pub trait ToVirtualDom {
    /// Converts this structure to a virtual DOM node.
    fn to_virtual_dom(&self) -> VirtualNode;
}

impl ToVirtualDom for Document<'_> {
    fn to_virtual_dom(&self) -> VirtualNode {
        let mut node = VirtualNode::new("div").with_class("document");

        // Add document ID if present.
        if let Some(id) = self.id() {
            node = node.with_id(id);
        }

        // Add child blocks, including block titles as separate siblings.
        for block in self.nested_blocks() {
            add_block_with_title(&mut node, block);
        }

        node
    }
}

/// Adds a block to the parent node, including its title as a separate sibling
/// element if present.
///
/// NOTE: Some block types (like lists) handle their titles internally, so we
/// skip adding a separate title element for those.
fn add_block_with_title<'a>(parent: &mut VirtualNode, block: &'a Block<'a>) {
    // Check if this block type handles its own title internally.
    let handles_title_internally = matches!(block, Block::List(_));

    // Add title as a separate sibling element if the block doesn't handle it
    // internally.
    if !handles_title_internally && let Some(title) = block.title() {
        // Add title as a separate div element with class="title".
        let title_node = VirtualNode::new("div").with_class("title").with_text(title);
        parent.children.push(title_node);
    }

    // Add the block itself (which will handle its own title if applicable).
    parent.children.push(block.to_virtual_dom());
}

impl ToVirtualDom for Block<'_> {
    fn to_virtual_dom(&self) -> VirtualNode {
        match self {
            Block::Simple(simple) => {
                // Comment blocks should not be rendered.
                if simple.declared_style() == Some("comment") {
                    return VirtualNode::new("comment");
                }

                let mut node = simple_block_to_node(simple);

                // Extract text content for paragraphs.
                if node.tag == "p" {
                    node.text = Some(simple.content().rendered().to_string());
                } else if simple.style() == SimpleBlockStyle::Literal
                    || simple.declared_style() == Some("literal")
                {
                    // For literal blocks, add a <pre> element containing the content.
                    let pre_node =
                        VirtualNode::new("pre").with_text(simple.content().rendered().to_string());
                    node = node.with_child(pre_node);
                }
                node
            }

            Block::List(list) => list_block_to_node(list),
            Block::ListItem(item) => list_item_to_node(item),

            Block::Section(section) => {
                let mut node = section_to_node(section);
                // Add section title as heading element.
                // Section levels are 1-5, which map to h2-h6.
                let heading_level = (section.level() + 1).min(6);
                let heading_tag = format!("h{}", heading_level);

                let title_node = VirtualNode::new(heading_tag).with_text(section.section_title());
                node.children.insert(0, title_node);
                node
            }

            Block::Media(media) => media_to_node(media),
            Block::RawDelimited(raw) => raw_delimited_to_node(raw),
            Block::CompoundDelimited(compound) => compound_delimited_to_node(compound),
            Block::Preamble(preamble) => preamble_to_node(preamble),
            Block::Break(break_) => break_to_node(break_),
            Block::DocumentAttribute(_) => {
                // Document attributes don't render in HTML.
                VirtualNode::new("comment")
            }
        }
    }
}

fn simple_block_to_node<'a>(block: &'a SimpleBlock<'a>) -> VirtualNode {
    let declared_style = block.declared_style();
    let block_style = block.style();

    // Determine tag and classes based on both declared style and block style.
    let (tag, wrapper_classes) =
        if block_style == SimpleBlockStyle::Literal || declared_style == Some("literal") {
            ("div", vec!["literalblock"])
        } else {
            match declared_style {
                Some("paragraph") | None => ("p", vec![]),
                Some("verse") => ("div", vec!["verseblock"]),
                Some("quote") => ("div", vec!["quoteblock"]),
                Some("sidebar") => ("div", vec!["sidebarblock"]),
                Some("example") => ("div", vec!["exampleblock"]),
                Some("open") => ("div", vec!["openblock"]),
                Some("pass") => ("div", vec!["passblock"]),
                _ => ("p", vec![]),
            }
        };

    let mut node = VirtualNode::new(tag);

    for class in wrapper_classes {
        node = node.with_class(class);
    }

    for role in block.roles() {
        node = node.with_class(role);
    }

    if let Some(id) = block.id() {
        node = node.with_id(id);
    }

    // TODO: Extract text content from title and body.
    node
}

fn list_block_to_node<'a>(list: &'a ListBlock<'a>) -> VirtualNode {
    let (list_tag, base_class) = match list.type_() {
        ListType::Unordered => ("ul", "ulist"),
        ListType::Ordered => ("ol", "olist"),
        ListType::Description => ("dl", "dlist"),
    };

    let mut list_element = VirtualNode::new(list_tag);

    // Add style class to the list element if present.
    if let Some(style) = list.declared_style() {
        list_element = list_element.with_class(style);
    }

    for item in list.nested_blocks() {
        list_element.children.push(item.to_virtual_dom());
    }

    // Wrap the list in a div container (matching Asciidoctor's HTML structure).
    let mut wrapper = VirtualNode::new("div").with_class(base_class);

    // Add style class to the wrapper if present.
    if let Some(style) = list.declared_style() {
        wrapper = wrapper.with_class(style);
    }

    for role in list.roles() {
        wrapper = wrapper.with_class(role);
    }

    if let Some(id) = list.id() {
        wrapper = wrapper.with_id(id);
    }

    // Add block title if present (inside the wrapper, before the list element).
    if let Some(title) = list.title() {
        let title_node = VirtualNode::new("div").with_class("title").with_text(title);
        wrapper.children.push(title_node);
    }

    wrapper.children.push(list_element);
    wrapper
}

fn list_item_to_node<'a>(item: &'a ListItem<'a>) -> VirtualNode {
    // TODO: Determine if this is a description list item.
    let mut node = VirtualNode::new("li");

    // Add "arabic" CSS class if the list marker is Dots.
    if matches!(item.list_item_marker(), ListItemMarker::Dots(_)) {
        node = node.with_class("arabic");
    }

    for role in item.roles() {
        node = node.with_class(role);
    }

    if let Some(id) = item.id() {
        node = node.with_id(id);
    }

    let nested = item.nested_blocks().collect::<Vec<_>>();
    let has_multiple_blocks = nested.len() > 1;

    for (index, child) in nested.iter().enumerate() {
        let child_vdom = child.to_virtual_dom();

        // Wrap paragraphs in div.paragraph when they appear after other blocks in the
        // list item. This matches Asciidoctor's HTML output for list
        // continuations. The first paragraph block is never wrapped, only
        // subsequent ones.
        if has_multiple_blocks
            && index > 0
            && child_vdom.tag == "p"
            && child_vdom.classes.is_empty()
        {
            let wrapper = VirtualNode::new("div")
                .with_class("paragraph")
                .with_child(child_vdom);
            node.children.push(wrapper);
        } else {
            node.children.push(child_vdom);
        }
    }

    node
}

fn section_to_node<'a>(section: &'a SectionBlock<'a>) -> VirtualNode {
    // TODO: Adjust class depending on section level.
    let mut node = VirtualNode::new("div").with_class("sect1");

    for role in section.roles() {
        node = node.with_class(role);
    }

    if let Some(id) = section.id() {
        node = node.with_id(id);
    }

    // TODO: Section title heading is added in the Block::Section match arm
    // using section.level() to determine the heading level (h2-h6) and
    // section.section_title() to get the rendered title text.

    // Add nested blocks, handling block titles as separate siblings.
    for child in section.nested_blocks() {
        add_block_with_title(&mut node, child);
    }

    node
}

fn media_to_node<'a>(media: &'a MediaBlock<'a>) -> VirtualNode {
    // Media blocks render as <div class="imageblock"> or similar.
    let context = media.raw_context();
    let class = format!("{}block", context.as_ref());

    let mut node = VirtualNode::new("div").with_class(class);

    for role in media.roles() {
        node = node.with_class(role);
    }

    if let Some(id) = media.id() {
        node = node.with_id(id);
    }

    // TODO: Media blocks typically contain an <img> or similar element.
    // We'll add this when we need more detailed media representation.

    node
}

fn raw_delimited_to_node<'a>(raw: &'a RawDelimitedBlock<'a>) -> VirtualNode {
    let context = raw.raw_context();

    let (tag, classes): (&str, Vec<String>) = match context.as_ref() {
        "listing" => ("div", vec!["listingblock".to_string()]),
        "literal" => ("div", vec!["literalblock".to_string()]),
        "comment" => ("comment", vec![]),
        _ => ("div", vec![format!("{}block", context.as_ref())]),
    };

    let mut node = VirtualNode::new(tag);
    for class in classes {
        node = node.with_class(class);
    }

    for role in raw.roles() {
        node = node.with_class(role);
    }

    if let Some(id) = raw.id() {
        node = node.with_id(id);
    }

    if tag != "comment" {
        let pre = VirtualNode::new("pre");
        node.children.push(pre);
    }

    node
}

fn compound_delimited_to_node<'a>(compound: &'a CompoundDelimitedBlock<'a>) -> VirtualNode {
    let context = compound.raw_context();
    let class = format!("{}block", context.as_ref());

    let mut node = VirtualNode::new("div").with_class(class);

    for role in compound.roles() {
        node = node.with_class(role);
    }

    if let Some(id) = compound.id() {
        node = node.with_id(id);
    }

    for child in compound.nested_blocks() {
        node.children.push(child.to_virtual_dom());
    }

    node
}

fn preamble_to_node<'a>(preamble: &'a Preamble<'a>) -> VirtualNode {
    let mut node = VirtualNode::new("div").with_id("preamble");

    for child in preamble.nested_blocks() {
        node.children.push(child.to_virtual_dom());
    }

    node
}

fn break_to_node<'a>(break_: &'a Break<'a>) -> VirtualNode {
    let context = break_.raw_context();

    match context.as_ref() {
        "thematic_break" => VirtualNode::new("hr"),
        "page_break" => VirtualNode::new("div").with_class("page-break"),
        _ => VirtualNode::new("hr"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Parser;

    #[test]
    fn empty_document() {
        let doc = Parser::default().parse("");
        let vdom = doc.to_virtual_dom();

        assert_eq!(vdom.tag, "div");
        assert_eq!(vdom.classes, vec!["document"]);
        assert_eq!(vdom.children.len(), 0);
    }

    #[test]
    fn single_paragraph() {
        let doc = Parser::default().parse("Hello, world!");
        let vdom = doc.to_virtual_dom();

        assert_eq!(vdom.tag, "div");
        assert_eq!(vdom.classes, vec!["document"]);
        assert_eq!(vdom.children.len(), 1);

        let para = &vdom.children[0];
        assert_eq!(para.tag, "p");
        assert_eq!(para.text.as_deref(), Some("Hello, world!"));
    }

    #[test]
    fn unordered_list() {
        let doc = Parser::default().parse("* item 1\n* item 2\n* item 3");
        let vdom = doc.to_virtual_dom();

        assert_eq!(vdom.children.len(), 1);

        let wrapper = &vdom.children[0];
        assert_eq!(wrapper.tag, "div");
        assert!(wrapper.classes.contains(&"ulist".to_string()));
        assert_eq!(wrapper.children.len(), 1);

        let ul = &wrapper.children[0];
        assert_eq!(ul.tag, "ul");
        assert_eq!(ul.children.len(), 3);

        for li in &ul.children {
            assert_eq!(li.tag, "li");
        }
    }

    #[test]
    fn section_with_paragraph() {
        let doc = Parser::default().parse("== Section Title\n\nSome text.");
        let vdom = doc.to_virtual_dom();

        assert_eq!(vdom.children.len(), 1);

        let section = &vdom.children[0];
        assert_eq!(section.tag, "div");
        assert!(section.classes.contains(&"sect1".to_string()));

        assert_eq!(section.children.len(), 2);
        assert_eq!(section.children[0].tag, "h2");
        assert_eq!(section.children[1].tag, "p");
    }

    #[test]
    fn ordered_list_has_arabic_class() {
        let doc = Parser::default().parse(". item 1\n. item 2\n. item 3");
        let vdom = doc.to_virtual_dom();

        assert_eq!(vdom.children.len(), 1);

        let wrapper = &vdom.children[0];
        assert_eq!(wrapper.tag, "div");
        assert!(wrapper.classes.contains(&"olist".to_string()));
        assert_eq!(wrapper.children.len(), 1);

        let ol = &wrapper.children[0];
        assert_eq!(ol.tag, "ol");
        assert_eq!(ol.children.len(), 3);

        for li in &ol.children {
            assert_eq!(li.tag, "li");
            assert!(li.classes.contains(&"arabic".to_string()));
        }
    }
}
