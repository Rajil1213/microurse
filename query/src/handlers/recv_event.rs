use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use tracing::info;

use crate::{models::Event, repository::Db};

pub async fn recv_event(
    State(db): State<Db>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Received event {:?}", event);

    match event {
        Event::PostCreated(p) => {
            info!("Creating post with id: {}", p.id);

            let post_id = db.create(&p);
            Ok((
                StatusCode::CREATED,
                format!("Post with id {post_id} created successfully"),
            ))
        }
        Event::CommentCreated(c) => {
            info!("Creating comments for post with id: {}", c.post_id);

            match db.set(&c.post_id, &c.comments) {
                Ok(_) => Ok((
                    StatusCode::OK,
                    format!(
                        "Updated comments on post with id {} successfully",
                        c.post_id
                    ),
                )),
                Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
            }
        }
        Event::CommentUpdated(c) => {
            info!("Updating comments for post with id: {}", c.post_id);

            match db.update(&c.post_id, &c.comment) {
                Ok(_) => Ok((
                    StatusCode::OK,
                    format!(
                        "Updated comments on post with id {} successfully",
                        c.post_id
                    ),
                )),
                Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
            }
        }
        _ => Ok((StatusCode::OK, "Unhandled event ignored".to_string())),
    }
}
