use log::info;

use crate::{infra::rmq_messaging::RMQConnection, services::service::BridgeServiceImpl};

mod infra;
mod services;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failure to read .env");
    env_logger::init();

    info!("Starting consumer application...");

    let service = BridgeServiceImpl::new();

    let mut consumer = RMQConnection::new(Box::new(service));

    consumer
        .connect()
        .await
        .expect("RabbitMQ connection failure!");

    


    info!("RabbitMQ connected!");
}
