use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
    pub fn new(title: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            content: title.to_string(),
            status: CommentStatus::Pending,
        }
    }
}
