use crate::L_Data::db_handler::DB_Manager::{database_service};
use crate::L_Data::db_handler::IConnection::SeenConnection;
use crate::L_Data::sharding_engine::Ishard::IShard;
use crate::L_Data::sharding_engine::shard_manager::shard_service;
use crate::L_Presentation::stream_module::stream::IStream;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::ServerEngine::server_service;
use crate::L_Presentation::webserver::wserver::WServer;

pub enum IService {
    DatabaseService(database_service),
    ShardService(shard_service),
    StreamService(stream_service),
    ServerService(server_service),
}
impl IService{
    pub async fn start_db_service(conn:SeenConnection) -> database_service{
        database_service{
            connection: conn,
        }
    }
    pub async fn start_shard_service() -> shard_service{
        shard_service{
            shard: IShard::default(),
        }
    }
    pub async fn start_stream_service(_stream:IStream) -> stream_service{
        stream_service{
            stream:_stream,
        }
    }
    pub async fn start_server_service(_server:WServer) -> server_service{
        server_service{
            server: _server ,
        }
    }
}