use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::{models::Comment, repository::Db};
use tracing::info;

pub async fn fetch(
    State(db): State<Db>,
    Path(post_id): Path<Uuid>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let comments = db
        .fetch_by_post_id(&post_id)
        .map_err(|e| (StatusCode::NOT_FOUND, e))?;

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

    Ok::<(StatusCode, Json<Vec<Comment>>), (StatusCode, String)>((StatusCode::OK, Json(comments)))
}
