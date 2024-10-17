pub mod repository;
pub mod rich_text;
pub mod routes;
use std::sync::Arc;
use std::{net::SocketAddr, sync::Mutex};
use tower::{ServiceBuilder, ServiceExt, Service};
use tower_http::cors::{Any, CorsLayer};
use std::convert::Infallible;
use http::{HeaderName, Method};




use axum::{
    routing::{get, post},
    Router,
};

use repository::RichTextRepositoryLike;

use log::{error, info};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Failed to bind to address: {0}")]
    BindError(#[from] std::io::Error),
    #[error("Server error: {0}")]
    AxumError(#[from] axum::Error),
}

#[derive(Clone)]
pub struct Server {
    db: Arc<Mutex<dyn RichTextRepositoryLike<RichText = String>>>,
}

impl Server {
    pub fn new(db: impl RichTextRepositoryLike<RichText = String> + 'static) -> Self {
        let db = Arc::new(Mutex::new(db));
        Self { db }
    }

    pub async fn run(&self, port: u16) -> Result<(), ServerError> {
        info!("Server running on port: {}", port);

        let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
         // allow the Content-Type header
         .allow_headers([HeaderName::from_static("content-type")])
        // allow requests from any origin
        .allow_origin(Any);
    


        // Build our application with routes
        let app = Router::new()
            .route(
                "/rich-text",
                post(routes::rich_text::post::handle_post_rich_text),
            )
            .route(
                "/rich-text/:id",
                get(routes::rich_text::get::handle_get_rich_text),
            )
            .route(
                "/rich-text",
                get(routes::rich_text::get::handle_list_rich_text),
            )
            .with_state(Self {
                db: self.db.clone(),
            }).layer(cors);

        // Run it with hyper on localhost:3001
        let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

        let listener = tokio::net::TcpListener::bind(addr).await?;

        info!("Server running on http://{}", addr);
        match axum::serve(listener, app).await {
            Ok(_) => info!("Server shut down gracefully"),
            Err(e) => error!("Server error: {}", e),
        }

        Ok(())
    }
}
