use async_trait::async_trait;
use crate::L_Data::IDataObj::IData::IData;
use crate::L_Data::sharding_engine::Ishard::IShard;
use crate::L_Data::sharding_engine::IShardController::ControlProtocol;
use crate::L_Presentation::stream_module::stream::IStream;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::ServerEngine::server_service;
use crate::MicroServiceHandler::ServerService::ServerServiceEngine;

#[async_trait]
pub trait StreamServiceEngine{
    async fn start(&self) -> &IStream;
    async fn send_datas_on_stream(&mut self,shard:Vec<IShard>);
    async fn send_datas_to_server(&self,server: server_service);
}
#[async_trait]
impl StreamServiceEngine for stream_service{
    async fn start(&self) -> &IStream {
        &self.stream
    }
    async fn send_datas_on_stream(&mut self, shard:Vec<IShard>){
        //Listing Shards for algorithm
            let listed_shard=ControlProtocol::list_shard_with_algorithm_and_insert(ControlProtocol::Alphabetic, shard).await;
            let mut data = IData::default();
        //Looping listed_shard and sending datas to stream
            for shard_data in listed_shard{
            data = shard_data.ivalue;
            &self.stream.send_data(data);
            }
    }
    async fn send_datas_to_server(&self,server: server_service){

    }
}