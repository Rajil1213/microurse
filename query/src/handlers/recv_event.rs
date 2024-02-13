use axum::{extract::State, response::IntoResponse, Json};
use tracing::info;

use crate::{models::Event, repository::Db};

pub async fn recv_event(
    State(db): State<Db>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Received event {:?}", event);

    event.parse(&db)
}
