use lapin::{options::BasicPublishOptions, BasicProperties, Channel, Connection, ConnectionProperties, Error};

pub struct RabbitMQService {
    channel: Channel
}

impl RabbitMQService {
    pub async fn new() -> Result<Self, Error> {
        let rabbit_address = "amqp://connector:coolpassword@127.0.0.1:5672";
        let rabbit_connection = Connection::connect(&rabbit_address, ConnectionProperties::default()).await?;
        let rabbit_channel = rabbit_connection.create_channel().await?;

        Ok(RabbitMQService { 
            channel: rabbit_channel 
        })
    }

    pub async fn publish(&self, msg: String, routing_key: String) -> Result<(), Error> {
        let _confirm = self.channel.basic_publish(
            "", 
            &routing_key, 
            BasicPublishOptions::default(), 
            msg.as_bytes(), 
            BasicProperties::default()
        ).await?.await?;
        Ok(())
    }
}