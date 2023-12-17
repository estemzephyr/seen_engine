use async_trait::async_trait;
use crate::L_Data::IDataObj::IData::IData;
use crate::L_Data::sharding_engine::Ishard::IShard;
use crate::L_Data::sharding_engine::IShardController::ControlProtocol;
use crate::L_Presentation::stream_module::stream::IStream;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::server_manager::server_service;
use crate::L_Presentation::webserver::wserver::{IRequest, WServer};
use crate::MicroServiceHandler::ServerService::ServerServiceEngine;

#[async_trait]
pub trait StreamServiceEngine {
    async fn start(&self) -> &IStream;
    async fn send_datas(&mut self, shard: Vec<IShard>);
}

#[async_trait]
impl StreamServiceEngine for stream_service {
    async fn start(&self) -> &IStream {
        &self.stream
    }
    async fn send_datas(&mut self, shard: Vec<IShard>) {
        let listed_shard = ControlProtocol::list_shard_with_algorithm_and_insert(ControlProtocol::Alphabetic, shard).await;

        for shard_data in listed_shard {
            let data = shard_data.ivalue;
            self.stream.sender.send(data).expect("Value sending is errored");
        }
    }
}