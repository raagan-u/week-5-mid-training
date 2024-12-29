pub mod config;
pub mod mongo_crud;
pub mod mongo_user_crud;
pub mod poll_crud;
pub mod user_crud;
use crate::db::{mongo_crud::MongoPollRepo, poll_crud::PollRepository};

use config::DbConfig;
use mongo_user_crud::MongoUserRepo;
use user_crud::UserRepository;

pub async fn init(config: DbConfig) -> impl PollRepository {
    match config.db_type.as_str() {
        "mongodb" => {
            let repo = MongoPollRepo::new(&config).await; // Await the Future to get MongoPollRepo
            repo // Box the resulting MongoPollRepo
        }
        _ => panic!("Unsupported database type"),
    }
}

pub async fn init_user_db(config: DbConfig) -> impl UserRepository {
    match config.db_type.as_str() {
        "mongodb" => {
            let repo = MongoUserRepo::new(&config).await;
            repo
        }
        _ => panic!("Unsupported database type"),
    }
}
