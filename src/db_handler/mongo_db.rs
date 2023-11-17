use std::str::FromStr;
use std::time::Duration;
use futures_util::StreamExt;
use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client, error::Error as MongoError, Collection, bson};
use mongodb::bson::{doc, Document};
use mongodb::error::Error;
use mongodb::options::FindOptions;
use serde_derive::{Deserialize, Serialize};
use tokio::time;
use crate::db_handler::errors::IError;
use crate::IData::IData::{IData};

#[derive(Deserialize, Serialize)]
pub struct MongoDbConnection {
    pub(crate) username: String,
    pub(crate) password: String,
}
impl MongoDbConnection {
    pub async fn create_new_mongodb_conn(&self) -> Result<Client, MongoError> {
        const MAX_RETRIES: usize = 3;
        const RETRY_INTERVAL: u64 = 5; // seconds

        for attempt in 1..=MAX_RETRIES {
            let db_url = format!(
                "mongodb+srv://{}:{}@cluster0.cy2q83g.mongodb.net/?retryWrites=true&w=majority",
                self.username.clone(),
                self.password.clone()
            )
                .to_string();
            let mut client_options = ClientOptions::parse(db_url).await?;
            client_options.server_api = Option::from((ServerApi::builder().version(ServerApiVersion::V1).build()));

            match Client::with_options(client_options) {
                Ok(valid_client) => {
                    println!("MongoDB connection is opened");
                    return Ok(valid_client);
                }
                Err(err) => {
                    format!("Error opening MongoDB connection (Attempt {}/{}): {}", attempt, MAX_RETRIES, err);
                }
            }
            // Sleep for the specified interval before the next attempt
            time::sleep(Duration::from_secs(RETRY_INTERVAL)).await;
        }

        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Max connection retries exceeded.",
        )))
    }


    pub async fn get_data_from_mongodb(self) -> Result<Vec<IData>, IError> {
        // Defined Static User for testing
        let conn = MongoDbConnection::create_new_mongodb_conn(&self).await?;
        let collection: Collection<Document> = conn.database("mydb").collection("mycoll");

        // Filter by item
        let filter = doc! {};
        let find_options = FindOptions::builder().build();
        let mut cursor = collection.find(filter, find_options).await?;
        let mut id_counter = 0;
        let mut data_vec = IData::create_new_data_vec();
        // Iterate through the results
        while let Some(result) = cursor.next().await {
            match result {
                Ok(value_doc) => {
                    let data = IData {
                        id: id_counter,
                        name: value_doc.get("name").unwrap().to_string(),
                        value: value_doc.get("value").unwrap().to_string(),
                    };
                    id_counter += 1;
                    data_vec.push(data);
                }
                Err(err) => {
                    eprintln!("Error retrieving document: {}", err);

                }
            }
        }
        Ok(data_vec)
    }
}

