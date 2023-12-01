use std::thread;
use crate::db_handler::DB_Manager;
use crate::ErrorManager;
use crate::ErrorManager::AuditLogger::AuditLogger;
use crate::ErrorManager::errors::IError;
use crate::sharding_engine::ShardService;
use crate::stream_module::stream_manager;

// Still building here, I Updating because sometimes i deletes wrongly projects or codes.
// This is just only Architecture idea.
pub(crate) enum Service {
    Default,
    ErrorService(ErrorManager),
    DatabaseService(DB_Manager),
    StreamService(stream_manager),
    ShardService(ShardService),
}

impl Service {
    // Constructor Block
    pub fn default() -> Service {
        Service::Default
    }

    pub async fn create_service_engine(self) -> Service {
        match self {
            Service::ErrorService(err_service) => {
                let error_manager = ErrorManager::error_manager::new_error_service(AuditLogger::default(), IError::Default).await;
                Service::ErrorService(error_manager)
            }
            Service::DatabaseService(db_service) => {
                let seen_connection = DB_Manager::SeenConnection::new_connection(db_service).await;
                Service::DatabaseService(seen_connection)
            }
            Service::StreamService(stream_service) => {
                let stream_manager = stream_manager::stream_manager(stream_service).await;
                Service::StreamService(stream_manager)
            }
            Service::ShardService(sharding_service) => {
                let shard_engine = ShardService::ShardEngine(sharding_service).await;
                Service::ShardService(shard_engine)
            }
            _ => Service::Default,
        }
    }

    // Opening threads for multithreading
    pub fn process_data_multithreaded(&self, data: &str) {
        match self {
            Service::ErrorService(_) => {
                let service_thread = thread::spawn(move || {
                    println!("Processing Errors");
                    // Add your ErrorService logic here
                });
                service_thread.join().expect("ErrorService thread panicked");
            }
            Service::DatabaseService(_) => {
                let service_thread = thread::spawn(move || {
                    // Add your DatabaseService logic here
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
