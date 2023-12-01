use crate::MicroServiceHandler::ServiceHandler::Service;

mod MicroServiceHandler;
mod db_handler;
mod ErrorManager;
mod IDataObj;
mod sharding_engine;
mod stream_module;

#[tokio::main]
async fn main() {
    let engine = Service::default();
    let services = Service::create_service_engine(engine).await;
}
