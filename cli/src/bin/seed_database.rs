use html_ops::{
    process_html_file,
    visitor::{orphan_visitor::OrphanVisitor, rich_text_wrapper_visitor::RichTextWrapperVisitor},
    walk::{process_html_files, process_html_files_async},
};
use log::info;
use std::{collections::HashMap, fs, io, path::Path};
use reqwest;
use uuid::Uuid;
use rich_text_api::routes::rich_text::{
    post::{RichTextRequest, RichTextResponse as PostResponse},
};

const SYNC_WITH_DB: bool = true;

async fn sync_with_database(client: &reqwest::Client, base_url: &str, text_map: &HashMap<Uuid, String>) -> color_eyre::Result<()> {
    for (id, html) in text_map {
        info!("Text Map:\n{:#?}", text_map);

        let post_request = RichTextRequest {
            id: *id,
            rich_text: html.to_string(),
        };

        let post_response: PostResponse = client
            .post(format!("{}/rich-text", base_url))
            .json(&post_request)
            .send()
            .await?
            .json()
            .await?;

        info!("POST response: {:#?}", post_response);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let src_dir = Path::new("assets/input");
    let dst_dir = Path::new("assets/output");

    let client = reqwest::Client::new();
    let base_url = "http://127.0.0.1:3001";


    let mut repository: HashMap<Uuid, String> = HashMap::new();

    process_html_files(src_dir, |path, relative_path| {
        let (html, visitor) = process_html_file(path, OrphanVisitor::new())?;
        let output_path = dst_dir.join(relative_path);

        match output_path.parent() {
            Some(parent) => fs::create_dir_all(parent)?,
            None => return Err(io::Error::new(io::ErrorKind::Other, "Invalid output path")),
        }

        let text_map = visitor.html_map();

        for (id, html) in text_map {
            repository.insert(*id, html.clone());
        }
      

        fs::write(&output_path, html)?;
        info!("Successfully processed file: {:?}", path);
        

        Ok(())
    })?;

    if SYNC_WITH_DB {
        sync_with_database(&client, base_url, &repository).await?;
    }




    Ok(())
}
