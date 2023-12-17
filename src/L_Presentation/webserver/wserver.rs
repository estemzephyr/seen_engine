use actix_web::{get, App, HttpServer, Responder};
use crate::L_Presentation::webserver::wserver::local_server::index;

pub enum IRequest {
    Get,
}
#[derive(Debug)]
pub struct WServer {
    pub socketaddr: String,
    pub port: String,
}
pub mod local_server {
    use super::{get};
    use actix_web::{Responder};
    #[get("/")]
    async fn index() -> impl Responder {
        "Hello, Local Server Online!!!"
    }
}
impl WServer{

    pub async fn start_local_server(&self) -> std::io::Result<()> {
        let h_server = HttpServer::new(move || {
            App::new().service(index)
        })
            .bind(format!("{}:{}", self.socketaddr, self.port))?;

        println!("Server Connection Opened on: {}:{}", self.socketaddr, self.port);

        h_server.run().await?;
        println!("Server Stopped");

        Ok(())
    }
    pub async fn handle_get_req(server: &WServer, req:IRequest){
        match req {
            IRequest::Get => {
                println!("Handling GET request");
                // You can add specific handling for the GET request here
            }
        }
    }
}
