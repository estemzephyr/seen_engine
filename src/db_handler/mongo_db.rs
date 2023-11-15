use mongodb::{
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
    error::Error as MongoError,
};

pub struct MongoDbConnection{
    pub(crate) username:String,
    pub(crate) password:String,
}
impl MongoDbConnection{
pub async fn create_new_mongodb_conn(self) -> Result<Client, MongoError> {
    let db_url = format!("mongodb+srv://{}:{}@cluster0.cy2q83g.mongodb.net/?retryWrites=true&w=majority", self.username, self.password);
    let mut client_options = ClientOptions::parse(db_url)
        .await?;
    client_options.server_api= Option::from((ServerApi::builder().version(ServerApiVersion::V1).build()));

    let client = Client::with_options(client_options)?;
    println!("U connected MongoDB");
    Ok(client)
}}
