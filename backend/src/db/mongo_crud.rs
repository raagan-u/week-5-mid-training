use crate::db::{config::DbConfig, poll_crud::PollRepository};
use crate::models::poll::Poll;

use mongodb::bson::doc;
use mongodb::{
    options::UpdateOptions,
    {Client, Collection},
};

#[derive(Clone)]
pub struct MongoPollRepo {
    collection: Collection<Poll>,
}

impl MongoPollRepo {
    pub async fn new(config: &DbConfig) -> Self {
        // Create a MongoDB client
        let client = Client::with_uri_str(&config.connection_string)
            .await
            .expect("Failed to initialize MongoDB client");

        // Get the specified database and collection
        let database = client.database(&config.database_name);
        let collection = database.collection("polls");

        MongoPollRepo { collection }
    }
}

#[async_trait::async_trait]
impl PollRepository for MongoPollRepo {
    async fn create_poll(&self, poll: Poll) -> Result<Poll, Box<dyn std::error::Error>> {
        println!("Entered Create Poll Func");
        let options = poll.options.clone();
        let mut op_id = 1;
        for mut option in options.clone() {
            option.option_id = op_id;
            op_id += 1;
        }
        println!("{:#?}", poll);
        match self.collection.insert_one(poll.clone(), None).await {
            Ok(insert_one_result) => {
                println!("Added Poll Successfully {}", insert_one_result.inserted_id)
            }
            Err(e) => {
                eprintln!("Failed adding polls to db {}", e)
            }
        }

        Ok(poll)
    }

    async fn get_poll(&self, poll_id: i64) -> Result<Option<Poll>, Box<dyn std::error::Error>> {
        println!("Entered get_poll");

        let filter = doc! { "poll_id": poll_id };

        match self.collection.find_one(filter, None).await {
            Ok(Some(poll)) => Ok(Some(poll)),
            Ok(None) => {
                eprintln!("No poll found with poll_id: {}", poll_id);
                Ok(None)
            }
            Err(e) => {
                eprintln!("Error retrieving poll: {:?}", e);
                Err(Box::new(e))
            }
        }
    }

    async fn update_poll(
        &self,
        poll_id: i64,
        target: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "poll_id": poll_id };
        let mut update = doc! {};
        if target == "reset" {
            update = doc! {
                  "$set": { "options.$[].votes": 0 }
            };
        } else {
            update = doc! {
                "$set": { "status": "closed" }
            }
        }

        self.collection.update_one(filter, update, None).await?;
        Ok(()) // Return the updated poll
    }

    async fn delete_poll(&self, poll_id: i64) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "poll_id": poll_id };
        self.collection.delete_one(filter, None).await?;
        Ok(())
    }

    async fn vote_poll(
        &self,
        poll_id: i64,
        option_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let filter = doc! { "poll_id": poll_id, "status": "active" };
        let update = doc! {
            "$inc": { "options.$[elem].votes": 1 }
        };
        let array_filters = vec![doc! { "elem.option_id": option_id }];

        let options = UpdateOptions::builder()
            .array_filters(array_filters)
            .build();

        let result = self.collection.update_one(filter, update, options).await?;

        if result.matched_count == 0 {
            println!("No matching active poll or option found.");
        } else if result.modified_count == 0 {
            println!("Vote not recorded; possibly a conflict.");
        } else {
            println!("Vote successfully recorded!");
        }

        Ok(())
    }
}
