use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::info;

use crate::{
    constants::EVENT_BUS_URL,
    models::{event::CommentUpdatedEvent, Event},
    repository::Db,
};

pub async fn recv_event(
    State(db): State<Db>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match event {
        Event::PostCreated(p) => {
            let post_id = db.create(&p.id);
            Ok((
                StatusCode::OK,
                format!("Post Event for {post_id} received successfully"),
            ))
        }
        Event::CommentCreated(_) => Ok((
            StatusCode::OK,
            "CommentCreated Event received successfully".to_string(),
        )),
        Event::CommentModerated(moderated_comment) => match db.moderate_comment(&moderated_comment)
        {
            Ok(comment) => {
                info!("Updated comment with moderated status successfully, dispatching...");
                dispatch_update_event(&CommentUpdatedEvent {
                    post_id: moderated_comment.post_id,
                    comment: comment.clone(),
                })
                .await
                // TODO: rollback on error
                .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

                Ok((
                    StatusCode::OK,
                    format!(
                        "Comment {} moderated successfully",
                        moderated_comment.comment_id
                    ),
                ))
            }
            Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
        },
        Event::CommentUpdated(_) => Ok((
            StatusCode::OK,
            "CommentUpdated event received successfully".to_string(),
        )),
    }
}

async fn dispatch_update_event(event: &CommentUpdatedEvent) -> Result<(), String> {
    info!("Dispatching comment updated event");

    reqwest::Client::new()
        .post(EVENT_BUS_URL)
        .json(&Event::CommentUpdated(event.clone()))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}
