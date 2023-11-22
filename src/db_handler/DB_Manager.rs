use crate::db_handler::mongo_db::*;
use crate::ErrorManager::errors::IError;
use crate::IDataObj::IData::IData;
use crate::sharding_engine::Ishard::IShard;
use crate::sharding_engine::IShardController::ControlProtocol;

#[derive(Clone)]
pub enum IDATABASE {
    Mysql,
    Postgres,
    Mongodb,
}
pub struct SeenConnection {
    pub(crate) username:String,
    pub(crate) password:String,
    pub(crate) dbtype:IDATABASE,
}
impl SeenConnection {
    pub async fn new_connection(&self) -> SeenConnection{
        SeenConnection{
            username: self.username.clone(),
            password: self.password.clone(),
            dbtype: self.dbtype.clone(),
        }
    }
    pub async fn perform_database_task(&self) -> Result<IData, IError> {
        let mut data = IData::default();
        match self.dbtype{
            IDATABASE::Mysql => {}
            IDATABASE::Postgres => {}
            IDATABASE::Mongodb => {
                let mongodb = MongoDbConnection{
                    //Cloning for MultiThread Works
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
               data = MongoDbConnection::get_data_from_mongodb(&mongodb).await.expect("TODO: panic message");

            }
        }
        println!("{:?}",data);
        Ok(data)
    }
}