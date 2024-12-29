use crate::models::poll::Poll;
#[async_trait::async_trait]
pub trait PollRepository: Send + Sync {
    async fn create_poll(&self, poll: Poll) -> Result<Poll, Box<dyn std::error::Error>>;
    async fn fetch_all(&self) -> Result<Vec<Poll>, Box<dyn std::error::Error>>;
    async fn get_poll(
        &self,
        poll_id: i64,
    ) -> Result<Option<Poll>, Box<dyn std::error::Error + Send + Sync>>;
    async fn update_poll(
        &self,
        poll_id: i64,
        target: String,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_poll(&self, poll_id: i64) -> Result<(), Box<dyn std::error::Error>>;
    async fn vote_poll(
        &self,
        poll_id: i64,
        option_id: i64,
        username: String,
    ) -> Result<(), Box<dyn std::error::Error>>;
}
