use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::info;

use crate::models::{event::Event, state::AppState};

pub async fn recv_event(
    State(app_state): State<AppState>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Received event: {:?}", event);

    app_state.db().add(&event);

    match app_state.client().dispatch_to_all(&event).await {
        Ok(()) => Ok((StatusCode::OK, "Event received successfully")),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn fetch(State(app_state): State<AppState>) -> impl IntoResponse {
    info!("Providing all events to the requester");

    Json(app_state.db().fetch())
}
