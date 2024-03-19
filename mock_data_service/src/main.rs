use std::{error::Error, sync::Arc};
use grpc::GRPCService;
use mongo::MongoDBService;
use tonic::transport::Server;
use virk_data::virk_data_server::VirkDataServer;
mod mongo;
mod grpc;

pub mod virk_data {
    tonic::include_proto!("virk_data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let mongo = Arc::new(MongoDBService::new().await?);

    let addr = "[::1]:50051"
            .parse()
            .expect("invalid address");

    let service = GRPCService::new(mongo);

    let _ = Server::builder()
        .add_service(VirkDataServer::new(service))
        .serve(addr)
        .await;
    
    Ok(())
}
