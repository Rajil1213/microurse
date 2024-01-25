use std::collections::HashMap;

use crate::models::Post;

#[derive(Clone, Debug, Default)]
pub struct Db {
    pub posts: HashMap<String, Post>,
}

impl Db {
    pub fn add_post<'a>(&mut self, post: &'a Post) -> &'a Post {
        self.posts
            .entry(post.id().to_string())
            .or_insert(post.clone());

        post
    }

    pub fn find_by_id(&self, id: &str) -> Option<&Post> {
        self.posts.get(id)
    }

    pub fn fetch_all(&self) -> Vec<&Post> {
        self.posts.values().collect::<Vec<&Post>>()
    }
}
