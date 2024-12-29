use crate::models::user::{User, Votes};

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn create_user(&self, user: User) -> Result<User, Box<dyn std::error::Error>>;
    async fn get_user(&self, user_name: String)
        -> Result<Option<User>, Box<dyn std::error::Error>>;
    async fn update_user(
        &self,
        user_name: String,
        vote: Votes,
    ) -> Result<(), Box<dyn std::error::Error>>;
    async fn delete_user(&self, user_id: String) -> Result<(), Box<dyn std::error::Error>>;
}
