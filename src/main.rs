use crate::db_handler::DB_Manager;
use crate::db_handler::DB_Manager::{IDATABASE, SeenConnection};
use crate::MicroServiceHandler::ServiceHandler::Service;

mod db_handler;
mod IDataObj;
mod sharding_engine;
mod ErrorManager;
mod MicroServiceHandler;
mod stream_module;

#[tokio::main]
async fn main() {
    // Example Service Build for future
    let services = Service::new(
        ErrorManager::new(),
        DB_Manager::new(),
        stream_module::new(),
        sharding_engine::new(),
    );


    let conn_mongodb = SeenConnection{
        username: "yusufayd2307".to_string(),
        password: "00fener00".to_string(),
        dbtype: IDATABASE::Mongodb,
    };
    let conn = SeenConnection::new_connection(&conn_mongodb).await;
    let datas = conn.perform_database_task().await.expect("TODO: panic message");
    for data in datas{
        println!("{:?}",data)
    }
}
