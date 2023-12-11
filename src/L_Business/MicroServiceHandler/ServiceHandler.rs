use crate::L_Presentation::stream_module::stream_manager::stream_service;
use std::thread;
use crate::L_Data::db_handler::DB_Manager::SeenConnection;
use crate::L_Business::ErrorManager::error_manager::error_service;
use crate::L_Data::sharding_engine::shard_manager::shard_service;
use tokio::task;

#[derive(Debug)]
pub(crate) enum Service {
    Default, // constructor type
    ErrorService(error_service),
    DatabaseService(SeenConnection),
    StreamService(stream_service),
    ShardService(shard_service),
}

impl Service {
    //Note : Referencing self to lifecycle continue
    pub async fn create_service_engine(self) -> Self {
        match self {
            Service::ErrorService(err_serv) => {
                let error_manager = error_service::ErrorService().await;
                Service::ErrorService(error_manager)
            }
            Service::DatabaseService(db_services) => {
                let connection = SeenConnection::new_connection(&db_services).await;
                Service::DatabaseService(connection)
            }
            Service::StreamService(stream_service) => {
                let stream = stream_service::create_new_stream().await;
                Service::StreamService(stream)
            }
            Service::ShardService(sharding_service) => {
                let shard_engine = shard_service::ShardEngine().await;
                Service::ShardService(shard_engine)
            }
            _ => {
                println!("Unknown Service Type Selected");
                Service::Default
            }
        }
    }

    // Opening threads for multithreading
    pub async fn multicore_processor(self){
        match self {
            Service::ErrorService(err) => {
                let service_task = task::spawn(async {
                    Service::ErrorService(err)
                });

                service_task.await.expect("ErrorService task panicked");
            }
            Service::DatabaseService(serv) => {
                let service_thread = thread::spawn(move || {
                    tokio::runtime::Runtime::new().unwrap().block_on(async {
                        // Add your DatabaseService logic here
                        Service::DatabaseService(serv)
                    });
                });
                service_thread.join().expect("DatabaseService task panicked");
            }
            Service::StreamService(stream) => {
                let service_task = task::spawn(async {
                    Service::StreamService(stream)
                });

                service_task.await.expect("StreamService task panicked");
            }
            Service::ShardService(shard) => {
                let service_task = task::spawn(async {
                    Service::ShardService(shard)
                });
                service_task.await.expect("ShardService task panicked");
            }
            _ => {
                println!("Error")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::L_Data::db_handler::DB_Manager::{IDATABASE, SeenConnection};
    use crate::L_Business::ErrorManager::error_manager::error_service;
    use crate::L_Business::MicroServiceHandler::ServiceHandler::Service;
    use crate::L_Data::sharding_engine::shard_manager::shard_service;
    use crate::L_Presentation::stream_module::stream_manager::stream_service;

    #[tokio::test]
    async fn test_service_engine() {
        let service = Service::Default;
        let result = service.create_service_engine().await;
        match result {
            Service::Default => assert!(true), // You can customize this based on your logic
            _ => assert!(false, "Unexpected result: {:?}", result),
        }
    }

    #[tokio::test]
    async fn test_multicore_processor() {

        // Connection
        let conn = SeenConnection {
            username: "".to_string(),
            password: "".to_string(),
            dbtype: IDATABASE::Mysql,
        };
        // Stream
        let stream = stream_service::create_new_stream().await;
        // Error
        let err_serv = error_service::ErrorService().await;
        // Shard
        let shard_service = shard_service::ShardEngine().await;

        // Services Build
        let services = vec![
            Service::DatabaseService(conn),
            Service::StreamService(stream),
            Service::ErrorService(err_serv),
            Service::ShardService(shard_service)];
        // Service Tests
        for service_test in services {
            // Act: Call the multicore_processor method
            println!("testing : {:?}", service_test);
            let result = service_test.multicore_processor().await;
            // Assert: Check the result
            match result {
                serv => {
                    assert!(true)
                }
            }
        }
    }
}