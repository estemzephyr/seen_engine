use crate::stream_module::stream_manager::stream_service;
use std::thread;
use crate::db_handler::DB_Manager::SeenConnection;
use crate::ErrorManager::error_manager::error_service;
use crate::sharding_engine::shard_manager::shard_service;


#[derive(Debug)]
pub(crate) enum Service {
    Default,
    ErrorService(error_service),
    DatabaseService(SeenConnection),
    StreamService(stream_service),
    ShardService(shard_service),
}
impl Service {
    pub async fn create_service_engine(self) -> Self{
        match self {
            Service::ErrorService(err_serv) => {
                let error_manager = error_service::ErrorService().await;
                Service::ErrorService(error_manager)
            }
            Service::DatabaseService(db_services) => {
                let connection = SeenConnection::new_connection(db_services).await;
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
    pub async fn process_data_multithreaded(self) {
        match self {
            Service::ErrorService(_) => {
                let service_thread = thread::spawn(move || {
                    println!("Processing Errors");
                    // Add your ErrorService logic here
                });
                service_thread.join().expect("ErrorService thread panicked");
            }
            Service::DatabaseService(serv) => {
                let service_thread = thread::spawn(move || {
                    tokio::runtime::Runtime::new().unwrap().block_on(async {
                        let datas = serv.perform_database_task().await.expect("TODO: panic message");
                    });
                });

                service_thread.join().expect("DatabaseService thread panicked");
            }
            Service::StreamService(_) => {
                let service_thread = thread::spawn(move || {
                    // Add your StreamService logic here
                });
                service_thread.join().expect("StreamService thread panicked");
            }
            Service::ShardService(_) => {
                let service_thread = thread::spawn(move || {
                    // Add your ShardService logic here
                });
                service_thread.join().expect("ShardService thread panicked");
            }
            _ => {} // Handle other variants if needed
        }
    }
}

#[cfg(test)]
mod tests{

    use crate::MicroServiceHandler::ServiceHandler::Service;
    #[tokio::test]
    async fn test_service_engine(){
        let service = Service::Default;
        let result =service.create_service_engine().await;
        match result {
            Service::Default => assert!(true), // You can customize this based on your logic
            _ => assert!(false, "Unexpected result: {:?}", result),
        }
    }
}