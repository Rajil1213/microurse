use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{dtos::NewCommentInput, repository::Db};
use tracing::info;

pub async fn add_comment(
    State(db): State<Db>,
    Path(post_id): Path<Uuid>,
    Json(comment): Json<NewCommentInput>,
) -> Result<impl IntoResponse, ()> {
    let comments = db.add_comment(&post_id, comment.content.as_str());

    info!("Commenting on post with id: {}", post_id);
    Ok((StatusCode::CREATED, Json(comments)))
}
