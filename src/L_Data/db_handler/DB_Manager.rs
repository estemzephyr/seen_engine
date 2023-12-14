use crate::L_Data::db_handler::IConnection::SeenConnection;

pub struct database_service{
    pub(crate) connection:SeenConnection
}
impl database_service{
    pub async fn start_db_service(conn:SeenConnection) -> database_service{
        database_service{
            connection: conn,
        }
    }
}
