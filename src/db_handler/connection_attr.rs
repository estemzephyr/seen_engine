use crate::db_handler::mongo_db::*;
#[derive(Clone)]
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
    pub async fn new_connection(&self) -> SeenConnection{
        SeenConnection{
            username: self.username.clone(),
            password: self.password.clone(),
            dbtype: self.dbtype.clone(),
        }
    }
    pub async fn perform_database_task(&self) {
        match self.dbtype{
            DatabaseType::Mysql => {}
            DatabaseType::Postgres => {}
            DatabaseType::Mongodb => {
                let mongodb = MongoDbConnection{
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                let datas = MongoDbConnection::get_data_from_mongodb(&mongodb).await.expect("TODO: panic message");
                println!("{:?}",datas);
            }
        }
    }
}