use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
}
