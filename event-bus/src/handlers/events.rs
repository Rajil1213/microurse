use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::info;

use crate::models::{event::Event, ServiceClient};

pub async fn recv_event(
    State(client): State<ServiceClient>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Received event: {:?}", event);

    match client.dispatch_to_all(&event).await {
        Ok(()) => Ok((StatusCode::OK, "Event received successfully")),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}
