use crate::L_Presentation::webserver::wserver::{WServer};

#[derive(Debug)]
pub struct server_service{
    pub(crate) server:WServer
}
impl server_service{
    pub fn connect_server(self) -> WServer {
        self.server
    }
    pub async fn create_local_serv(server:WServer){
        server.start_local_server().await.expect("");
    }
}

