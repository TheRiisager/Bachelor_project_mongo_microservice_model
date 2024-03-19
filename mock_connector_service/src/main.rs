use std::error::Error;
use db::DBService;
use rabbitmq::RabbitMQService;
use tokio_stream::StreamExt;
use virk_data::{virk_data_client::VirkDataClient, EntryUpdateRequest};

pub mod db;
pub mod rabbitmq;

pub mod virk_data {
    tonic::include_proto!("virk_data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut rabbitmq_service = RabbitMQService::new().await?;
    let mut data_client = VirkDataClient::connect("http://[::1]:50051").await?;
    let db_service = DBService::new().await;

    while let Some(Ok(delivery)) = rabbitmq_service.consumer.next().await {
        let id: u32 = serde_json::from_slice(&delivery.data).unwrap();
        let entry = db_service.load_entry(id).await?;
        let _ = data_client.add_or_update_entry(EntryUpdateRequest {
            id: entry.id,
            col1: entry.col1,
            col2: entry.col2
        }).await?;
    }

    Ok(())
}
