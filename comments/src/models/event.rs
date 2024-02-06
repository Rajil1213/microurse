use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::Comment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostEvent {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentEvent {
    pub post_id: Uuid,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    PostCreated(PostEvent),
    CommentCreated(CommentEvent),
}
