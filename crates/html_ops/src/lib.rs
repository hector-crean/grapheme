pub mod rc_dom;
pub mod visitor;
pub mod walk;

use crate::{rc_dom::RcDom, visitor::NodeVisitor};
use std::path::Path;

pub fn process_html_file<P: AsRef<Path>, V: NodeVisitor>(
    file_path: P,
    mut visitor: V,
) -> Result<(String, V), std::io::Error> {
    // Read the HTML file
    let dom = RcDom::from_file(file_path)?;

    // Run the visitor
    let (node_handle, _) = visitor.traverse(dom.document);
    let updated_html = node_handle.to_html_string();

    Ok((updated_html, visitor))
}
