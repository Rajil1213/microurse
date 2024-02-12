use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
