use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NewCommentInput {
    pub content: String,
}
