use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    id: String,
    content: String,
}

impl Comment {
    pub fn new(title: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            content: title.to_string(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}
