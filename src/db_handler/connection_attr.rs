use crate::db_handler::mongo_db::MongoDbConnection;
use crate::db_handler::my_sql::MySqlConnection;
use crate::db_handler::postgres::Postgres;

trait DatabaseConnection {
    fn connect(&self);
    fn get_data_from_db(){

    }
}
//MySql Connection
impl DatabaseConnection for MySqlConnection {
    fn connect(&self) {
        println!("MySQL bağlantısı kuruldu.");
    }
}

impl DatabaseConnection for Postgres {
    fn connect(&self) {
        Postgres::connect_postgres();
        println!("U Successfully Connected Postgres");
    }
}

// MongoDB bağlantısı
impl DatabaseConnection for MongoDbConnection {
    fn connect(&self) {
        println!("MongoDB bağlantısı kuruldu.");
    }
}

// Uygulama bileşeni
struct DatabaseHandler<T: DatabaseConnection> {
    connection: T,
}

impl<T: DatabaseConnection> DatabaseHandler<T> {
    fn new(connection: T) -> Self {
        DatabaseHandler { connection }
    }

    fn perform_database_task(&self) {
        // Bağlantı kullanılarak veritabanı işlemleri gerçekleştirilir
        self.connection.connect();
        // Diğer işlemler...
    }
}