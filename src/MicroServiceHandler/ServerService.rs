use async_trait::async_trait;
use crate::L_Presentation::webserver::wserver::{IRequest, WServer};

#[async_trait]
pub trait ServerServiceEngine{
    async fn start_server(&self) -> WServer;
    async fn process_request(&self,req:IRequest){}
}