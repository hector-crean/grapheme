use super::NodeVisitor;
use crate::rc_dom::{Handle, NodeData};
use html5ever::{Attribute, QualName};
use log::{debug, info, trace};
use markup5ever::{local_name, namespace_url, ns};
use std::cell::RefCell;

/// A visitor that moves source URLs from <source> elements to their parent img/video elements.
pub struct SourceAttributeVisitor;

impl Default for SourceAttributeVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl SourceAttributeVisitor {
    pub fn new() -> Self {
        Self
    }

    /// Extracts the src attribute from a source element
    fn extract_source_src(attrs: &[Attribute]) -> Option<String> {
        let src = attrs
            .iter()
            .find(|attr| attr.name.local.as_ref() == "src")
            .map(|attr| attr.value.to_string());

        debug!("Extracted source src: {:?}", src);
        src
    }

    /// Adds or updates the src attribute of an element
    fn update_element_src(element_attrs: &RefCell<Vec<Attribute>>, src: String) {
        let mut attrs = element_attrs.borrow_mut();

        // Try to find existing src attribute
        if let Some(attr) = attrs
            .iter_mut()
            .find(|attr| attr.name.local.as_ref() == "src")
        {
            debug!("Updating existing src from '{}' to '{}'", attr.value, src);
            attr.value = src.into();
        } else {
            debug!("Adding new src attribute: {}", src);
            attrs.push(Attribute {
                name: QualName::new(None, ns!(), local_name!("src")),
                value: src.into(),
            });
        }
    }
}

impl NodeVisitor for SourceAttributeVisitor {
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
            "img" | "video" => {
                info!("Processing {} element", name.local);

                // Look for source elements in children
                let mut children = handle.children.borrow_mut();

                // Collect indices first
                let indices: Vec<_> = children.iter()
                    .enumerate()
                    .filter(|(_, child)| matches!(&child.data, NodeData::Element { name, .. } if name.local.as_ref() == "source"))
                    .map(|(i, _)| i)
                    .collect();

                debug!("Found {} source elements", indices.len());

                // Process first source element and update parent's src
                if let Some(child) = indices.first().and_then(|&i| children.get(i)) {
                    if let NodeData::Element {
                        attrs: ref child_attrs,
                        ..
                    } = child.data
                    {
                        if let Some(src) = Self::extract_source_src(&child_attrs.borrow()) {
                            Self::update_element_src(attrs, src);
                        }
                    }
                }

                // Remove all source elements
                for &index in indices.iter().rev() {
                    children.remove(index);
                }

                children.clear();

                (None, true)
            }
            "source" => {
                debug!("Removing source element");
                (None, false)
            }
            _ => (None, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rc_dom::RcDom;
    use html5ever::parse_fragment;
    use html5ever::tendril::TendrilSink;
    use markup5ever::namespace_url;
    use markup5ever::ns;

    fn parse_html(html: &str) -> RcDom {
        let parser = parse_fragment(
            RcDom::default(),
            Default::default(),
            QualName::new(None, ns!(html), "div".into()),
            vec![],
        );
        parser.one(html)
    }

    #[test]
    fn test_process_img_with_source() {
        let mut visitor = SourceAttributeVisitor::new();
        let html = r#"<img><source src="test.jpg"/></img>"#;
        let dom = parse_html(html);
        let (result, _) = visitor.traverse(dom.document.clone());
        let output = result.to_html_string();
        assert_eq!(output, r#"<img src="test.jpg"></img>"#);
    }

    #[test]
    fn test_process_video_with_source() {
        let mut visitor = SourceAttributeVisitor::new();
        let html = r#"<video><source src="test.mp4"/></video>"#;
        let dom = parse_html(html);
        let (result, _) = visitor.traverse(dom.document.clone());
        let output = result.to_html_string();
        assert_eq!(output, r#"<video src="test.mp4"></video>"#);
    }

    #[test]
    fn test_existing_src_override() {
        let mut visitor = SourceAttributeVisitor::new();
        let html = r#"<img src="old.jpg"><source src="new.jpg"/></img>"#;
        let dom = parse_html(html);
        let (result, _) = visitor.traverse(dom.document.clone());
        let output = result.to_html_string();
        assert_eq!(output, r#"<img src="new.jpg"></img>"#);
    }
}
