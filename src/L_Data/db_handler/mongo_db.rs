use std::str::FromStr;
use std::time::Duration;
use futures_util::StreamExt;
use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client, error::Error as MongoError, Collection};
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;
use serde_derive::{Deserialize, Serialize};
use tokio::time::sleep;
use crate::L_Business::ErrorManager::errors::IError;
use crate::L_Data::IDataObj::IData::{IData};

#[derive(Deserialize, Serialize)]
pub struct MongoDbConnection {
    pub(crate) username: String,
    pub(crate) password: String,
}
impl MongoDbConnection {
    //Constructor Block
    fn default() -> MongoDbConnection{
        MongoDbConnection{
            username: String::new(),
            password: String::new(),
        }
    }
    async fn create_new_mongodb_conn(&self) -> Result<Client, MongoError> {
        let db_url = format!("mongodb+srv://{}:{}@cluster0.cy2q83g.mongodb.net/?retryWrites=true&w=majority",self.username,self.password).to_string();
        let mut client_options = ClientOptions::parse(db_url).await?;
        client_options.server_api = Some(ServerApi::builder().version(ServerApiVersion::V1).build());

        let client = Client::with_options(client_options)?;
        Ok(client)
    }
    async fn establish_connection(&self) -> Result<Client, MongoError> {
        let max_retries = 5;
        let retry_interval = Duration::from_secs(3);
        println!("MongoDB connection opening...");
        for attempt in 1..=max_retries{
            match self.create_new_mongodb_conn().await {
                Ok(client) => {
                    // Check the connection by running a simple command
                    if let Err(_) = client.list_database_names(None, None).await {
                        eprintln!("Connection is Unavailable \nAttempt {}/{} succeeded", attempt, max_retries);
                        if attempt < max_retries {
                            println!("Retrying in {} seconds...", retry_interval.as_secs());
                            sleep(retry_interval).await;
                        }
                    } else {
                        println!("Connection Available");
                        return Ok(client);
                    }
                }
                Err(err) => {
                    eprintln!("Attempt {}/{} failed: {:?}", attempt, max_retries, err);
                    if attempt < max_retries {
                        println!("Retrying in {} seconds...", retry_interval.as_secs());
                        sleep(retry_interval).await;
                    }
                }
            }
        }

        Err(MongoError::from(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Max retries reached, failed to connect to MongoDB.",
        )))
    }



    pub async fn get_data_from_mongodb(&self) -> Result<Vec<IData>, IError> {
        let conn = MongoDbConnection::establish_connection(&self).await?;
        let collection: Collection<Document> = conn.database("mydb").collection("mycoll");
        // Filter by item
        let filter = doc! {};
        let find_options = FindOptions::builder().build();
        let mut cursor = collection.find(filter, find_options).await?;
        let mut id_counter:i16 = 0;
        let mut data = IData::create_new_data_vec();

        // Iterate through the results
        while let Some(result) = cursor.next().await {
            match result {
                Ok(value_doc) => {
                    match process_document(id_counter, value_doc).await {
                        Ok(mut processed_data) => {
                            processed_data.clean_up_values().await;
                            data.push(processed_data);
                        }
                        Err(err) => {
                            println!("Error processing document: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Error retrieving document: {:?}", err);
                }
            }
            id_counter+=1;
        }
        Ok(data)
    }
}
async fn process_document(mut id_counter: i16, value_doc: Document) -> Result<IData, IError> {
    // U can Customize here for our data ( match db indexes for IDATA)
    // Add IDATA.rs for indexes
    let data = IData {
        id: id_counter,
        name: value_doc.get("name").unwrap().to_string(),
        value: value_doc.get("value").unwrap().to_string(),
    };
    id_counter += 1;
    Ok(data)
}
