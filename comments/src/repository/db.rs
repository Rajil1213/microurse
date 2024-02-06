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
    pub fn create(&self, post_id: &Uuid) -> Vec<Comment> {
        let mut comments = self.comments.write().unwrap();
        let comments = comments.entry(post_id.to_string()).or_default();

        comments.clone()
    }

    pub fn add_comment(&self, post_id: &Uuid, content: &str) -> Result<Vec<Comment>, String> {
        let mut comments = self.comments.write().unwrap();

        if comments.get(&post_id.to_string()).is_none() {
            return Err(format!("Post with id: {post_id} not found"));
        }

        if !content.is_empty() {
            return Err("Comment cannot be empty".to_string());
        }

        let new_comment = Comment::new(content);

        let comments = comments.entry(post_id.to_string()).or_default();
        comments.push(new_comment);

        Ok(comments.clone())
    }

    pub fn fetch_by_post_id(&self, post_id: &Uuid) -> Result<Vec<Comment>, String> {
        let posts = self.comments.read().unwrap();

        if posts.get(&post_id.to_string()).is_none() {
            return Err(format!("Post with id: {post_id} not found"));
        }

        Ok(posts
            .get(&post_id.to_string())
            .unwrap_or(&Vec::new())
            .to_vec())
    }
}
