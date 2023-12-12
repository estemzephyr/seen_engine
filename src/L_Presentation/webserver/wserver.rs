use actix_web::{get, App, HttpServer, Responder};

pub enum IRequest {
    Get,
}

pub struct WServer {
    pub socketaddr: String,
    pub port: String,
}

pub mod local_server {
    use super::{get, IRequest, WServer};
    use actix_web::{App, HttpServer, Responder};

    #[get("/")]
    async fn index() -> impl Responder {
        "Hello, Local Server Online!!!"
    }

    pub async fn start_server(server: &WServer, request: IRequest) -> std::io::Result<()> {
        let h_server = HttpServer::new(move || {
            App::new().service(index)
        })
            .bind(format!("{}:{}", server.socketaddr, server.port))?;

        println!("Server Connection Opened on: {}:{}", server.socketaddr, server.port);

        match request {
            IRequest::Get => {
                println!("Handling GET request");
                // You can add specific handling for the GET request here
            }
        }

        h_server.run().await?;
        println!("Server Stopped");

        Ok(())
    }
}
