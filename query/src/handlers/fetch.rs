use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

use crate::repository::db::Db;

#[derive(Deserialize)]
pub struct FetchParams {
    pub post_id: Uuid,
}

pub async fn fetch(
    State(db): State<Db>,
    Query(FetchParams { post_id }): Query<FetchParams>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    info!("Fetching posts with id: {post_id}");

    match db.fetch(&post_id) {
        Ok(post_comments) => Ok((StatusCode::OK, Json(post_comments))),
        Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
    }
}
