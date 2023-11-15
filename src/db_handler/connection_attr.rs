use crate::db_handler::mongo_db::*;
use crate::db_handler::my_sql::*;
use crate::db_handler::postgres::*;
pub enum DatabaseType {
    Mysql,
    Postgres,
    Mongodb,
}
pub struct SeenConnection<T> {
    pub(crate) username:String,
    pub(crate) password:String,
    pub(crate) dbtype:DatabaseType,
    pub(crate) connection:T
}
impl <T>SeenConnection<T> {
    pub async fn new_connection(self) -> SeenConnection<T>{
        SeenConnection{
            username: self.username,
            password: self.password,
            dbtype: self.dbtype,
            connection: self.connection,
        }
    }
    pub async fn perform_database_task(self) {
        match self.dbtype{
            DatabaseType::Mysql => {}
            DatabaseType::Postgres => {}
            DatabaseType::Mongodb => {
                let mongodb = MongoDbConnection{
                    username: self.username,
                    password: self.password,
                };
                MongoDbConnection::get_data_from_mongodb(&mongodb).await.expect("TODO: panic message");

            }
        }
    }
}