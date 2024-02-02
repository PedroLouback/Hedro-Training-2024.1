use crate::services::messages::RMQMessage;
use crate::services::service::BridgeService;
use futures_util::StreamExt;
use lapin::message::Delivery;
use lapin::{
    options::{
        BasicAckOptions, BasicConsumeOptions, ExchangeDeclareOptions, QueueBindOptions,
        QueueDeclareOptions,
    },
    types::FieldTable,
    Channel, Connection, ConnectionProperties,
};
use log::{error, info};
use std::env;
struct RMQConfigs {
    host: String,
    port: String,
    user: String,
    password: String,
    queue: String,
    exchange: String,
}

pub struct RMQConnection {
    service: Box<dyn BridgeService>,
}

impl RMQConnection {
    pub fn new(service: Box<dyn BridgeService>) -> Self {
        RMQConnection { service }
    }
    fn envs(&self) -> Result<RMQConfigs, ()> {
        let Ok(host) = env::var("RABBITMQ_HOST") else {
            error!("Failed to read RABBITMQ_HOST env");
            return Err(());
        };

        let Ok(port) = env::var("RABBITMQ_PORT") else {
            error!("Failed to read RABBITMQ_PORT env");
            return Err(());
        };

        let Ok(user) = env::var("RABBITMQ_USER") else {
            error!("Failed to read RABBITMQ_USER env");
            return Err(());
        };

        let Ok(password) = env::var("RABBITMQ_PASSWORD") else {
            error!("Failed to read RABBITMQ_PASSWORD env");
            return Err(());
        };

        let Ok(queue) = env::var("RABBITMQ_QUEUE") else {
            error!("Failed to read RABBITMQ_QUEUE env");
            return Err(());
        };

        let Ok(exchange) = env::var("RABBITMQ_EXCHANGE") else {
            error!("Failed to read RABBITMQ_EXCHANGE env");
            return Err(());
        };

        Ok(RMQConfigs {
            host,
            port,
            user,
            password,
            queue,
            exchange,
        })
    }

    pub async fn connect(&mut self) -> Result<(Connection, Channel), ()> {
        let envs = self.envs()?;

        info!("Starting RabbitMQ connection...");

        let addr = format!(
            "amqp://{}:{}@{}:{}",
            envs.user, envs.password, envs.host, envs.port
        );

        let Ok(conn) = Connection::connect(&addr, ConnectionProperties::default()).await else {
            error!("RabbitMQ connection failure!");
            return Err(());
        };

        info!("RabbitMQ Connected!");

        info!("Starting RabbitMQ Channel...");

        let Ok(channel) = conn.create_channel().await else {
            error!("RabbitMQ channel failure!");
            return Err(());
        };

        let Ok(_queue) = channel
            .queue_declare(
                &envs.queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("RabbitMQ queue failure!");
            return Err(());
        };

        let Ok(_exchange) = channel
            .exchange_declare(
                &envs.exchange,
                lapin::ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("RabbitMQ exchange failure!");
            return Err(());
        };

        let Ok(_bind) = channel
            .queue_bind(
                &envs.queue,
                &envs.exchange,
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("RabbitMQ queue bind failure!");
            return Err(());
        };

        let mut consumer = channel
            .basic_consume(
                &envs.queue,
                "my_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("Failure to create consumer");

        while let Some(event_result) = consumer.next().await {
            let event = match event_result {
                Ok(event) => event,
                Err(err) => {
                    error!("Error receiving message from RabbitMQ: {:?}", err);
                    continue;
                }
            };

            let delivery_tag = event.delivery_tag;

            self.handler(Ok(event)).await;

            let Ok(_ack) = channel
                .basic_ack(delivery_tag, BasicAckOptions::default())
                .await
            else {
                error!("Acknowledge the message failure!");
                return Err(());
            };
        }

        Ok((conn, channel))
    }

    async fn handler(&self, event: Result<Delivery, lapin::Error>) {
        let delivery = match event {
            Ok(delivery) => delivery,
            Err(_) => {
                error!("Error receiving message from RabbitMQ");
                return;
            }
        };

        let data = delivery.data;

        let Ok(msg) = serde_json::from_slice::<RMQMessage>(&*data) else {
            error!("Failed to deserialized message!");
            return;
        };

        info!("Receiving message successfully: {:?}", msg);

        match self.service.exec(&msg) {
            Ok(_) => {
                info!("Message processed successfully");
            }
            Err(_) => {
                error!("Failed to process message");
            }
        }
    }
}
