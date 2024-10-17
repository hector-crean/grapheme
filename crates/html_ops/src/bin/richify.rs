use html_ops::{
    process_html_file,
    visitor::{orphan_visitor::OrphanVisitor, rich_text_wrapper_visitor::RichTextWrapperVisitor},
    walk::process_html_files,
};
use log::info;
use std::{fs, io, path::Path};

const SYNC_WTH_DB: bool = true;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let src_dir = Path::new("assets/input");
    let dst_dir = Path::new("assets/output");

    process_html_files(src_dir, |path, relative_path| {
        let (html, visitor) = process_html_file(path, OrphanVisitor::new())?;
        let output_path = dst_dir.join(relative_path);

        match output_path.parent() {
            Some(parent) => fs::create_dir_all(parent)?,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid output path")),
        }

        fs::write(&output_path, html)?;
        info!("Successfully processed file: {:?}", path);

        if SYNC_WTH_DB {
            let text_map = visitor.html_map();

            for (id, html) in text_map {
                info!("Text Map:\n{:#?}", &visitor.html_map());
            }

        }


        Ok(())
    })?;

    Ok(())
}
