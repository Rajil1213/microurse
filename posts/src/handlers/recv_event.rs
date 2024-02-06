use axum::{http::StatusCode, response::IntoResponse};

pub async fn recv_event() -> Result<impl IntoResponse, ()> {
    Ok((StatusCode::OK, "Event Received"))
}
