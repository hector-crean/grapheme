use std::path::Path;
use std::{fs, io};

use html5ever::{namespace_url, ns, parse_fragment, QualName};
// ... existing imports ...
use html5ever::tendril::TendrilSink;
use html_ops::rc_dom::RcDom;
use html_ops::visitor::source_attribute_visitor::SourceAttributeVisitor;
use html_ops::visitor::{self, NodeVisitor};
use html_ops::walk::process_html_files;
use log::info;

fn parse_html(html: &str) -> RcDom {
    let parser = parse_fragment(
        RcDom::default(),
        Default::default(),
        QualName::new(None, ns!(html), "div".into()),
        vec![],
    );
    parser.one(html)
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let html = r#"<video><source src="test.mp4"/></video>"#;
    let dom = parse_html(html);

    let mut visitor = SourceAttributeVisitor::new();

    let (new_dom, _) = visitor.traverse(dom.document);

    // Write the modified HTML back to the file
    let html = new_dom.to_html_string();

    info!("{}", html);

    Ok(())
}
