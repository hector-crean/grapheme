use citation::Server;

#[tokio::main]
async fn main() {
    Server::new().run().await;
}
