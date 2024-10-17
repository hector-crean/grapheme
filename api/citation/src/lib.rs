pub mod reference;
pub mod routes;
use std::net::SocketAddr;

use axum::{
    routing::post, Router,
};

pub struct Server;

impl Default for Server {
    fn default() -> Self {
        Self::new()
    }
}

impl Server {
    pub fn new() -> Self {
        Self
    }

    pub async fn run(&self) {
        // Build our application with routes
        let app = Router::new()
            .route("/format", post(routes::format::post::format_reference))
            .route("/search", post(routes::search::post::handle_search));

        // Run it with hyper on localhost:3001
        let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3001));

        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

        println!("Server running on http://{}", addr);
        axum::serve(listener, app).await.unwrap();
    }
}
