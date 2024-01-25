use crate::models::Post;

#[derive(Clone, Debug, Default)]
pub struct Db {
    pub posts: Vec<Post>,
}

impl Db {
    pub fn add_post<'a>(&mut self, post: &'a Post) -> &'a Post {
        self.posts.push(post.clone());

        post
    }

    pub fn find(&self) -> &[Post] {
        &self.posts[..]
    }
}
