pub mod server {
    tonic::include_proto!("server");
}

use aws_sdk_timestreamquery::Client;
use log::{debug, error, info};
use std::error::Error;

use server::{
    io_t_data_services_server::{IoTDataServices, IoTDataServicesServer}, ListIoTDataRequest, ListIoTDataResponse,
};

use tonic::transport::Server;

struct IoTDataServicesImpl {
    client: Client,
}

impl IoTDataServicesImpl {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[tonic::async_trait]
impl IoTDataServices for IoTDataServicesImpl {
    async fn list_io_t_data(
        &self,
        _request: tonic::Request<ListIoTDataRequest>,
    ) -> Result<tonic::Response<ListIoTDataResponse>, tonic::Status> {

        debug!("list_io_t_data function called");

        //realizar a consulta no timestrem

        let response = match self
            .client
            .query()
            .query_string("SELECT * FROM hdr-training.pedro")
            .send()
            .await
        {
            Ok(response) => response,
            Err(err) => {
                info!("Erro ao executar a consulta: {:?}", err);
                return Err(tonic::Status::internal("Falha ao executar a consulta"));
            }
        };

        debug!("Timestream query executed successfully");


        // converter o dado obtido no timestream para o tipo criado no protofile IoTData, e adicionar esses valores em um vetor
        for row in response.rows {
            let data = row.data;
            debug!("Column 1: {:?}", data[0]);
        }

        debug!("Timestream data processed successfully");

        // retornar o vetor

        //self.client.query().........
        //conversao do dado
        //
        Ok(tonic::Response::new(ListIoTDataResponse { data: vec![] }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let addr = "0.0.0.0:50051".parse()?;

    println!("Starting gRPC server");

    //fazemos a conexao com o timestrem, usando o sdk da aws na crate timestreamquery;
    let config = aws_config::load_from_env().await;

    let client = match aws_sdk_timestreamquery::Client::new(&config)
        .with_endpoint_discovery_enabled()
        .await
    {
        Ok((c, _)) => Ok(c),
        Err(err) => {
            error!("Failure to connect");
            Err(err)
        }
    }?;

    println!("Connected to Timestream");

    let service = IoTDataServicesImpl::new(client);

    Server::builder()
        .add_service(IoTDataServicesServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}