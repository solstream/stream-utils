use crate::proto::{Entry, SubscribeEntriesRequest};
use std::error::Error;
use tonic::{
    codec::CompressionEncoding,
    transport::{Channel, ClientTlsConfig},
    Streaming,
};

pub mod proto;

#[derive(Default)]
pub struct ShredstreamClientConfig {
    pub send_compressed: Option<CompressionEncoding>,
    pub accept_compressed: Option<CompressionEncoding>,
    pub max_decoding_message_size: Option<usize>,
    pub max_encoding_message_size: Option<usize>,
}

#[derive(Clone)]
pub struct ShredstreamClient {
    client: crate::proto::shredstream::shredstream_proxy_client::ShredstreamProxyClient<Channel>,
}

impl ShredstreamClient {
    pub fn new(
        endpoint_url: impl AsRef<str>,
        config: Option<ShredstreamClientConfig>,
    ) -> Result<Self, Box<dyn Error>> {
        let config = config.unwrap_or_default();

        let tls_config = ClientTlsConfig::new()
            .with_native_roots()
            .with_webpki_roots();
        let channel = Channel::from_shared(endpoint_url.as_ref().to_string())
            .unwrap()
            .tls_config(tls_config)?
            .connect_lazy();

        let mut client =
            crate::proto::shredstream::shredstream_proxy_client::ShredstreamProxyClient::new(
                channel,
            );

        if let Some(encoding) = config.send_compressed {
            client = client.send_compressed(encoding);
        }
        if let Some(encoding) = config.accept_compressed {
            client = client.accept_compressed(encoding);
        }
        if let Some(limit) = config.max_decoding_message_size {
            client = client.max_decoding_message_size(limit);
        }
        if let Some(limit) = config.max_encoding_message_size {
            client = client.max_encoding_message_size(limit);
        }

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
