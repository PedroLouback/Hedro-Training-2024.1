struct AWSConfigs {
    database: String,
    table: String,
}

use std::env;

use aws_sdk_timestreamquery::types::Row;
use log::error;
use serde_json::to_string;
use tonic::{Request, Response};

use crate::{
    infra::aws_timestream::AWSConnection,
    server::{
        io_t_data_services_server::IoTDataServices, IoTData, IoTDataType, ListIoTDataRequest,
        ListIoTDataResponse,
    },
};

pub struct IoTDataServicesImpl {
    client: AWSConnection,
}

impl IoTDataServicesImpl {
    pub fn new(client: AWSConnection) -> Self {
        Self { client }
    }

    fn envs(&self) -> Result<AWSConfigs, ()> {
        let Ok(database) = env::var("AWS_DATABASE_NAME") else {
            error!("Failed to read AWS_DATABASE_NAME env");
            return Err(());
        };

        let Ok(table) = env::var("AWS_TABLE_NAME") else {
            error!("Failed to read AWS_TABLE_NAME env");
            return Err(());
        };

        Ok(AWSConfigs { database, table })
    }
}

#[tonic::async_trait]
impl IoTDataServices for IoTDataServicesImpl {
    async fn list_io_t_data(
        &self,
        request: Request<ListIoTDataRequest>,
    ) -> Result<Response<ListIoTDataResponse>, tonic::Status> {
        let env = self.envs().expect("Failed to read .env");

        let request_type = request.into_inner().r#type;

        let query_string = match request_type.as_str() {
            "HUMIDITY" => format!(
                "SELECT * FROM \"{}\".{} WHERE HUMIDITY IS NOT NULL",
                env.database, env.table
            ),
            "TEMPERATURE" => format!(
                "SELECT * FROM \"{}\".{} WHERE TEMP IS NOT NULL",
                env.database, env.table
            ),
            "" => format!("SELECT * FROM \"{}\".{}", env.database, env.table),
            _ => format!("SELECT * FROM \"{}\".{}", env.database, env.table),
        };

        // Realiza a consulta no Timestream
        let query = self.client.query(&query_string).await;

        // Converte o dado obtido no Timestream para o tipo criado no protofile IoTData,
        // e adiciona esses valores em um vetor
        let mut data_list = Vec::new();
        for response in query {
            for row in response.rows {
                let data = row_to_data(row);
                data_list.push(data);
            }
        }

        // Retorna o vetor como parte da resposta do gRPC
        Ok(Response::new(ListIoTDataResponse { data: data_list }))
    }
}

fn row_to_data(row: Row) -> IoTData {
    let device = to_string(&row.data[0].clone().scalar_value())
        .unwrap_or_default()
        .replace("\"", "");
    let temp_value = to_string(&row.data[3].clone().scalar_value())
        .unwrap_or_default()
        .replace("\"", "");
    let humidity_value = to_string(&row.data[4].clone().scalar_value())
        .unwrap_or_default()
        .replace("\"", "");

    if temp_value != "null" {
        return IoTData {
            device: device.clone(),
            r#type: IoTDataType::Temperature.into(),
            value: temp_value,
        };
    }

    if humidity_value != "null" {
        return IoTData {
            device: device.clone(),
            r#type: IoTDataType::Humidity.into(),
            value: humidity_value,
        };
    }

    IoTData {
        device: device.clone(),
        r#type: IoTDataType::Temperature.into(),
        value: "".to_string(),
    }
}
