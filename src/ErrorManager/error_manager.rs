use crate::ErrorManager::AuditLogger::AuditLogger;
use crate::ErrorManager::errors::IError;

pub struct error_service {
    i_error: IError,
}
impl error_service{
    pub async fn ErrorService() -> error_service {
        let service = error_service{
            i_error: IError::Default,
        };
        service
    }
}