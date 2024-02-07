use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::Event, repository::Db};

pub async fn recv_event(
    State(db): State<Db>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse, ()> {
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
            "Comment Event received successfully".to_string(),
        )),
    }
}
