mod rabbitmq;

use std::env;
use std::error::Error;
use rabbitmq::RabbitMQService;
use tokio::time::{sleep, Duration};
use virk_data::virk_data_client::VirkDataClient;
use virk_data::GetAllIdsRequest;

pub mod virk_data {
    tonic::include_proto!("virk_data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let (job_interval, entry_interval) = read_args();
    let mut grpc_client = VirkDataClient::connect("http://[::1]:50051").await?;
    let rabbit_service = RabbitMQService::new().await?;

    loop {
        let ids = grpc_client.get_all_ids(GetAllIdsRequest{})
            .await?
            .into_inner()
            .ids;

        for id in ids {
            let _ = rabbit_service.publish(id.to_string(), "data_updates".to_string()).await?;
            sleep(Duration::from_millis(entry_interval)).await;
        }

        sleep(Duration::from_secs(job_interval)).await;
    }
}

fn read_args() -> (u64, u64){
    let args: Vec<String> = env::args().collect();
    
    let job_interval: u64 = match args[1].parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Could not parse arg 1 as valid int, using default value");
            3600
        }
    };
    let entry_interval: u64 = match args[2].parse() {
        Ok(i) => i,
        Err(_) => {
            println!("Could not parse arg 2 as valid int, using default value");
            10
        }
    };

    return (job_interval, entry_interval);
}