use std::error::Error;

use aws_sdk_timestreamquery::{operation::query::QueryOutput, Client};
use log::{error, info};

pub struct AWSConnection {
    client: Option<Client>,
}

impl AWSConnection {
    pub fn new() -> Self {
        AWSConnection { client: None }
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let config = aws_config::load_from_env().await;

        self.client = match aws_sdk_timestreamquery::Client::new(&config)
            .with_endpoint_discovery_enabled()
            .await
        {
            Ok((c, _)) => Some(c),
            Err(err) => {
                error!("Failure to connect");
                return Err(err.into());
            }
        };

        info!("Connected to Timestream");

        Ok(())
    }

    pub async fn query(
        &self,
        query_string: &str,
    ) -> Result<QueryOutput, Box<dyn Error + Send + Sync>> {
        if let Some(client) = &self.client {
            let response = client.query().query_string(query_string).send().await?;
            Ok(response)
        } else {
            Err("
            Not connected to Timestream"
                .into())
        }
    }
}
