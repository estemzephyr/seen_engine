use crate::L_Data::db_handler::DB_Manager::database_service;

pub trait MicroServiceController{
    fn start_service(&self){}
}
impl MicroServiceController for database_service{
}