use super::NodeVisitor;
use crate::rc_dom::{Handle, Node, NodeData};
use html5ever::{Attribute, LocalName, Namespace, QualName};
use uuid::Uuid;
use std::{cell::RefCell, collections::HashMap};
use markup5ever::{namespace_url, ns};

const RICH_TEXT_TAG: &str = "rich-text";

pub struct OrphanVisitor {
    html_map: HashMap<Uuid, String>,
}

impl OrphanVisitor {
    pub fn new() -> Self {
        OrphanVisitor {
            html_map: HashMap::new(),
        }
    }
    pub fn html_map(&self) -> &HashMap<Uuid, String> {
        &self.html_map
    }
}

impl NodeVisitor for OrphanVisitor {
    fn visit_text(
        &mut self,
        contents: &RefCell<tendril::StrTendril>,
        handle: &Handle,
    ) -> (Option<Handle>, bool) {
        let text = contents.borrow();
        if !text.trim().is_empty() {
            log::info!("Visiting non-empty text node: {:?}", text);

            let sanitized_text = sanitize_text(&text);
            let text_node = create_text_node(&sanitized_text);
            let uuid = uuid::Uuid::new_v4();
            let new_attrs: Vec<Attribute> = vec![Attribute {
                name: QualName::new(None, ns!(), LocalName::from("id")),
                value: uuid.to_string().into(),
            }];
            let rich_text_node = create_element(RICH_TEXT_TAG, new_attrs, vec![text_node]);
            
            // Insert the UUID and inner HTML into the html_map
            self.html_map.insert(uuid, sanitized_text);
            
            (Some(rich_text_node), false)
        } else {
            log::debug!("Skipping empty or whitespace-only text node");
            (None, true)
        }
    }
    fn visit_element(
        &mut self,
        name: &QualName,
        attrs: &RefCell<Vec<Attribute>>,
        template_contents: &RefCell<Option<Handle>>,
        mathml_annotation_xml_integration_point: bool,
        handle: &Handle,
    ) -> (Option<Handle>, bool) {
        let element_name = name.local.as_ref();
        match element_name {
            lists @ ("ul" | "ol") => {
                log::info!("Visiting list element");
              
                let sanitized_children = sanitize_children(&handle.children.borrow());
                let uuid = uuid::Uuid::new_v4();
                let rich_text_attrs: Vec<Attribute> = vec![Attribute {
                    name: QualName::new(None, ns!(), LocalName::from("id")),
                    value: uuid.to_string().into(),
                }];
                let rich_text_node = create_element(RICH_TEXT_TAG, rich_text_attrs, sanitized_children.clone());
                
                // Insert the UUID and inner HTML into the html_map
                let inner_html = rich_text_node.to_html_string();
                self.html_map.insert(uuid, inner_html);
                
                let new_node = create_element(lists, attrs.borrow().clone(), vec![rich_text_node]);
                (Some(new_node), true)
            },
            li @ "li" => {
                log::info!("Visiting list item element");       
                let sanitized_children = sanitize_children(&handle.children.borrow());
                let new_node = create_element(li, attrs.borrow().clone(), sanitized_children);
                (Some(new_node), false)
            },
            paragraph @ "p" => {
                log::info!("Visiting paragraph element");
                let sanitized_children = sanitize_children(&handle.children.borrow());
                let uuid = uuid::Uuid::new_v4();
                let rich_text_attrs: Vec<Attribute> = vec![Attribute {
                    name: QualName::new(None, ns!(), LocalName::from("id")),
                    value: uuid.to_string().into(),
                }];
                let rich_text_node = create_element(RICH_TEXT_TAG, rich_text_attrs, sanitized_children.clone());
               
                // Insert the UUID and inner HTML into the html_map
                let inner_html = rich_text_node.to_html_string();
                self.html_map.insert(uuid, inner_html);
               
                let new_node = create_element(paragraph, attrs.borrow().clone(), vec![rich_text_node]);
                (Some(new_node), false)
            },
            _ => (None, true)
        }
    }
}

// Add this new function to sanitize children nodes
fn sanitize_children(children: &Vec<Handle>) -> Vec<Handle> {
    children.iter().map(|child| {
        match child.data {
            NodeData::Text { ref contents } => {
                let sanitized_text = sanitize_text(&contents.borrow());
                create_text_node(&sanitized_text)
            },
            _ => child.clone(),
        }
    }).collect()
}

/// Creates a new element with the given name, attributes, and children.
fn create_element(name: &str, attrs: Vec<Attribute>, children: Vec<Handle>) -> Handle {
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

fn sanitize_text(text: &str) -> String {
    text.lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>()
        .join(" ")
        .replace('\u{00A0}', " ") // Replace non-breaking spaces with regular spaces
        .replace("  ", " ") // Replace double spaces with single spaces
}
