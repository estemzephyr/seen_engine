use crate::db_handler::DB_Manager::{IDATABASE, SeenConnection};

mod db_handler;
mod IDataObj;
mod sharding_engine;
mod ErrorManager;

#[tokio::main]
async fn main() {
    let conn_mongodb = SeenConnection{
        username: "yusufayd2307".to_string(),
        password: "00fener00".to_string(),
        dbtype: IDATABASE::Mongodb,
    };
    let conn = SeenConnection::new_connection(&conn_mongodb).await;
    conn.perform_database_task().await;
}
