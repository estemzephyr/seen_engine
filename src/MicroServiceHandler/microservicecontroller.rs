use async_trait::async_trait;
use crate::L_Data::db_handler::DB_Manager::database_service;
use crate::L_Data::db_handler::IConnection::SeenConnection;
use crate::L_Data::IDataObj::IData::IData;

#[async_trait]
pub trait DatabaseServiceEngine {
    async fn start_service(&self) -> SeenConnection;
    async fn get_data(&self) -> Vec<IData>;
}

#[async_trait]
impl DatabaseServiceEngine for database_service {
    async fn start_service(&self) -> SeenConnection {
        self.connection.new_connection().await
    }

    async fn get_data(&self) ->Vec<IData>{
        let datas = self.connection.perform_database_task().await.expect("TODO: panic message");
        datas
    }
}