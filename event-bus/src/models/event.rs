use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    id: String,
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentEvent {
    pub post_id: Uuid,
    pub comments: Vec<Comment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum Event {
    PostCreated(Post),
    CommentCreated(CommentEvent),
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
        let event = Event::CommentCreated(CommentEvent {
            post_id: uuid::Uuid::from_str("71de1f21-ced1-4d39-a8e3-cd1ce2d12afc").unwrap(),
            comments: vec![
                Comment {
                    id: "71de1f21-ced1-4d39-a8e3-cd1ce2d12afc".to_string(),
                    content: "My First Comment".to_string(),
                },
                Comment {
                    id: "71de1f21-ced1-4d39-a8e3-cd1ce2d12afc".to_string(),
                    content: "My Second Comment".to_string(),
                },
            ],
        });

        let actual: String = serde_json::to_string(&event).unwrap();
        let expected = r#"{"type":"CommentCreated","data":{"post_id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","comments":[{"id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","content":"My First Comment"},{"id":"71de1f21-ced1-4d39-a8e3-cd1ce2d12afc","content":"My Second Comment"}]}}"#;

        assert_eq!(actual, expected);
    }
}
