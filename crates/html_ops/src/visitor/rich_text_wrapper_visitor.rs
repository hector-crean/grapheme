use crate::rc_dom::{Handle, NodeData};
use html5ever::{Attribute, QualName};
use log::{debug, info, trace};
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use tendril::StrTendril;
use uuid::Uuid;
use html5ever::namespace_url;

use super::NodeVisitor;
use crate::rc_dom::Node;

/// A visitor that wraps text content in rich-text elements and maintains a map of their IDs to content.
pub struct RichTextWrapperVisitor {
    /// Maps unique IDs to the content of rich-text elements.
    content_map: HashMap<String, String>,
    /// Set of HTML elements considered as root elements for rich-text wrapping.
    root_elements: HashSet<String>,
}

impl Default for RichTextWrapperVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl RichTextWrapperVisitor {
    /// Creates a new RichTextWrapperVisitor with predefined root elements.
    pub fn new() -> Self {
        info!("Creating new RichTextWrapperVisitor");
        let root_elements: HashSet<String> = [
            "p", 
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect();

        Self {
            content_map: HashMap::new(),
            root_elements,
        }
    }

    /// Returns a reference to the content map.
    pub fn content_map(&self) -> &HashMap<String, String> {
        &self.content_map
    }

    /// Checks if the given element name is considered a root element.
    fn is_root_element(&self, name: &str) -> bool {
        self.root_elements.contains(name)
    }

    /// Extracts the content of child nodes, including text and element names.
    fn extract_children_content(&self, children: &[Handle]) -> String {
        children
            .iter()
            .map(|child| match child.data {
                NodeData::Text { ref contents } => contents.borrow().to_string(),
                NodeData::Element { ref name, .. } => format!("<{}>", name.local),
                _ => String::new(),
            })
            .collect::<Vec<String>>()
            .join("")
    }
}

impl NodeVisitor for RichTextWrapperVisitor {
    fn visit_element(
        &mut self,
        name: &QualName,
        attrs: &RefCell<Vec<Attribute>>,
        template_contents: &RefCell<Option<Handle>>,
        mathml_annotation_xml_integration_point: bool,
        handle: &Handle,
    ) -> (Option<Handle>, bool) {
        trace!("Visiting element: {:?}", name);

        match name.local.as_ref() {
            "rich-text" => {
                debug!("Skipping existing rich-text element");
                (None, false)
            }
            element if self.is_root_element(element) => {
                info!("Processing root element: {:?}", name);
                let unique_id = Uuid::new_v4().to_string();
                let children_content = self.extract_children_content(&handle.children.borrow());

                let rich_text_node = create_element(
                    "rich-text",
                    vec![("id", unique_id.clone())],
                    vec![handle.clone()],
                );

                self.content_map.insert(unique_id, children_content);

                (Some(rich_text_node), false)
            }
            _ => (None, false)  // Changed from (None, true) to allow visiting children
        }
    }

    fn visit_text(
        &mut self,
        contents: &RefCell<StrTendril>,
        _handle: &Handle,
    ) -> (Option<Handle>, bool) {
        // let text_content = contents.borrow().to_string();
        // let sanitized_content = sanitize_text(&text_content);
        
        // if !sanitized_content.is_empty() {
        //     let unique_id = Uuid::new_v4().to_string();
        //     let rich_text_node = create_rich_text_element(&unique_id, &sanitized_content);
            
        //     self.content_map.insert(unique_id, sanitized_content);
            
        //     (Some(rich_text_node), false)
        // } else {
        //     (None, false)
        // }
        (None, true)

    }
}


/// Creates a new rich-text element with the given ID and text content.
fn create_rich_text_element(id: &str, content: &str) -> Handle {
    create_element(
        "rich-text",
        vec![("id", id.to_string())],
        vec![create_text_node(content)],
    )
}

/// Creates a new element with the given name, attributes, and children.
fn create_element(name: &str, attrs: Vec<(&str, String)>, children: Vec<Handle>) -> Handle {
    let attrs = attrs
        .into_iter()
        .map(|(name, value)| html5ever::Attribute {
            name: html5ever::QualName::new(
                None,
                html5ever::ns!(),
                html5ever::LocalName::from(name),
            ),
            value: value.into(),
        })
        .collect::<Vec<Attribute>>();

    let element = NodeData::Element {
        name: html5ever::QualName::new(
            None,
            html5ever::ns!(),
            html5ever::LocalName::from(name),
        ),
        attrs: RefCell::new(attrs),
        template_contents: RefCell::new(None),
        mathml_annotation_xml_integration_point: false,
    };
    let handle = Node::new(element);

    *handle.children.borrow_mut() = children;
    handle
}

/// Creates a new text node with the given content.
fn create_text_node(content: &str) -> Handle {
    Node::new(NodeData::Text {
        contents: RefCell::new(content.into()),
    })
}

/// Sanitizes the given text by trimming lines, removing empty lines, and normalizing spaces.
fn sanitize_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
        .replace('\u{00A0}', " ") // Replace non-breaking spaces with regular spaces
        .replace("  ", " ") // Replace double spaces with single spaces
}

#[cfg(test)]
mod tests {
    use super::*;
    use html5ever::parse_fragment;
    use html5ever::tendril::TendrilSink;
    use crate::rc_dom::RcDom;
    use markup5ever::ns;

    fn parse_html(html: &str) -> RcDom {
        let parser = parse_fragment(RcDom::default(), Default::default(), QualName::new(None, ns!(html), "div".into()), vec![]);
        parser.one(html)
    }

    #[test]
    fn test_new_visitor() {
        let visitor = RichTextWrapperVisitor::new();
        assert!(visitor.root_elements.contains("p"));
        assert!(visitor.content_map.is_empty());
    }

    #[test]
    fn test_is_root_element() {
        let visitor = RichTextWrapperVisitor::new();
        assert!(visitor.is_root_element("p"));
        assert!(!visitor.is_root_element("div"));
    }

    #[test]
    fn test_extract_children_content() {
        let visitor = RichTextWrapperVisitor::new();
        let html = "<p>Hello <strong>world</strong>!</p>";
        let dom = parse_html(html);
        let children = dom.document.children.borrow();
        let content = visitor.extract_children_content(&children);
        assert_eq!(content, "Hello <strong>!");
    }

    #[test]
    fn test_sanitize_text() {
        let input = "  Hello,\n  world!  \n\n  How are you?  ";
        let expected = "Hello, world! How are you?";
        assert_eq!(sanitize_text(input), expected);
    }

    #[test]
    fn test_process_non_root_element() {
        let mut visitor = RichTextWrapperVisitor::new();
        let html = "<div><p>Hello, world!</p><p>This is a test.</p></div>";
        let dom = parse_html(html);
        let (result, _) = visitor.traverse(dom.document.clone());
        let output = result.to_html_string();
        assert_eq!(output, "<div><rich-text id=\"1\"><p>Hello, world!</p></rich-text><rich-text id=\"2\"><p>This is a test.</p></rich-text></div>");
        assert_eq!(visitor.content_map.len(), 2);
    }
}
