use crate::db_handler::mongo_db::*;
use crate::sharding_engine::Ishard::IShard;
use crate::sharding_engine::IShardController::ControlProtocol;

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
                let def_shard = IShard{
                    key: "some".to_string(),
                    id: 0,
                    ivalue: Vec::new(),
                };
                let data_vec =vec![datas.clone()];
                let mut shard = IShard::new_shard(def_shard).await;
                for idata in data_vec{
                shard.ivalue.clone().push(Some(Box::new(idata)));
                let algorithm = ControlProtocol::list_shard_with_algorithm(ControlProtocol::Alphabetic,shard.clone());
                //println!("{:?}",datas);
            }}
        }
    }
}