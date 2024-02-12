use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use tracing::info;
use uuid::Uuid;

use crate::models::{Comment, CommentModeratedEvent};

#[derive(Clone, Debug, Default)]
pub struct Db {
    pub comments: Arc<RwLock<HashMap<String, Vec<Comment>>>>,
}

impl Db {
    pub fn create<'u>(&self, post_id: &'u Uuid) -> &'u Uuid {
        info!("Initializing comments for post_id: {post_id:?}");
        let mut comments = self.comments.write().unwrap();
        comments.entry(post_id.to_string()).or_default();

        post_id
    }

    pub fn add_comment(&self, post_id: &Uuid, content: &str) -> Result<Vec<Comment>, String> {
        let mut comments = self.comments.write().unwrap();

        if comments.get(&post_id.to_string()).is_none() {
            return Err(format!("Post with id: {post_id} not found"));
        }

        if content.is_empty() {
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

    pub fn moderate_comment(
        &self,
        moderation_event: &CommentModeratedEvent,
    ) -> Result<Comment, String> {
        info!("Fetching comment by id: {}", moderation_event.comment_id);

        let mut comments = self.comments.write().unwrap();

        if comments
            .get(&moderation_event.post_id.to_string())
            .is_none()
        {
            return Err(format!(
                "Post with id: {} not found, could not moderate",
                moderation_event.post_id
            ));
        }

        let comment_to_moderate = comments
            .get_mut(&moderation_event.post_id.to_string())
            .unwrap()
            .iter_mut()
            .find(|c| c.id == moderation_event.comment_id);

        if comment_to_moderate.is_none() {
            return Err(format!(
                "Comment with id: {} not found in post with id: {}",
                moderation_event.comment_id, moderation_event.post_id
            ));
        }

        let comment_to_moderate = comment_to_moderate.unwrap();
        comment_to_moderate.status = moderation_event.status.clone();

        Ok(comment_to_moderate.clone())
    }
}
