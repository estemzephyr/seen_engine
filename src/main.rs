use crate::db_handler::connection_attr::{DatabaseType, SeenConnection};

mod db_handler;
mod data_control;

#[tokio::main]
async fn main() {
    let conn_mongodb = SeenConnection{
        username: "yusufayd2307".to_string(),
        password: "00fener00".to_string(),
        dbtype: DatabaseType::Mongodb,
        connection: (),
    };
    let conn = SeenConnection::new_connection(conn_mongodb).await;
    let data = conn.perform_database_task().await;
    println!("{:?}",data);
}
