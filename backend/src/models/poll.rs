use chrono::{DateTime, Utc};
use futures::Stream;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use std::task::Context;
use tokio::sync::mpsc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PollOption {
    pub option_id: i64,
    pub text: String,
    pub votes: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Poll {
    pub poll_id: i64,
    pub title: String,
    pub creator: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub expiration_date: Option<DateTime<Utc>>,
    pub status: String, // Active, expired, closed
    pub options: Vec<PollOption>,
    pub users_voted: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResultsQuery {
    pub live: bool,
}

pub struct ServerEvents {
    pub events: mpsc::Receiver<String>,
}

impl Stream for ServerEvents {
    type Item = Result<String, std::io::Error>;

    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        match self.events.poll_recv(cx) {
            std::task::Poll::Ready(Some(event)) => std::task::Poll::Ready(Some(Ok(event))),
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
