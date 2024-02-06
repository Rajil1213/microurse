use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{models::Event, repository::Db};

pub async fn recv_event(
    State(db): State<Db>,
    Json(event): Json<Event>,
) -> Result<impl IntoResponse, ()> {
    match event {
        Event::PostCreated(p) => {
            db.create(&p.id);
            Ok((StatusCode::OK, "Post Event received successfully"))
        }
        Event::CommentCreated(_) => Ok((StatusCode::OK, "Comment Event received successfully")),
    }
}
