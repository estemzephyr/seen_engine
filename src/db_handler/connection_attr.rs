use crate::db_handler::mongo_db::MongoDbConnection;
use crate::db_handler::my_sql::MySqlConnection;
use crate::db_handler::postgres::Postgres;

trait DatabaseConnection {
    fn connect(&self);
}

//MySql Connection
impl DatabaseConnection for MySqlConnection {
    fn connect(&self) {
        let mysql = MySqlConnection {
            username: "".to_string(),
        };

        MySqlConnection::connect_mysql_db(mysql);
    }
}

impl DatabaseConnection for Postgres {
    fn connect(&self) {
        let postgre = Postgres {
            username: "".to_string(),
        };

        Postgres::connect_postgres(postgre);
        println!("U Successfully Connected Postgres");
    }
}

impl DatabaseConnection for MongoDbConnection {
    async fn connect(&self) {
        let mongo = MongoDbConnection {
            username: "".to_string(),
            password: "".to_string(),
        };

        MongoDbConnection::create_new_mongodb_conn(mongo).await.expect("TODO: panic message");
    }
}

struct DatabaseHandler<T: DatabaseConnection> {
    connection: T,
}

impl<T: DatabaseConnection> DatabaseHandler<T> {
    fn new(connection: T) -> Self {
        DatabaseHandler { connection }
    }

    async fn perform_database_task(&self) {
        self.connection.connect();
        match self.connection {
            _ => {}
        }
    }
}