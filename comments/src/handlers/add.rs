use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{
    constants::EVENT_BUS_URL,
    dtos::NewCommentInput,
    models::{CommentCreatedEvent, Event},
    repository::Db,
};
use tracing::{error, info};

pub async fn add_comment(
    State(db): State<Db>,
    Path(post_id): Path<Uuid>,
    Json(comment): Json<NewCommentInput>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let comments = db
        .add_comment(&post_id, comment.content.as_str())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    info!("Commenting on post with id: {}", post_id);

    let event = Event::CommentCreated(CommentCreatedEvent {
        post_id,
        comments: comments.clone(),
    });

    match dispatch_event(&event).await {
        Ok(()) => Ok((StatusCode::CREATED, Json(comments))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

pub async fn dispatch_event(event: &Event) -> Result<(), String> {
    info!("Dispatching comment creation event");

    reqwest::Client::new()
        .post(EVENT_BUS_URL)
        .json(event)
        .send()
        .await
        .map_err(|e| {
            error!(
                "Could not dispatch comment creation event due to: {}",
                e.to_string()
            );
            e.to_string()
        })?;

    Ok(())
}
