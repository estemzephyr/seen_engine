use crate::L_Business::ErrorManager::error_manager::error_service;
use crate::L_Data::db_handler::DB_Manager::{IDATABASE, SeenConnection};
use crate::L_Business::MicroServiceHandler::ServiceHandler::Service;
use crate::L_Data::sharding_engine::shard_manager::shard_service;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::wserver::{IRequest, local_server, WServer};

mod L_Data;
mod L_Business;
mod L_Presentation;

#[tokio::main]
async fn main() {
    let conn_mongodb = SeenConnection {
        username: "yusufayd2307".to_string(),
        password: "00fener00".to_string(),
        dbtype: IDATABASE::Mongodb,
    };
    let stream = stream_service::create_new_stream().await;
    let err_service = error_service::ErrorService().await;
    let shard_serv = shard_service::ShardEngine().await;
    //-----------------------------------------------------------------------------------
    // Services engines
    let db_engine = Service::create_service_engine(Service::DatabaseService(conn_mongodb)).await;
    let stream_engine = Service::create_service_engine(Service::StreamService(stream)).await;
    let err_engine = Service::create_service_engine(Service::ErrorService(err_service)).await;
    let shard_engine = Service::create_service_engine(Service::ShardService(shard_serv)).await;

    let server = WServer {
        socketaddr: "127.0.0.1".to_string(),
        port: "8080".to_string(),
    };

    server.start_server().await.expect("Server Start Error");
    WServer::handle_get_req(&server, IRequest::Get).await;

}
