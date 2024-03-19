use lapin::{Error, Connection, ConnectionProperties, options::BasicConsumeOptions, types::FieldTable, Channel, Consumer};
pub struct RabbitMQService {
    channel: Channel,
    pub consumer: Consumer
}

impl RabbitMQService {
    pub async fn new() -> Result<Self, Error> {
        let address = "amqp://connector:coolpassword@127.0.0.1:5672";
        let connection = Connection::connect(&address, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;
        let consumer = channel
            .basic_consume(
                "data_update",
                "consumer_app",
                BasicConsumeOptions { 
                    no_local: false, 
                    no_ack: true, 
                    exclusive: false, 
                    nowait: false },
                FieldTable::default(),
            )
        .await?;

        Ok(RabbitMQService {
            channel,
            consumer
        })
    }
}