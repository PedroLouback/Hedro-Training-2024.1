pub mod server {
    tonic::include_proto!("server");
}

use aws_sdk_timestreamquery::types::Row;
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
}

#[tonic::async_trait]
impl IoTDataServices for IoTDataServicesImpl {
    async fn list_io_t_data(
        &self,
        request: Request<ListIoTDataRequest>,
    ) -> Result<Response<ListIoTDataResponse>, tonic::Status> {
        let request_type = request.into_inner().r#type;

        let query_string = match request_type.as_str() {
            "HUMIDITY" => "SELECT * FROM \"hdr-training\".pedro WHERE HUMIDITY IS NOT NULL",
            "TEMPERATURE" => "SELECT * FROM \"hdr-training\".pedro WHERE TEMP IS NOT NULL",
            "" => "SELECT * FROM \"hdr-training\".pedro",
            _ => "SELECT * FROM \"hdr-training\".pedro",
        };

        // Realiza a consulta no Timestream
        let query = self.client.query(&query_string).await;

        // Converte o dado obtido no Timestream para o tipo criado no protofile IoTData,
        // e adicionar esses valores em um vetor
        let mut data_list = Vec::new();
        for response in query {
            for row in response.rows {
                let data = row_to_data(row);
                data_list.push(data);
            }
        }

        // Retornar o vetor como parte da resposta do gRPC
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
