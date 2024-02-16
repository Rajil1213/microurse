use axum::{
    http::Method,
    routing::{get, post},
    Router,
};
use posts::{
    handlers::{create, fetch, recv_event},
    repository::Db,
};
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_headers(Any)
        .allow_origin(Any);

    let app = Router::new()
        .route("/posts/new", post(create))
        .route("/posts", get(fetch))
        .route("/events", post(recv_event))
        .with_state(Db::default())
        .layer(cors);

    const HOST: &str = "0.0.0.0";
    const PORT: usize = 4000;
    let addr: &str = &format!("{}:{}", HOST, PORT);
    let tcp_listener = TcpListener::bind(addr).await.unwrap();

    info!("Starting Posts Service at {}", addr);

    axum::serve(tcp_listener, app).await.unwrap();
}
