use crate::app::app_run;

mod L_Data;
mod L_Business;
mod L_Presentation;
mod MicroServiceHandler;

mod app {
    use crate::L_Business::ErrorManager::error_manager::error_service;
    use crate::L_Data::db_handler::DB_Manager::database_service;
    use crate::L_Data::sharding_engine::shard_manager::shard_service;
    use crate::L_Presentation::stream_module::stream_manager::stream_service;
    use crate::L_Presentation::webserver::ServerEngine::server_service;
    use crate::L_Data::db_handler::IConnection::SeenConnection;
    use crate::L_Data::db_handler::IConnection::IDATABASE;
    use crate::MicroServiceHandler::microservicecontroller::DatabaseServiceEngine;
    //-----------------------------------------------------------------------

    pub async fn app_run() {
        let conn_mongodb = SeenConnection {
            username: "yusufayd2307".to_string(),
            password: "00fener00".to_string(),
            dbtype: IDATABASE::Mongodb,
        };
        let db_service = database_service::start_db_service(conn_mongodb).await;
        let server = server_service::create_new_server();
        let stream = stream_service::create_new_stream().await;
        let err_service = error_service::ErrorService().await;
        let shard_serv = shard_service::ShardEngine().await;
        //--------------------------------------------
        db_service.start_service().await;
        let datas = db_service.get_data().await;
        for data in datas {
            println!("data:{:?}",data)
        }
    }
}

#[tokio::main]
async fn main() {
    app_run().await;
}
