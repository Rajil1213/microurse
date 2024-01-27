use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::repository::Db;

pub async fn fetch(State(db): State<Db>) -> Result<impl IntoResponse, ()> {
    let posts = db.fetch_all();

    Ok((StatusCode::OK, Json(posts)))
}
