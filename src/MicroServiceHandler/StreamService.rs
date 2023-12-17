use async_trait::async_trait;
use crate::L_Data::sharding_engine::Ishard::IShard;
use crate::L_Presentation::stream_module::stream::IStream;
use crate::L_Presentation::stream_module::stream_manager::stream_service;

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

        for shard_data in shard {
            // Taking the value to send to stream , in theoretical we just wanna show names on search bar
            let data = shard_data.ivalue;
            self.stream.sender.send(data).expect("Value sending is errored");
        }
    }
}