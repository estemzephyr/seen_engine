use crate::stream_module::stream_manager::stream_service;
use std::thread;
use crate::db_handler::DB_Manager;
use crate::db_handler::DB_Manager::SeenConnection;
use crate::ErrorManager::error_manager::error_service;
use crate::sharding_engine::ShardService;

// Still building here, I Updating because sometimes i deletes wrongly projects or codes.
// This is just only Architecture idea.
pub(crate) enum Service {
    Default,
    ErrorService(error_service),
    DatabaseService(SeenConnection),
    StreamService(stream_service),
    ShardService(ShardService),
}

impl Service {
    // Constructor Block
    pub fn default() -> Service {
        Service::Default
    }

    pub async fn create_service_engine(self) {
        match self {
            Service::ErrorService(err_serv) => {
                let error_manager = error_service::ErrorService(err_serv);
            }
            Service::DatabaseService(db_services) => {
                let seen_connection = SeenConnection::new_connection(db_services).await;
                Service::DatabaseService(seen_connection)
            }
            Service::StreamService(stream_service) => {
            }
            Service::ShardService(sharding_service) => {}
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
