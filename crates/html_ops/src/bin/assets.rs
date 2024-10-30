use std::path::Path;
use std::{fs, io};

// ... existing imports ...
use html_ops::rc_dom::RcDom;
use html_ops::visitor::asset_mover_visitor::AssetMoverVisitor;
use html_ops::visitor::NodeVisitor;
use html_ops::walk::process_html_files;

pub fn move_assets<P: AsRef<Path>, Q: AsRef<Path>>(
    directory: P,
    assets_dir: Q,
) -> Result<(), io::Error> {
    // Create assets directory if it doesn't exist
    fs::create_dir_all(&assets_dir)?;

    process_html_files(directory.as_ref(), |path, relative_path| {
        // Parse the HTML file
        let dom = RcDom::from_file(path)?;

        // Create and run the visitor
        let mut visitor =
            AssetMoverVisitor::new(directory.as_ref(), assets_dir.as_ref(), relative_path);
        let (new_dom, _) = visitor.traverse(dom.document);

        // Write the modified HTML back to the file
        let html = new_dom.to_html_string();
        fs::write(path, html)?;

        Ok(())
    })
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let src_dir =
        Path::new(r#"C:\Users\Hector.C\desktop\projects\OTS110_WebApp\src\content\bipolar"#);
    let assets_dir =
        Path::new(r#"C:\Users\Hector.C\desktop\projects\OTS110_WebApp\public\assets\bipolar"#);

    move_assets(src_dir, assets_dir)?;

    Ok(())
}
