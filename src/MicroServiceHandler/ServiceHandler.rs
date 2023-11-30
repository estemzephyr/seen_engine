use std::thread;
use crate::ErrorManager;
use crate::db_handler::DB_Manager;
use crate::sharding_engine;
use crate::stream_module;

//Architecture idea, not works yet.
pub(crate) struct Service {
    error_service: ErrorManager,
    database_service: DB_Manager,
    stream_service: stream_module,
    shard_service: sharding_engine,
}

impl Service {
    pub fn new(
        error_manager: ErrorManager,
        db_manager: DB_Manager,
        stream_module: stream_module,
        sharding_engine: sharding_engine,
    ) -> Self {
        Service {
            error_service: error_manager,
            database_service: db_manager,
            stream_service: stream_module,
            shard_service: sharding_engine,
        }
    }

    // opening threads
    pub fn process_data_multithreaded(&self, data: &str) {
        let error_service_clone = self.error_service.clone();

        let service_shard_1 = thread::spawn(move || {
            println!("Processing data in Thread 1: {}", data);
            error_service_clone.handle_error();
        });
        let database_service_clone = self.database_service.clone();

        let service_shard_2 = thread::spawn(move || {
            database_service_clone.perform_database_operation();
        });

        service_shard_1.join().expect("Thread 1 panicked");
        service_shard_2.join().expect("Thread 2 panicked");

    }
}