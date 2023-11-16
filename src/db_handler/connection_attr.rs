use crate::db_handler::mongo_db::*;
use crate::db_handler::my_sql::*;
use crate::db_handler::postgres::*;
use crate::IDATA::IData::{IData};
use mongodb::bson::oid::ObjectId;

pub enum DatabaseType {
    Mysql,
    Postgres,
    Mongodb,
}
pub struct SeenConnection {
    pub(crate) username:String,
    pub(crate) password:String,
    pub(crate) dbtype:DatabaseType,
}
impl SeenConnection {
    pub async fn new_connection(self) -> SeenConnection{
        SeenConnection{
            username: self.username,
            password: self.password,
            dbtype: self.dbtype,
        }
    }
    pub async fn perform_database_task(self) {
        let def_data = IData::default();
        match self.dbtype{
            DatabaseType::Mysql => {}
            DatabaseType::Postgres => {}
            DatabaseType::Mongodb => {
                let mongodb = MongoDbConnection{
                    username: self.username,
                    password: self.password,
                };
                MongoDbConnection::get_data_from_mongodb(&mongodb,def_data).await.expect("TODO: panic message");
            }
        }
    }
}