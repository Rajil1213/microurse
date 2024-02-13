use anyhow::{Context, Result};
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use std::result::Result as StdResult;
use tracing::{error, info};
use uuid::Uuid;

use crate::{constants::EVENT_BUS_URL, repository::Db};

use super::{post_comments::Comment, CommentStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCreatedEvent {
    pub post_id: Uuid,
    // TODO: convert to `HashMap<Uuid, Comment>`
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentUpdatedEvent {
    pub post_id: Uuid,
    pub comment: Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentModeratedEvent {
    pub post_id: Uuid,
    pub comment_id: Uuid,
    pub status: CommentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    PostCreated(Post),
    CommentCreated(CommentCreatedEvent),
    CommentModerated(CommentModeratedEvent),
    CommentUpdated(CommentUpdatedEvent),
}

impl Event {
    pub async fn load() -> Result<Vec<Event>> {
        let events: Vec<Event> = match reqwest::get(EVENT_BUS_URL).await {
            Ok(res) => res
                .json()
                .await
                .context("failed to parse events from event bus")?,
            Err(e) => {
                format!("could not contact event bus, setting db to empty, {}", e);
                Vec::new()
            }
        };

        info!("Fetched events: {:?}", events);

        Ok(events)
    }

    pub fn parse(&self, db: &Db) -> StdResult<(StatusCode, String), (StatusCode, String)> {
        match self {
            Event::PostCreated(p) => {
                info!("Creating post with id: {}", p.id);

                let post_id = db.create(p);
                Ok((
                    StatusCode::CREATED,
                    format!("Post with id {post_id} created successfully"),
                ))
            }
            Event::CommentCreated(c) => {
                info!("Creating comments for post with id: {}", c.post_id);

                match db.set(&c.post_id, &c.comments) {
                    Ok(_) => Ok((
                        StatusCode::OK,
                        format!(
                            "Created comments on post with id {} successfully",
                            c.post_id
                        ),
                    )),
                    Err(err) => Err((StatusCode::NOT_FOUND, err.to_string())),
                }
            }
            Event::CommentUpdated(c) => {
                info!("Updating comments for post with id: {}", c.post_id);

                match db.update(&c.post_id, &c.comment) {
                    Ok(_) => Ok((
                        StatusCode::OK,
                        format!(
                            "Updated comments on post with id {} successfully",
                            c.post_id
                        ),
                    )),
                    Err(err) => {
                        error!("Comment not found");
                        Err((StatusCode::NOT_FOUND, err.to_string()))
                    }
                }
            }
            _ => Ok((StatusCode::OK, "Unhandled event ignored".to_string())),
        }
    }

    pub fn variant(&self) -> &str {
        match self {
            Event::PostCreated(_) => "PostCreated",
            Event::CommentCreated(_) => "CommentCreated",
            Event::CommentModerated(_) => "CommentModerated",
            Event::CommentUpdated(_) => "CommentUpdated",
        }
    }
}
