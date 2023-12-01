use crate::db_handler::DB_Manager::{IDATABASE, SeenConnection};
use crate::MicroServiceHandler::ServiceHandler::Service;

mod MicroServiceHandler;
mod db_handler;
mod ErrorManager;
mod IDataObj;
mod sharding_engine;
mod stream_module;

#[tokio::main]
async fn main() {
    let conn_mongodb = SeenConnection {
        username: "yusufayd2307".to_string(),
        password: "00fener00".to_string(),
        dbtype: IDATABASE::Mongodb,
    };
    let engine = Service::DatabaseService(conn_mongodb);
    let services = Service::create_service_engine(engine).await;
    services.process_data_multithreaded();
}
