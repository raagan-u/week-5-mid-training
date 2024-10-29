pub mod mongo_crud;
pub mod poll_crud;

pub mod config;

use crate::db::{mongo_crud::MongoPollRepo, poll_crud::PollRepository};

use config::DbConfig;

pub async fn init(config: DbConfig) -> impl PollRepository {
    match config.db_type.as_str() {
        "mongodb" => {
            let repo = MongoPollRepo::new(&config).await; // Await the Future to get MongoPollRepo
            repo // Box the resulting MongoPollRepo
        }
        _ => panic!("Unsupported database type"),
    }
}
