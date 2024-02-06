use crate::{constants::EVENTS_BUS_URL, models::Event};
use crate::{dtos::NewPostInput, repository::Db};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::info;

pub async fn create(
    State(db): State<Db>,
    Json(post): Json<NewPostInput>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let created_post = db.add_post(post.title.as_str());

    info!("Creating post with id: {}", created_post.id());

    info!("Dispatching event to bus");
    let event = Event::PostCreated(created_post.clone());

    match dispatch_event(&event).await {
        Ok(()) => Ok((StatusCode::CREATED, Json(created_post.clone()))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn dispatch_event(event: &Event) -> Result<(), String> {
    info!("Dispatching event post creation event");

    reqwest::Client::new()
        .post(EVENTS_BUS_URL)
        .json(&event)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
