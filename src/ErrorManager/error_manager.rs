use crate::ErrorManager::AuditLogger::AuditLogger;
use crate::ErrorManager::errors::IError;

pub struct error_service {
    logger: AuditLogger,
    i_error: IError,
}
impl error_service{
    pub fn ErrorService(self){

    }
}