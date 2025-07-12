use crate::proto::{Entry, SubscribeEntriesRequest};
use std::error::Error;
use tonic::{transport::Channel, Streaming};

pub mod proto;

#[derive(Clone)]
pub struct ShredstreamClient {
    client: crate::proto::shredstream::shredstream_proxy_client::ShredstreamProxyClient<Channel>,
}

impl ShredstreamClient {
    pub async fn connect(endpoint: impl AsRef<str>) -> Result<Self, Box<dyn Error>> {
        let client =
            crate::proto::shredstream::shredstream_proxy_client::ShredstreamProxyClient::connect(
                endpoint.as_ref().to_string(),
            )
            .await
            .unwrap();

        Ok(Self { client })
    }

    pub async fn subscribe_entries(
        &mut self,
        request: SubscribeEntriesRequest,
    ) -> Result<Streaming<Entry>, Box<dyn Error>> {
        let response = self
            .client
            .subscribe_entries(request)
            .await
            .map_err(|e| Box::new(e))?;

        Ok(response.into_inner())
    }
}
