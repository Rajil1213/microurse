use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::repository::db::Db;

pub async fn fetch(State(db): State<Db>) -> Result<impl IntoResponse, ()> {
    Ok((StatusCode::OK, Json(db.fetch())))
}
