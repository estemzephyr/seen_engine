use crate::L_Business::ErrorManager::errors::IError;
#[derive(Debug)]
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