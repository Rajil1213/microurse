use axum::{routing::get, Router};
use posts::{
    handlers::{create, fetch},
    repository::Db,
};
use tokio::net::TcpListener;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/posts", get(fetch).post(create))
        .with_state(Db::default());

    const HOST: &str = "0.0.0.0";
    const PORT: usize = 4000;
    let addr: &str = &format!("{}:{}", HOST, PORT);
    let tcp_listener = TcpListener::bind(addr).await.unwrap();

    info!("Starting Posts Service at {}", addr);

    axum::serve(tcp_listener, app).await.unwrap();
}
