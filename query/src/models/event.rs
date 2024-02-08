use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::post_comments::Comment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
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
    PostCreated(Post),
    CommentCreated(CommentEvent),
}
