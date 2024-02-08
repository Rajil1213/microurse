use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use anyhow::{Context, Result};
use uuid::Uuid;

use crate::models::{Comment, Post, PostComment};

#[derive(Debug, Clone, Default)]
pub struct Db {
    post_comments: Arc<RwLock<HashMap<Uuid, PostComment>>>,
}

impl Db {
    pub fn create(&self, post: &Post) -> Uuid {
        let mut post_comments = self.post_comments.write().unwrap();

        post_comments
            .entry(post.id)
            .or_insert(PostComment::new(post, &[]))
            .id
    }

    pub fn fetch(&self) -> Vec<PostComment> {
        let post_comments = self.post_comments.read().unwrap();

        post_comments
            .values()
            .into_iter()
            .map(|v| v.clone())
            .collect::<Vec<PostComment>>()
    }

    pub fn update(&self, post_id: &Uuid, comments: &[Comment]) -> Result<()> {
        let mut post_comments = self.post_comments.write().unwrap();

        post_comments
            .get(post_id)
            .with_context(|| format!("Post with id: {post_id:?} not found"))?;

        post_comments
            .entry(post_id.to_owned())
            .and_modify(|pc| pc.comments = comments.to_vec());

        Ok(())
    }
}
