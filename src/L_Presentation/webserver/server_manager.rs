use tokio::sync::mpsc::UnboundedReceiver;
use crate::L_Data::IDataObj::IData::IData;
use crate::L_Presentation::stream_module::stream_manager::stream_service;
use crate::L_Presentation::webserver::wserver::{IRequest, WServer};
use crate::MicroServiceHandler::StreamService::StreamServiceEngine;
use crate::MicroServiceHandler::ServerService::ServerServiceEngine;
#[derive(Debug)]
pub struct server_service{
    pub(crate) server:WServer
}
impl server_service{
    pub fn connect_server(self) -> WServer {
        self.server
    }
    pub fn create_new_server() -> server_service{
        server_service{
            server: WServer { socketaddr: "".to_string(), port: "".to_string() },
        }
    }
}

