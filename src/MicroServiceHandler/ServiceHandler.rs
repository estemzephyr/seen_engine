use crate::stream_module::stream_manager::stream_service;
use std::thread;
use crate::db_handler::DB_Manager::SeenConnection;
use crate::ErrorManager::error_manager::error_service;
use crate::IDataObj::IData::IData;
use crate::sharding_engine::shard_manager::shard_service;


pub(crate) enum Service {
    Default,
    ErrorService(error_service),
    DatabaseService(SeenConnection),
    StreamService(stream_service),
    ShardService(shard_service),
}

impl Service {
    // Constructor Block
    pub fn default() -> Service {
        Service::Default
    }

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
            _ => Service::Default,
        }
    }

    // Opening threads for multithreading
    pub fn process_data_multithreaded(self) {
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
                        for data in datas {
                            println!("{:?}", data);
                        }
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
