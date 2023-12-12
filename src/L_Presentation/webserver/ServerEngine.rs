use crate::L_Presentation::webserver::wserver::WServer;

#[derive(Debug)]
pub struct server_service{
    server:WServer
}
impl server_service{
    pub fn create_new_server() -> server_service{
        server_service{
            server: WServer { socketaddr: "".to_string(), port: "".to_string() },
        }
    }
}