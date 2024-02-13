use core::time;
use std::thread;

use anyhow::Result;
use axum::{http::StatusCode, response::IntoResponse, Json};
use tracing::{error, info};

use crate::{
    constants::EVENT_BUS_URL,
    models::{event::CommentModeratedEvent, CommentStatus, Event},
};

pub async fn recv_event(Json(event): Json<Event>) -> Result<impl IntoResponse, impl IntoResponse> {
    const MODERATOR_THINKING_TIME: u64 = 1000; // 1 sec

    match event {
        Event::CommentCreated(created_comments) => {
            info!("Received CommentCreated Event, starting moderation...");
            // only one comment is allowed to have been created just now at any given time
            for comment in created_comments.comments {
                match comment.status {
                    CommentStatus::Pending => {
                        let status = comment.check();
                        thread::spawn(move || {
                            thread::sleep(time::Duration::from_millis(MODERATOR_THINKING_TIME));

                            dispatch(&CommentModeratedEvent {
                                post_id: created_comments.post_id,
                                status,
                                comment_id: comment.id,
                            })
                            .unwrap_or_else(|e| {
                                error!(
                                    "Could not dispatch moderation event due to: {}",
                                    e.to_string()
                                );
                            });
                        });
                        return Ok((
                            StatusCode::OK,
                            "comment scheduled for moderation".to_string(),
                        ));
                    }
                    _ => continue,
                }
            }
            Ok((
                StatusCode::PRECONDITION_FAILED,
                "No comment left to moderate".to_string(),
            ))
        }
        _ => Ok::<(StatusCode, String), (StatusCode, String)>((
            StatusCode::OK,
            "Unhandled event ignored".to_string(),
        )),
    }
}

fn dispatch(event: &CommentModeratedEvent) -> Result<()> {
    info!("Dispatching comment moderated event");

    let res = reqwest::blocking::Client::new()
        .post(EVENT_BUS_URL)
        .json(&Event::CommentModerated(event.clone()))
        .send()?
        .text()?;

    info!("Dispatched event successfully: {}", res);
    Ok(())
}
