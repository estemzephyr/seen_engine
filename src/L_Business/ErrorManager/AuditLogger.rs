use std::io::{Result};
use std::path::Path;
use std::time::SystemTime;
use tokio_fs::File;
use tokio_io::{AsyncWriteExt, AsyncReadExt};
// Type Security for File
//--------------------------------
pub struct AuditLogger {
    log_file: SFile,
}

pub struct SFile {
    file: File,
}
// -------------------------------
//File Manager
impl SFile {
    pub async fn new(file_name: String) -> Result<SFile> {
        let file = File::create(file_name).await?;
        Ok(SFile { file })
    }

    pub async fn open_or_create(file_name: String) -> Result<SFile> {
        if Path::new(&file_name).exists() {
            // If File Exists open
            let file = File::open(file_name).await?;
            Ok(SFile { file })
        } else {
            // Create new file
            SFile::new(file_name).await
        }
    }

    pub async fn write(&mut self, data: &[u8]) -> Result<()> {
        self.file.write_all(data).await?;
        Ok(())
    }
    pub async fn read(&mut self, data: &mut Vec<u8>) -> Result<usize> {
        self.file.read(data).await
    }
}
//---------------------------------------------------------
impl AuditLogger {
    pub async fn default() -> Result<AuditLogger> {
        Ok(AuditLogger {
            log_file: SFile::new("default_log.txt".to_string()).await?,
        })
    }
    pub fn build_logger(file: Result<SFile>) -> Result<AuditLogger> {
        Ok(AuditLogger { log_file:file.unwrap()})
    }

    pub async fn log_event(&mut self, event: &str) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Time:")
            .as_secs();

        let log_entry = format!("{}: {}\n", timestamp, event);

        self.log_file.write(log_entry.as_bytes()).await?;
        Ok(())
    }
}