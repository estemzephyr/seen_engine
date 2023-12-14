use crate::L_Presentation::webserver::wserver::{IRequest, WServer};
use crate::MicroServiceHandler::Services::IService;

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
    pub async fn process_req(request: IRequest, engine: &IService,stream_service: IService) {
        // Server Defined Local If want to connect use method WServer::connect_server()
        let server = WServer { socketaddr: "127.0.0.1".to_string(), port: "8080".to_string() };

    }
}

