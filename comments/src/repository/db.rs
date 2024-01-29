use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use uuid::Uuid;

use crate::models::Comment;

#[derive(Clone, Debug, Default)]
pub struct Db {
    pub comments: Arc<RwLock<HashMap<String, Vec<Comment>>>>,
}

impl Db {
    pub fn add_comment(&self, post_id: &Uuid, content: &str) -> Vec<Comment> {
        let mut comments = self.comments.write().unwrap();
        let new_comment = Comment::new(content);

        let comments = comments.entry(post_id.to_string()).or_default();
        comments.push(new_comment.clone());

        comments.clone()
    }

    pub fn fetch_by_post_id(&self, post_id: &Uuid) -> Vec<Comment> {
        let posts = self.comments.read().unwrap();

        // TODO: check if the post exists

        posts
            .get(&post_id.to_string())
            .unwrap_or(&Vec::new())
            .to_vec()
    }
}
