use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::event::Post;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostComment {
    pub id: Uuid,
    pub post: String,
    pub comments: Vec<Comment>,
}

impl PostComment {
    pub fn new(post: &Post, comments: &[Comment]) -> Self {
        Self {
            id: post.id,
            post: post.title.clone(),
            comments: comments.to_vec(),
        }
    }
}
