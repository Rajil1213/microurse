use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::models::Post;

#[derive(Clone, Debug, Default)]
pub struct Db {
    pub posts: Arc<RwLock<HashMap<String, Post>>>,
}

impl Db {
    pub fn add_post(&self, title: &str) -> Post {
        let mut posts = self.posts.write().unwrap();
        let new_post = Post::new(title);

        posts
            .entry(new_post.id().to_string())
            .or_insert(new_post)
            .clone()
    }

    pub fn find_by_id(&self, id: &str) -> Option<Post> {
        let posts = self.posts.read().unwrap();

        posts.get(id).cloned()
    }

    pub fn fetch_all(&self) -> Vec<Post> {
        self.posts
            .read()
            .unwrap()
            .values()
            .cloned()
            .collect::<Vec<Post>>()
    }
}
