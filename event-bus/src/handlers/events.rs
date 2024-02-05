use axum::Json;
use tracing::info;

use crate::models::event::Event;

pub async fn recv_event(Json(event): Json<Event>) {
    info!("Received event: {:?}", event);
    dispatch_event(&event).await;
}

async fn dispatch_event(event: &Event) {
    info!("Dispatching event: {:?}", event);
}
