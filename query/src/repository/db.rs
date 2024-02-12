use std::{
    collections::HashMap,
    sync::{Arc, RwLock, RwLockWriteGuard},
};

use anyhow::{Context, Result};
use tracing::info;
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
            .cloned()
            .collect::<Vec<PostComment>>()
    }

    pub fn set(&self, post_id: &Uuid, comments: &[Comment]) -> Result<()> {
        let mut post_comments = self.post_comments.write().unwrap();

        Self::find_by_post_id(&post_comments, post_id)?;

        post_comments
            .entry(post_id.to_owned())
            .and_modify(|pc| pc.comments = comments.to_vec());

        Ok(())
    }

    pub fn update(&self, post_id: &Uuid, comment: &Comment) -> Result<()> {
        info!(
            "Updating comment with id: {} for post with id: {}",
            comment.id, post_id
        );
        let mut post_comments = self.post_comments.write().unwrap();

        Self::find_by_post_id(&post_comments, post_id)?;

        post_comments.entry(post_id.to_owned()).and_modify(|pc| {
            pc.comments = pc
                .comments
                .iter()
                .map(|c| {
                    if c.id == comment.id {
                        return comment;
                    }

                    c
                })
                .cloned()
                .collect::<Vec<Comment>>();
        });

        Ok(())
    }

    fn find_by_post_id(
        post_comments: &RwLockWriteGuard<HashMap<Uuid, PostComment>>,
        post_id: &Uuid,
    ) -> Result<()> {
        post_comments
            .get(post_id)
            .with_context(|| format!("Post with id: {post_id:?} not found"))?;

        Ok(())
    }
}
