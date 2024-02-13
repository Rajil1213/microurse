use axum::{http::Method, routing::post, Router};
use event_bus::{
    handlers::{events::fetch, recv_event},
    models::state::AppState,
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
        .route("/events", post(recv_event).get(fetch))
        .layer(cors)
        .with_state(AppState::default());

    const HOST: &str = "0.0.0.0";
    const PORT: usize = 4003;
    let addr: &str = &format!("{}:{}", HOST, PORT);
    let tcp_listener = TcpListener::bind(addr).await.unwrap();

    info!("Starting Events Bus at {}", addr);

    axum::serve(tcp_listener, app).await.unwrap();
}
