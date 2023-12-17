use crate::app::app_run;

mod L_Data;
mod L_Business;
mod L_Presentation;
mod MicroServiceHandler;

mod app {
    use tokio::sync::mpsc;
    use crate::L_Business::ErrorManager::error_manager::error_service;
    use crate::L_Data::db_handler::DB_Manager::database_service;
    use crate::L_Data::sharding_engine::shard_manager::shard_service;
    use crate::L_Presentation::stream_module::stream_manager::stream_service;
    use crate::L_Presentation::webserver::server_manager::server_service;
    use crate::L_Data::db_handler::IConnection::SeenConnection;
    use crate::L_Data::db_handler::IConnection::IDATABASE;
    use crate::L_Data::IDataObj::IData::IData;
    use crate::L_Data::sharding_engine::Ishard::IShard;
    use crate::L_Presentation::webserver::wserver::IRequest;
    use crate::MicroServiceHandler::DatabaseService::DatabaseServiceEngine;
    use crate::MicroServiceHandler::ServerService::ServerServiceEngine;
    use crate::MicroServiceHandler::StreamService::StreamServiceEngine;
    //-----------------------------------------------------------------------

    pub async fn app_run() {
        let conn_mongodb = SeenConnection {
            username: "yusufayd2307".to_string(),
            password: "00fener00".to_string(),
            dbtype: IDATABASE::Mongodb,
        };
        let db_service = database_service::start_db_service(conn_mongodb).await;
        let server = server_service::create_new_server();
        let mut stream = stream_service::create_new_stream().await;
        let err_service = error_service::ErrorService().await;
        let shard_serv = shard_service::ShardEngine().await;
        //--------------------------------------------
        ServerServiceEngine::start_server(&server).await;
        DatabaseServiceEngine::start_service(&db_service).await;
        ServerServiceEngine::start_server(&server).await;
        DatabaseServiceEngine::start_service(&db_service).await;
        let datas = db_service.get_data().await;
        //Defined a default shard to test
        let mut shard = IShard::default();
        let mut shard_vec = vec![];
        for data in datas {
            shard.ivalue = data;
            shard_vec.push(shard.clone())
        }
        StreamServiceEngine::send_datas(&mut stream, shard_vec).await;
        server_service::process_req(IRequest::Get, server, stream).await;


    }
}

#[tokio::main]
async fn main() {
    app_run().await;
}
