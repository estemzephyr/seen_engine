use std::fs::{File, OpenOptions};
use std::path::Path;
use std::time::SystemTime;
use crate::db_handler::DB_Manager::SeenConnection;
use crate::ErrorManager::errors::IError;

struct DB_OR_FILE {
    a_file: File,
    db_url: SeenConnection,
}

struct ILogger {
    file_or_db_path: DB_OR_FILE,
    timestamp: u64,
    error_type: IError,
    event: String,
}

impl ILogger {
    pub fn log_event(event: &str) -> Result<Self, String> {
        let mut logger = ILogger {
            file_or_db_path: DB_OR_FILE {
                a_file: File::default(),
                db_url: SeenConnection::default(),
            },
            timestamp: 0,
            error_type: IError::Default,
            event: "".to_string(),
        };

        // Define values here.
        // Not used Constructor Block bcz no needs to create after and after a new logger. Using Dependency Injection.
        logger.file_or_db_path.a_file = match get_or_create_file("s") {
            Ok(file) => file,
            Err(err) => return Err(format!("Error creating or opening file: {}", err)),
        };

        logger.timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| format!("Unknown Time Type: {}", e))?
            .as_secs();

        logger.error_type = IError::Default;
        logger.event = event.to_string();

        Ok(logger)
    }
}

// File Automation
fn get_timestamped_file_name(file_path: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_or_else(|e| {
            eprintln!("Time went backwards: {}", e);
            0
        }, |dur| dur.as_secs());

    format!("{}_{}.txt", file_path, timestamp)
}

pub fn get_or_create_file(file_path: &str) -> Result<File, String> {
    let full_file_path = get_timestamped_file_name(file_path);

    if Path::new(&full_file_path).exists() {
        OpenOptions::new()
            .append(true)
            .open(&full_file_path)
            .map_err(|e| format!("Error opening file: {}", e))
    } else {
        OpenOptions::new()
            .create(true)
            .write(true)
            .open(&full_file_path)
            .map_err(|e| format!("Error creating file: {}", e))
    }
}
