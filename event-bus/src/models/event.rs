use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    PostCreated(Post),
    CommentCreated(Comment),
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
            Event::CommentCreated(_) => unreachable!("serialized to CommentCreated"),
        }
    }

    #[test]
    fn serializes_correctly() {
        let event = Event::CommentCreated(Comment {
            id: uuid::Uuid::from_str("71de1f21-ced1-4d39-a8e3-cd1ce2d12afc").unwrap(),
            post_id: uuid::Uuid::from_str("71de1f21-ced1-4d39-a8e3-cd1ce2d12afc").unwrap(),
            content: "My First Comment".to_string(),
        });

        let actual: String = serde_json::to_string(&event).unwrap();
        let expected = r#"{"type":"CommentCreated","data":{"id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","post_id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","content":"My First Comment"}}"#;

        assert_eq!(actual, expected);
    }
}
