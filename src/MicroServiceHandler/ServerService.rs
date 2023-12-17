
use async_trait::async_trait;
use crate::L_Data::IDataObj::IData::IData;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::server_manager::server_service;
use crate::L_Presentation::webserver::wserver::{IRequest};

#[async_trait]
pub trait ServerServiceEngine {
    async fn start_server(&self);
    async fn process_req(request: IRequest, stream_service: stream_service) -> Option<IData>;
}

#[async_trait]
impl ServerServiceEngine for server_service {
    async fn start_server(&self){
        &self.server;
    }
    async fn process_req(request: IRequest, mut stream_service: stream_service) -> Option<IData> {
        match request {
            IRequest::Get => {
                // Veri almak için bir loop oluştur
                while let Some(datas) = stream_service.stream.receiver.recv().await {
                    println!("On Stream data is: {:?}", datas);

                    // Process value and send server , but i not have any real server
                }

                // Option type
                None
            }
        }
    }
}
