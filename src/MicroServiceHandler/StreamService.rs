use async_trait::async_trait;
use crate::L_Presentation::stream_module::stream::IStream;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
#[async_trait]
pub trait StreamServiceEngine{
    async fn start(&self) -> &IStream;
    async fn send_datas_on_stream(&self);
}
#[async_trait]
impl StreamServiceEngine for stream_service{
    async fn start(&self) -> &IStream {
        &self.stream
    }
    async fn send_datas_on_stream(&self){

    }
}