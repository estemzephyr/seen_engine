use crate::L_Data::db_handler::mongo_db::*;
use crate::L_Business::ErrorManager::errors::IError;
use crate::L_Data::IDataObj::IData::IData;

#[derive(Clone)]
#[derive(Debug)]
pub enum IDATABASE {
    Mysql,
    Postgres,
    Mongodb,
}
#[derive(Debug)]
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
        let mut datas = IData::create_new_data_vec();
        match self.dbtype {
            IDATABASE::Mysql => {
                // MySQL is build soon
            }
            IDATABASE::Postgres => {
                // Postgres is build soon
            }
            IDATABASE::Mongodb => {
                let mongodb = MongoDbConnection {
                    // Cloning for MultiThread Works
                    username: self.username.clone(),
                    password: self.password.clone(),
                };
                datas = match MongoDbConnection::get_data_from_mongodb(&mongodb).await {
                    Ok(result) => result,
                    Err(err) => return Err(err.into()),
                };
            }
        }
        Ok(datas)
    }
}
#[cfg(test)]
mod tests{
    use crate::L_Data::db_handler::IConnection::{IDATABASE, SeenConnection};

    #[tokio::test]
    async fn test_perform_database_task_with_mongodb() {

        // Create a SeenConnection with the mock instances
        let seen_connection = SeenConnection {
            username: "yusufayd2307".to_string(),
            password: "00fener00".to_string(),
            dbtype: IDATABASE::Mongodb,
        };
        // Perform the database task and assert the result
        let result = seen_connection.perform_database_task().await;
        assert!(result.is_ok());
        let datas = result.unwrap();
        for data in datas{
            println!("{:?}",data)
        }
    }
}

