mod infra;
mod services;

use log::info;

use crate::{
    infra::{mqtt_messaging::MQTTMessaging, rmq_messaging::RabbitMQMessaging},
    services::service::BridgeServiceImpl,
};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("failure to read .env");
    env_logger::init();

    info!("starting application...");

    let mut rmq_messaging = RabbitMQMessaging::new();
    let _connect = rmq_messaging.connect().await;

    let service = BridgeServiceImpl::new(Box::new(rmq_messaging));

    let mut mqqt_messaging = MQTTMessaging::new(Box::new(service));

    mqqt_messaging.subscribe("HedroTraining2024/#".into(), 2);

    mqqt_messaging
        .connect()
        .await
        .expect("failure to connect to MQTT");

    info!("MQTT connected!")
}
