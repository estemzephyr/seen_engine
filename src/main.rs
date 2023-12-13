use crate::app::app_run;

mod L_Data;
mod L_Business;
mod L_Presentation;
mod MicroServiceHandler;

mod app {
    use crate::L_Business::ErrorManager::error_manager::error_service;
    use crate::L_Data::db_handler::DB_Manager::{IDATABASE, SeenConnection};
    use crate::L_Data::sharding_engine::shard_manager::shard_service;
    use crate::L_Presentation::stream_module::stream_manager::stream_service;
    use crate::L_Presentation::webserver::wserver::{IRequest, WServer};
    use crate::MicroServiceHandler::Services::Services;
    //-----------------------------------------------------------------------

    pub async fn app_run() {
        let conn_mongodb = SeenConnection {
            username: "yusufayd2307".to_string(),
            password: "00fener00".to_string(),
            dbtype: IDATABASE::Mongodb,
        };
        let stream = stream_service::create_new_stream().await;
        let err_service = error_service::ErrorService().await;
        let shard_serv = shard_service::ShardEngine().await;

        let db_engine = Services::create_service_engine(Services::DatabaseService(conn_mongodb)).await;
        let stream_engine = Services::create_service_engine(Services::StreamService(stream)).await;
        let shard_engine = Services::create_service_engine(Services::ShardService(shard_serv)).await;

/*
        // Stream servisine gönderme
        stream_engine.send_data(processed_data).await;

        let server_request = IRequest::Get;
        process_req(server_request, &stream_engine).await;*/
    }

    pub async fn process_req(request: IRequest, engine: &Services) {
        // Server Defined Local If want to connect use method WServer::connect_server()
        let server = WServer{ socketaddr: "127.0.0.1".to_string(), port: "8080".to_string() };
        match request {
            IRequest::Get => {
                match engine {
                    Services::StreamService(stream) => {
                        match WServer::start_server(&server).await {
                            Ok(server_handle) => {
                                let server_response = WServer::handle_get_req(&server, IRequest::Get).await;
                                println!("Server Response: {:?}", server_response);
                            }
                            Err(err) => {
                                println!("Error starting server: {:?}", err);
                            }
                        }
                    }
                    _ => {
                        println!("Only Supporting Stream Service pls fix");
                    }
                }
            }
            //Add another Requests
        }
    }
}

#[tokio::main]
async fn main() {
    app_run().await;
}
