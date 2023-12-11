pub mod local_server {
    use actix_web::{get, App, HttpServer, Responder};

    // WServer yapısı
    pub struct WServer {
        pub socketaddr: String,
        pub port: String,
    }

    // IRequest enum'u
    pub enum IRequest {
        Get,
    }

    #[get("/")]
    async fn index() -> impl Responder {
        "Hello, Local Server Online!!!"
    }

    pub async fn start_server(server: &WServer) -> std::io::Result<()> {
        let h_server = HttpServer::new(move || {
            App::new().service(index)
        })
            .bind(format!("{}:{}", server.socketaddr, server.port))?;

        println!("Server Connection Opened on: {}:{}", server.socketaddr, server.port);

        h_server.run().await?;
        println!("Server Stopped");

        Ok(())
    }
}