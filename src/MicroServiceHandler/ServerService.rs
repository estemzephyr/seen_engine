
use async_trait::async_trait;
use crate::L_Data::IDataObj::IData::IData;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::server_manager::server_service;
use crate::L_Presentation::webserver::wserver::{IRequest, WServer};

#[async_trait]
pub trait ServerServiceEngine {
    async fn start_server(&self) -> &WServer;
    async fn process_req(request: IRequest, engine: server_service, stream_service: stream_service) -> Option<IData>;
}

#[async_trait]
impl ServerServiceEngine for server_service {
    async fn start_server(&self) -> &WServer {
        &self.server
    }
    async fn process_req(request: IRequest, engine: server_service, mut stream_service: stream_service) -> Option<IData> {
        // Server Defined Local If want to connect use method WServer::connect_server()
        engine.connect_server();
        match request {
            IRequest::Get => {
                    if let Some(data) = stream_service.stream.receiver.recv().await {
                    println!("data on stream: {:?}", data);
                    Some(data)
                } else {
                    println!("No data received on stream");
                    None
                }
            }
        }
    }
}
