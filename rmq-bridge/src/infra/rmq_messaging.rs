use crate::services::service::Messaging;
use async_trait::async_trait;
use lapin::{
    options::{ExchangeDeclareOptions, QueueBindOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Channel, Connection, ConnectionProperties,
};
use log::{error, info};
use std::env;

struct RabbitMQConfigs {
    host: String,
    port: String,
    user: String,
    password: String,
}

pub struct RabbitMQMessaging {
    conn: Option<Connection>,
    channel: Option<Channel>,
}

impl RabbitMQMessaging {
    pub fn new() -> Self {
        RabbitMQMessaging {
            conn: None,
            channel: None,
        }
    }
}

#[async_trait]
impl Messaging for RabbitMQMessaging {
    async fn publish(&self, destination: String, data: &[u8]) -> Result<(), ()> {
        if self.channel.is_none() {
            error!("connection wasnt established yet");
            return Err(());
        }

        match self
            .channel
            .clone()
            .unwrap()
            .basic_publish(
                &destination,
                "",
                lapin::options::BasicPublishOptions {
                    mandatory: false,
                    immediate: false,
                },
                data,
                BasicProperties::default(),
            )
            .await
        {
            Ok(_) => {
                info!("published to rqm");
                Ok(())
            }
            Err(_) => {
                error!("failed to publish msg to rmq");
                Err(())
            }
        }
    }
}

impl RabbitMQMessaging {
    fn envs(&self) -> Result<RabbitMQConfigs, ()> {
        let Ok(host) = env::var("RABBITMQ_HOST") else {
            error!("failed to read RABBITMQ_HOST env");
            return Err(());
        };

        let Ok(port) = env::var("RABBITMQ_PORT") else {
            error!("failed to read RABBITMQ_PORT");
            return Err(());
        };

        let Ok(user) = env::var("RABBITMQ_USER") else {
            error!("failed to read RABBITMQ_USER");
            return Err(());
        };

        let Ok(password) = env::var("RABBITMQ_PASSWORD") else {
            error!("failed to read RABBITMQ_PASSWORD");
            return Err(());
        };

        Ok(RabbitMQConfigs {
            host,
            port,
            user,
            password,
        })
    }

    pub async fn connect(&mut self) -> Result<(), ()> {
        let envs = self.envs()?;

        info!("starting rabbitmq connection... ");

        let addr = format!(
            "amqp://{}:{}@{}:{}",
            envs.user, envs.password, envs.host, envs.port
        );

        let Ok(conn) = Connection::connect(&addr, ConnectionProperties::default()).await else {
            error!("rabbitmq connection failure");
            return Err(());
        };

        info!("rabbitmq connected!... ");

        info!("starting rabbitmq channel... ");

        let Ok(channel) = conn.create_channel().await else {
            error!("rabbitmq channel failure");
            return Err(());
        };

        info!("rabbitmq channel created! ");

        let Ok(_exchange) = channel
            .exchange_declare(
                "test",
                lapin::ExchangeKind::Fanout,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("rabbitmq exchange failure");
            return Err(());
        };

        info!("rabbitmq exchange created! ");

        let Ok(_queue) = channel
            .queue_declare(
                "batatinha",
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("rabbitmq queue failure");
            return Err(());
        };

        info!("rabbitmq queue created! ",);

        let Ok(_queue_bind) = channel
            .queue_bind(
                "batatinha",
                "test",
                "",
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await
        else {
            error!("rabbitmq queue bind failure");
            return Err(());
        };

        info!("rabbitmq queue bind created! ",);

        self.conn = Some(conn);
        self.channel = Some(channel);

        Ok(())
    }
}
