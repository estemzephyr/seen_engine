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

    async fn get_data(&self) -> Vec<IData> {
        let mut data_vec = IData::create_new_data_vec();
        let datas =self.connection.perform_database_task().await;
        for data in datas.unwrap(){
            data_vec.push(data)
        }
        data_vec
    }
}
#[cfg(test)]
mod tests {
    use crate::L_Data::db_handler::IConnection::{IDATABASE, SeenConnection};
    use crate::L_Data::db_handler::DB_Manager::database_service;
    use crate::MicroServiceHandler::DatabaseService::DatabaseServiceEngine;

    #[tokio::test]
    async fn test_get_data() {
        let db_service: database_service = {
            let conn_mongodb = SeenConnection {
                username: "yusufayd2307".to_string(),
                password: "00fener00".to_string(),
                dbtype: IDATABASE::Mongodb,
            };
            database_service::start_db_service(conn_mongodb).await
        };


        let datas = db_service.get_data().await;
        assert!(!datas.is_empty());
    }
}