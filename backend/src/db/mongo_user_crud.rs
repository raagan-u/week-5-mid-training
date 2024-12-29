use crate::db::{config::DbConfig, user_crud::UserRepository};
use crate::models::user::{User, Votes};

use mongodb::bson::{self, doc};
use mongodb::{Client, Collection};

#[derive(Clone)]
pub struct MongoUserRepo {
    collection: Collection<User>,
}

impl MongoUserRepo {
    pub async fn new(config: &DbConfig) -> Self {
        // Create a MongoDB client
        let client = Client::with_uri_str(&config.connection_string)
            .await
            .expect("Failed to initialize MongoDB client");

        // Get the specified database and collection
        let database = client.database(&config.database_name);
        let collection = database.collection("users");

        MongoUserRepo { collection }
    }
}

#[async_trait::async_trait]
impl UserRepository for MongoUserRepo {
    async fn create_user(&self, user: User) -> Result<User, Box<dyn std::error::Error>> {
        println!("Entered Create Poll Func");
        println!("{:#?}", user);
        match self.collection.insert_one(user.clone(), None).await {
            Ok(insert_one_result) => {
                println!("Added Poll Successfully {}", insert_one_result.inserted_id)
            }
            Err(e) => {
                eprintln!("Failed adding polls to db {}", e)
            }
        }

        Ok(user)
    }

    async fn get_user(
        &self,
        user_name: String,
    ) -> Result<Option<User>, Box<dyn std::error::Error>> {
        let filter = doc! { "user_name": user_name.clone() };

        match self.collection.find_one(filter, None).await {
            Ok(Some(user)) => Ok(Some(user)),
            Ok(None) => {
                eprintln!("No poll found with user_id: {}", user_name);
                Ok(None)
            }
            Err(e) => {
                eprintln!("Error retrieving user: {:?}", e);
                Err(Box::new(e))
            }
        }
    }
    async fn update_user(
        &self,
        user_name: String,
        vote: Votes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "user_name": user_name };
        let update = doc! {
            "$push": { "votes": bson::to_bson(&vote)? }
        };
        self.collection.update_one(filter, update, None).await?;
        Ok(())
    }
    async fn delete_user(&self, user_id: String) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "user_id": user_id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }
}
