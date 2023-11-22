use crate::db_handler::mongo_db::*;
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
    pub async fn perform_database_task(&self) {
        match self.dbtype{
            IDATABASE::Mysql => {}
            IDATABASE::Postgres => {}
            IDATABASE::Mongodb => {
                let mongodb = MongoDbConnection{
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                let datas = MongoDbConnection::get_data_from_mongodb(&mongodb).await.expect("TODO: panic message");
                let def_shard = IShard{
                    key: "some".to_string(),
                    id: 0,
                    ivalue: Vec::new(),
                };
                let mut shard = IShard::new_shard(def_shard).await;
                shard.ivalue.push(Some(Box::new(datas)));
                let algorithm = ControlProtocol::list_shard_with_algorithm(
                    ControlProtocol::Alphabetic,
                    shard.clone(),
                );
            }
        }
    }
}