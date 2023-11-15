use std::str::FromStr;
use crate::db_handler::mongo_db::bson::oid::ObjectId;
use futures_util::StreamExt;
use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client, error::Error as MongoError, Collection, bson};
use mongodb::bson::{doc, Document};
use mongodb::error::Error;
use mongodb::options::FindOptions;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct MongoDbConnection{
    pub(crate) username:String,
    pub(crate) password:String,
}
impl MongoDbConnection{
pub async fn create_new_mongodb_conn() -> Result<Client, MongoError> {
    let db_url =
        ("mongodb+srv://yusufayd2307:00fener00@cluster0.cy2q83g.mongodb.net/?retryWrites=true&w=majority")
            .to_string();
    let mut client_options = ClientOptions::parse(db_url)
        .await?;
    client_options.server_api= Option::from((ServerApi::builder().version(ServerApiVersion::V1).build()));

    let client = Client::with_options(client_options)?;
    println!("U connected MongoDB");
    Ok(client)
}

    pub async fn get_data_from_mongodb(&self) -> Result<(), Error> {
        // Defined Static User for testing
        let conn = MongoDbConnection::create_new_mongodb_conn().await?;
        let collection: Collection<Document> = conn.database("mydb").collection("mycoll");

        // Filter by item
        let filter = doc! {};
        let find_options = FindOptions::builder().build();
        let mut cursor = collection.find(filter, find_options).await?;

        // Iterate through the results
        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => {
                    // Process the document here
                    println!("Document: {:?}", document);
                }
                Err(err) => {
                    eprintln!("Error retrieving document: {}", err);
                    // Handle the error as needed
                }
            }
        }

        Ok(())
    }
}

