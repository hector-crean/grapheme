use crate::rc_dom::Handle;
use html5ever::{Attribute, QualName};
use std::cell::RefCell;
use std::collections::HashMap;

use tendril::StrTendril;

use super::NodeVisitor;

pub struct TextCollectorVisitor {
    text_map: HashMap<String, String>,
    current_id: Option<String>,
}

impl Default for TextCollectorVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl TextCollectorVisitor {
    pub fn new() -> Self {
        TextCollectorVisitor {
            text_map: HashMap::new(),
            current_id: None,
        }
    }

    pub fn text_map(&self) -> &HashMap<String, String> {
        &self.text_map
    }
}

impl NodeVisitor for TextCollectorVisitor {
    fn visit_element(
        &mut self,
        name: &QualName,
        attrs: &RefCell<Vec<Attribute>>,
        template_contents: &RefCell<Option<Handle>>,
        mathml_annotation_xml_integration_point: bool,
        handle: &Handle,
    ) -> (Option<Handle>, bool) {
        // Check if the element has an ID attribute
        let id = attrs
            .borrow()
            .iter()
            .find(|attr| attr.name.local.to_string() == "id")
            .map(|attr| attr.value.to_string());

        // Update the current_id
        self.current_id = id;

        (None, true)
    }

    fn visit_text(
        &mut self,
        contents: &RefCell<StrTendril>,
        handle: &Handle,
    ) -> (Option<Handle>, bool) {
        if let Some(id) = &self.current_id {
            let text = contents.borrow().to_string();
            self.text_map
                .entry(id.clone())
                .and_modify(|existing| *existing += &text)
                .or_insert(text);
        }
        (None, true)
    }
}
