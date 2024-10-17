//! # Rich Text CLI
//!
//! This CLI provides two main commands:
//!
//! - `rich-text-server`: Starts the rich text server on a specified port
//! - `process-html`: Processes HTML files in a source directory and outputs to a destination directory
//!
//! ## Usage
//!
//! To run the CLI, use the following command structure:
//!
//! ```
//! cargo run -- <COMMAND> [ARGS]
//! ```
//!
//! ### Examples:
//!
//! 1. Start the rich text server on port 8080:
//!    ```
//!    cargo run -- rich-text-server 8080
//!    ```
//!
//! 2. Process HTML files:
//!    ```
//!    cargo run -- process-html --src-dir /path/to/source --dst-dir /path/to/destination --api-endpoint http://localhost:8080/rich-text
//!    ```
//!
//! ## Commands
//!
//! ### rich-text-server
//!
//! Starts the rich text server on the specified port.
//!
//! Usage:
//! ```
//! cargo run -- rich-text-server <PORT>
//! ```
//!
//! ### process-html
//!
//! Processes HTML files in the source directory and outputs the results to the destination directory.
//!
//! Usage:
//! ```
//! cargo run -- process-html --src-dir <SOURCE_DIRECTORY> --dst-dir <DESTINATION_DIRECTORY> --api-endpoint <API_ENDPOINT>
//! ```

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use color_eyre::eyre;
use log::info;
use rich_text_api::repository::hashmap::HashMapRepository;

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Starts the rich text server on the specified port
    RichTextServer {
        /// The port number to run the server on
        port: u16,
    },
    /// Processes HTML files from the source directory and outputs to the destination directory
    ProcessHtml {
        /// The source directory containing HTML files to process
        #[arg(short, long)]
        src_dir: PathBuf,
        /// The destination directory for processed HTML files
        #[arg(short, long)]
        dst_dir: PathBuf,
        /// The API endpoint to use for the rich text database
        #[arg(short, long)]
        api_endpoint: String,
    },
    SeedDatabase {
        /// The source directory containing HTML files to process
        #[arg(short, long)]
        src_dir: PathBuf,
        /// The destination directory for processed HTML files
        #[arg(short, long)]
        dst_dir: PathBuf,
        /// The API endpoint to use for the rich text database
        #[arg(short, long)]
        api_endpoint: String,
    },
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    dotenv::dotenv().ok();
    env_logger::init();

    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Commands::RichTextServer { port } => {
            let server = rich_text_api::Server::new(HashMapRepository::new());
            server.run(*port).await?;
        }
        Commands::ProcessHtml {
            src_dir,
            dst_dir,
            api_endpoint,
        } => {
            info!("Processing HTML files in directory: {:?}", src_dir);
            
        }
        Commands::SeedDatabase {
            src_dir,
            dst_dir,
            api_endpoint,
        } => {
           
        }
    }

    Ok(())
}
