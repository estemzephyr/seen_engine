use tokio::task;
use crate::L_Business::ErrorManager::error_manager::error_service;
use crate::L_Data::db_handler::DB_Manager::SeenConnection;
use crate::L_Data::sharding_engine::shard_manager::shard_service;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::ServerEngine::server_service;

pub enum Services{
    DatabaseService(SeenConnection),
    StreamService(stream_service),
    ShardService(shard_service),
    ServerService(server_service)
}
impl Services{
    pub async fn create_service_engine(self){
        match self {
            Services::DatabaseService(_conn) => {
                SeenConnection::new_connection(&_conn).await;
            }
            Services::StreamService(stream) => {
                stream_service::create_new_stream().await;
            }
            Services::ShardService(shard) => {
                shard_service::ShardEngine().await;
            }
            Services::ServerService(server) => {
                server_service::connect_server(server);
            }
        }
    }
    pub async fn multicore_processor(self) -> Result<Services, String> {
        let result = match self {
            Services::DatabaseService(serv) => {
                let service_task = task::spawn(async move {
                    // Add your DatabaseService logic here
                    Services::DatabaseService(serv)
                });

                if let Ok(result) = service_task.await {
                    Ok(result)
                } else {
                    // Handle the error as needed
                    Err("Error processing DatabaseService".to_string())
                }
            }
            Services::StreamService(stream) => {
                let service_task = task::spawn(async {
                    Services::StreamService(stream)
                });

                if let Ok(result) = service_task.await {
                    Ok(result)
                } else {
                    // Handle the error as needed
                    Err("Error processing StreamService".to_string())
                }
            }
            Services::ShardService(sharding_serv) => {
                let service_task = task::spawn(async {
                    Services::ShardService(sharding_serv)
                });

                if let Ok(result) = service_task.await {
                    Ok(result)
                } else {
                    // Handle the error as needed
                    Err("Error processing ShardService".to_string())
                }
            }
            Services::ServerService(server_serv) => {
                // Add your ServerService logic here if needed
                Ok(Services::ServerService(server_serv))
            }
        };

        result
    }
}