use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct NewPostInput {
    pub title: String,
}
