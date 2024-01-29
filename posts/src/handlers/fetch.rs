use std::time::{SystemTime, UNIX_EPOCH};

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::repository::Db;
use tracing::info;

pub async fn fetch(State(db): State<Db>) -> Result<impl IntoResponse, ()> {
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let posts = db.fetch_all();

    let stop = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    info!(
        "Fetched {} posts in {} nanoseconds",
        posts.len(),
        stop - start
    );

    Ok((StatusCode::OK, Json(posts)))
}
