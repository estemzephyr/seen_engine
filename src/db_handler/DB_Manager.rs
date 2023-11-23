use crate::db_handler::mongo_db::*;
use crate::ErrorManager::errors::IError;
use crate::IDataObj::IData::IData;

#[derive(Clone)]
pub enum IDATABASE {
    Mysql,
    Postgres,
    Mongodb,
}

pub struct SeenConnection {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) dbtype: IDATABASE,
}

impl SeenConnection {
    pub async fn new_connection(&self) -> SeenConnection {
        SeenConnection {
            username: self.username.clone(),
            password: self.password.clone(),
            dbtype: self.dbtype.clone(),
        }
    }
    pub async fn perform_database_task(&self) -> Result<Vec<IData>, IError> {
        let mut data = IData::create_new_data_vec();
        match self.dbtype {
            IDATABASE::Mysql => {
                // MySQL ile ilgili işlemleri buraya ekleyin
            }
            IDATABASE::Postgres => {
                // Postgres ile ilgili işlemleri buraya ekleyin
            }
            IDATABASE::Mongodb => {
                let mongodb = MongoDbConnection {
                    // Cloning for MultiThread Works
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                data = match MongoDbConnection::get_data_from_mongodb(&mongodb).await {
                    Ok(result) => result,
                    Err(err) => return Err(err.into()), // Hata durumunda geri dön
                };
            }
        }
        //Data Unwrapping for Tests
        // let unwrapped_data = IData::get_datas_on_vec(data.clone()).await;
        for datas in &data {
            //Process Shards Here
            println!("{:?}",datas);
        }

        Ok(data)
    }
}