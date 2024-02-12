use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::constants::BLACKLISTED_WORDS;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostEvent {
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
pub struct CommentModeratedEvent {
    pub post_id: Uuid,
    pub comment_id: Uuid,
    pub status: CommentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentUpdatedEvent {
    pub post_id: Uuid,
    pub comment: Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub content: String,
    pub status: CommentStatus,
}

impl Comment {
    pub fn check(&self) -> CommentStatus {
        for blacklisted_word in BLACKLISTED_WORDS {
            if self.content.to_lowercase().contains(blacklisted_word) {
                return CommentStatus::Rejected;
            }
        }
        CommentStatus::Approved
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    PostCreated(PostEvent),
    CommentCreated(CommentCreatedEvent),
    CommentModerated(CommentModeratedEvent),
    CommentUpdated(CommentUpdatedEvent),
}
