pub mod reference;
pub mod repository;
pub mod routes;
use http::{HeaderName, Method};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use tower_http::cors::{Any, CorsLayer};

use axum::{routing::post, Router};
use log::{error, info};
use repository::ReferenceRepositoryLike;

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("Failed to bind to address: {0}")]
    BindError(#[from] std::io::Error),
    #[error("Server error: {0}")]
    AxumError(#[from] axum::Error),
}

#[derive(Clone)]
pub struct Server {
    db: Arc<Mutex<dyn ReferenceRepositoryLike<Reference = String>>>,
}

impl Server {
    pub fn new(db: impl ReferenceRepositoryLike<Reference = String> + 'static) -> Self {
        let db = Arc::new(Mutex::new(db));
        Self { db }
    }

    pub async fn run(&self, port: u16) -> Result<(), ServerError> {
        let cors = CorsLayer::new()
            // allow `GET` and `POST` when accessing the resource
            .allow_methods([Method::GET, Method::POST])
            // allow the Content-Type header
            .allow_headers([HeaderName::from_static("content-type")])
            // allow requests from any origin
            .allow_origin(Any);

        // Build our application with routes
        let app = Router::new()
            .route("/format", post(routes::format::post::format_reference))
            .route("/search", post(routes::search::post::handle_search))
            .with_state(Self {
                db: self.db.clone(),
            })
            .layer(cors);
        // Run it with hyper on localhost:3001
        let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        info!("Server running on http://{}", addr);
        match axum::serve(listener, app).await {
            Ok(_) => info!("Server shut down gracefully"),
            Err(e) => error!("Server error: {}", e),
        }

        Ok(())
    }
}
