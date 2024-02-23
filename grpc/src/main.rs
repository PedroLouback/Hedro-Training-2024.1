pub mod server {
    tonic::include_proto!("server");
}

mod infra;
mod services;

use log::{error, info};
use std::{env, error::Error};

use tonic::transport::Server;

use crate::{
    infra::aws_timestream::AWSConnection, server::io_t_data_services_server::IoTDataServicesServer,
    services::data_services::IoTDataServicesImpl,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().expect("failure to read .env");
    env_logger::init();

    let env = match envs() {
        Ok(env) => env,
        Err(_) => return Err("Failed to read .env".into()),
    };
    let addr = format!("{}:{}", env.host, env.port).parse()?;

    info!("Starting gRPC server");

    let mut aws_connection = AWSConnection::new();
    aws_connection
        .connect()
        .await
        .expect("Failure to connect in AWS Timestream");

    let service = IoTDataServicesImpl::new(aws_connection);

    Server::builder()
        .add_service(IoTDataServicesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

struct Configs {
    host: String,
    port: String,
}

fn envs() -> Result<Configs, ()> {
    let Ok(host) = env::var("ADDRESS_HOST") else {
        error!("Failed to read ADDRESS_HOST env");
        return Err(());
    };

    let Ok(port) = env::var("ADDRESS_PORT") else {
        error!("Failed to read ADDRESS_PORT env");
        return Err(());
    };

    Ok(Configs { host, port })
}
