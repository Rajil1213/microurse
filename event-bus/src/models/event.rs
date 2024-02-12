use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentStatus {
    Pending,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    id: String,
    content: String,
    status: CommentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentCreatedEvent {
    pub post_id: Uuid,
    // TODO: convert to `HashMap<Uuid, Comment>`
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentModeratedEvent {
    pub post_id: Uuid,
    pub comment_id: Uuid,
    pub status: CommentStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentUpdatedEvent {
    pub post_id: Uuid,
    pub comment: Comment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    PostCreated(Post),
    CommentCreated(CommentCreatedEvent),
    CommentModerated(CommentModeratedEvent),
    CommentUpdated(CommentUpdatedEvent),
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;
    use serde_json;

    #[test]
    fn deserializes_correctly() {
        let input = r#"{ "type": "PostCreated", "data": {  "id": "71de1f21-ced1-4d39-a8e3-cd1ce2d12afc", "title": "My First Post" } }"#;
        let data: Event = serde_json::from_str(input).unwrap();

        match data {
            Event::PostCreated(data) => assert_eq!(data.title, "My First Post"),
            _ => unreachable!("serialized to CommentCreated"),
        }
    }

    #[test]
    fn serializes_correctly() {
        let event = Event::CommentCreated(CommentCreatedEvent {
            post_id: uuid::Uuid::from_str("71de1f21-ced1-4d39-a8e3-cd1ce2d12afc").unwrap(),
            comments: vec![
                Comment {
                    id: "71de1f21-ced1-4d39-a8e3-cd1ce2d12afc".to_string(),
                    content: "My First Comment".to_string(),
                    status: CommentStatus::Pending,
                },
                Comment {
                    id: "71de1f21-ced1-4d39-a8e3-cd1ce2d12afc".to_string(),
                    content: "My Second Comment".to_string(),
                    status: CommentStatus::Approved,
                },
            ],
        });

        let actual: String = serde_json::to_string(&event).unwrap();
        let expected = r#"{"type":"CommentCreated","data":{"post_id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","comments":[{"id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","content":"My First Comment","status":"pending"},{"id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","content":"My Second Comment","status":"approved"}]}}"#;

        assert_eq!(actual, expected);
    }
}
