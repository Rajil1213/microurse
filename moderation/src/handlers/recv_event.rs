use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::info;

use crate::{
    constants::EVENT_BUS_URL,
    models::{event::CommentModeratedEvent, CommentStatus, Event},
};

pub async fn recv_event(Json(event): Json<Event>) -> Result<impl IntoResponse, impl IntoResponse> {
    match event {
        Event::CommentCreated(created_comments) => {
            info!("Received CommentCreated Event, starting moderation...");
            // only one comment is allowed to have been created just now at any given time
            for comment in created_comments.comments {
                match comment.status {
                    CommentStatus::Pending => {
                        let status = comment.check();
                        dispatch(&CommentModeratedEvent {
                            post_id: created_comments.post_id,
                            status,
                            comment_id: comment.id,
                        })
                        .await
                        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

                        return Ok((StatusCode::OK, "comment moderated".to_string()));
                    }
                    _ => continue,
                }
            }
            Ok((
                StatusCode::PRECONDITION_FAILED,
                "No comment left to moderate".to_string(),
            ))
        }
        _ => Ok::<(StatusCode, String), (StatusCode, String)>((
            StatusCode::OK,
            "Unhandled event ignored".to_string(),
        )),
    }
}

async fn dispatch(event: &CommentModeratedEvent) -> Result<()> {
    info!("Dispatching comment moderated event");

    reqwest::Client::new()
        .post(EVENT_BUS_URL)
        .json(event)
        .send()
        .await?;

    Ok(())
}
