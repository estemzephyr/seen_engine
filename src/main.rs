use crate::L_Data::db_handler::DB_Manager::{IDATABASE, SeenConnection};
use crate::L_Business::MicroServiceHandler::ServiceHandler::Service;
mod L_Data;
mod L_Business;
mod L_Presentation;

#[tokio::main]
async fn main() {
    let services= Service::Default;
    let conn_mongodb = SeenConnection {
        username: "yusufayd2307".to_string(),
        password: "00fener00".to_string(),
        dbtype: IDATABASE::Mongodb,
    };
    let engine = Service::DatabaseService(conn_mongodb);
    let data_service = engine.create_service_engine().await;
    data_service.multicore_processor().await;
}
