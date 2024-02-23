pub mod server {
    tonic::include_proto!("server");
}

mod infra;
mod services;

use log::info;
use std::error::Error;

use tonic::transport::Server;

use crate::{
    infra::aws_timestream::AWSConnection, server::io_t_data_services_server::IoTDataServicesServer, services::data_services::IoTDataServicesImpl
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    dotenvy::dotenv().expect("failure to read .env");
    env_logger::init();

    let addr = "0.0.0.0:50051".parse()?;

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
