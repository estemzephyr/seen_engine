use futures_util::StreamExt;
use mongodb::{options::{ClientOptions, ServerApi, ServerApiVersion}, Client, error::Error as MongoError, Collection, bson};
use mongodb::bson::{doc, Document};
use mongodb::options::FindOptions;

pub struct MongoDbConnection{
    pub(crate) username:String,
    pub(crate) password:String,
}
impl MongoDbConnection{
pub async fn create_new_mongodb_conn(&self) -> Result<Client, MongoError> {
    let db_url = format!("mongodb+srv://{}:{}@cluster0.cy2q83g.mongodb.net/?retryWrites=true&w=majority",
                         self.username.clone(), self.password.clone());
    let mut client_options = ClientOptions::parse(db_url)
        .await?;
    client_options.server_api= Option::from((ServerApi::builder().version(ServerApiVersion::V1).build()));

    let client = Client::with_options(client_options)?;
    println!("U connected MongoDB");
    Ok(client)
}
pub async fn get_data_from_mongodb(&self) -> Result<(), mongodb::error::Error> {
    //Defined Static User for testing
    let mongodb = MongoDbConnection{
        username: self.username.clone(),
        password: self.password.clone(),
    };
    let conn = MongoDbConnection::create_new_mongodb_conn(&mongodb).await;
    let collection: Collection<Document> = conn?.database("mydb").collection("users");

    // Filter by item
    let filter = doc! {
            "_id": "example_id",
            "name": "example_name",
        };

    let find_options = FindOptions::builder().build();
    let mut cursor = collection.find(filter, find_options).await?;

    // Process Values on cursor
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                let data = bson::from_document(document)?;
                println!("Found data: {:?}", data);
            }
            Err(e) => println!("Error reading document from MongoDB: {}", e),
        }
    }

    Ok(())
}
}

