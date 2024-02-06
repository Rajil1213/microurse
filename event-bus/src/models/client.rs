use reqwest;
use tracing::info;

use crate::constants::{COMMENTS_PORT, POSTS_PORT, QUERIES_PORT};
use crate::models::event::Event;

#[derive(Debug, Clone)]
pub struct ServiceClient {
    client: reqwest::Client,
    posts_url: String,
    comments_url: String,
    queries_url: String,
}

impl Default for ServiceClient {
    fn default() -> Self {
        let base_url = "http://localhost";
        let posts_url = format!("{base_url}:{POSTS_PORT}");
        let comments_url = format!("{base_url}:{COMMENTS_PORT}");
        let queries_url = format!("{base_url}:{QUERIES_PORT}");

        Self {
            client: reqwest::Client::new(),
            posts_url,
            comments_url,
            queries_url,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ClientType {
    Comment,
    Post,
    Query,
}

impl ServiceClient {
    pub async fn dispatch(
        &self,
        client_type: ClientType,
        event: &Event,
    ) -> Result<reqwest::Response, String> {
        match client_type {
            ClientType::Post => {
                info!("Dispatching to posts client");

                self.client
                    .post(&self.posts_url)
                    .json(event)
                    .send()
                    .await
                    .map_err(|e| e.to_string())
            }
            ClientType::Comment => {
                info!("Dispatching to posts client");

                self.client
                    .post(&self.comments_url)
                    .json(event)
                    .send()
                    .await
                    .map_err(|e| e.to_string())
            }
            ClientType::Query => {
                info!("Dispatching to queries client");

                self.client
                    .post(&self.queries_url)
                    .json(event)
                    .send()
                    .await
                    .map_err(|e| e.to_string())
            }
        }
    }

    pub async fn dispatch_to_all(&self, event: &Event) -> Result<(), String> {
        info!("Dispatching to posts client");
        self.client
            .post(&self.posts_url)
            .json(event)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        info!("Dispatching to comments client");
        self.client
            .post(&self.comments_url)
            .json(event)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        info!("Dispatching to queries client");
        self.client
            .post(&self.queries_url)
            .json(event)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}
