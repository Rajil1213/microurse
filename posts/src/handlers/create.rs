use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{dtos::NewPostInput, repository::Db};

pub async fn create(
    State(db): State<Db>,
    Json(post): Json<NewPostInput>,
) -> Result<impl IntoResponse, ()> {
    let created_post = db.add_post(post.title.as_str());

    Ok((StatusCode::CREATED, Json(created_post.clone())))
}
