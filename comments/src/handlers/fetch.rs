use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::repository::Db;
use tracing::info;

pub async fn fetch(State(db): State<Db>, Path(post_id): Path<Uuid>) -> impl IntoResponse {
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let comments = db.fetch_by_post_id(&post_id);

    let stop = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    info!(
        "Fetched {} comments for post {} in {} nanoseconds",
        comments.len(),
        post_id,
        stop - start
    );

    (StatusCode::OK, Json(comments))
}
