use reqwest;
use tracing::info;

use crate::constants::{
    COMMENTS_PORT, COMMENTS_URL, MODERATION_PORT, MODERATION_URL, POSTS_PORT, POSTS_URL,
    QUERIES_PORT, QUERIES_URL,
};
use crate::models::event::Event;

#[derive(Debug, Clone)]
pub struct ServiceClient {
    client: reqwest::Client,
    posts_url: String,
    comments_url: String,
    queries_url: String,
    moderation_url: String,
}

impl Default for ServiceClient {
    fn default() -> Self {
        let posts_url = format!("{POSTS_URL}:{POSTS_PORT}/events");
        let comments_url = format!("{COMMENTS_URL}:{COMMENTS_PORT}/events");
        let queries_url = format!("{QUERIES_URL}:{QUERIES_PORT}/events");
        let moderation_url = format!("{MODERATION_URL}:{MODERATION_PORT}/events");

        Self {
            client: reqwest::Client::new(),
            posts_url,
            comments_url,
            queries_url,
            moderation_url,
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
    pub async fn dispatch_to_all(&self, event: &Event) -> Result<(), String> {
        info!("Dispatching to posts client");
        let posts_response = self
            .client
            .post(&self.posts_url)
            .json(event)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        info!("Dispatched posts event successfully: {:?}", posts_response);

        info!("Dispatching to comments client");
        let comments_response = self
            .client
            .post(&self.comments_url)
            .json(event)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        info!(
            "Dispatched comments event successfully: {:?}",
            comments_response
        );

        // queries service can recover from errors by ingesting from the event bus
        // even if this service fails, others should not
        info!("Dispatching to queries client");
        let query_response = match self.client.post(&self.queries_url).json(event).send().await {
            Ok(res) => res
                .text()
                .await
                .map_err(|e| format!("could not parse response from query client: {}", e))?,
            Err(_) => "could not contact queries client".to_string(),
        };

        info!("Dispatched queries event: {}", query_response);

        info!("Dispatching to moderator client");
        let query_response = self
            .client
            .post(&self.moderation_url)
            .json(event)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        info!(
            "Dispatched to moderator event successfully: {:?}",
            query_response
        );

        Ok(())
    }
}
