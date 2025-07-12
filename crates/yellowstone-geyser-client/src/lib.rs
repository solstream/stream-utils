use futures::stream;
use std::error::Error;
use tonic::{
    codec::CompressionEncoding,
    metadata::{AsciiMetadataValue, MetadataValue},
    service::{interceptor::InterceptedService, Interceptor},
    transport::{Channel, ClientTlsConfig},
    Request, Status, Streaming,
};

use crate::proto::geyser::{
    CommitmentLevel, GetBlockHeightRequest, GetBlockHeightResponse, GetLatestBlockhashRequest,
    GetLatestBlockhashResponse, GetSlotRequest, GetSlotResponse, GetVersionRequest,
    GetVersionResponse, IsBlockhashValidRequest, IsBlockhashValidResponse, PingRequest,
    PongResponse, SubscribeRequest, SubscribeUpdate,
};

pub mod proto;

impl Interceptor for InterceptorXToken {
    fn call(&mut self, mut request: Request<()>) -> Result<Request<()>, Status> {
        if let Some(x_token) = self.x_token.clone() {
            request.metadata_mut().insert("x-token", x_token);
        }
        if self.x_request_snapshot {
            request
                .metadata_mut()
                .insert("x-request-snapshot", MetadataValue::from_static("true"));
        }
        Ok(request)
    }
}

#[derive(Debug, Clone)]
pub struct InterceptorXToken {
    pub x_token: Option<AsciiMetadataValue>,
    pub x_request_snapshot: bool,
}

#[derive(Default)]
pub struct GeyserClientConfig {
    pub x_token: Option<AsciiMetadataValue>,
    pub x_request_snapshot: bool,
    pub send_compressed: Option<CompressionEncoding>,
    pub accept_compressed: Option<CompressionEncoding>,
    pub max_decoding_message_size: Option<usize>,
    pub max_encoding_message_size: Option<usize>,
}

#[derive(Clone)]
pub struct GeyserClient {
    client: crate::proto::geyser::geyser_client::GeyserClient<
        InterceptedService<Channel, InterceptorXToken>,
    >,
}

impl GeyserClient {
    pub fn new(
        endpoint_url: impl AsRef<str>,
        config: Option<GeyserClientConfig>,
    ) -> Result<Self, Box<dyn Error>> {
        let config = config.unwrap_or_default();

        let tls_config = ClientTlsConfig::new()
            .with_native_roots()
            .with_webpki_roots();
        let channel = Channel::from_shared(endpoint_url.as_ref().to_string())
            .unwrap()
            .tls_config(tls_config)?
            .connect_lazy();

        let mut client = crate::proto::geyser::geyser_client::GeyserClient::with_interceptor(
            channel,
            InterceptorXToken {
                x_token: config.x_token,
                x_request_snapshot: config.x_request_snapshot,
            },
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

    pub async fn subscribe(
        &mut self,
        request: SubscribeRequest,
    ) -> Result<Streaming<SubscribeUpdate>, Box<dyn Error>> {
        let request = Request::new(stream::once(async move { request }));

        let response = self
            .client
            .subscribe(request)
            .await
            .map_err(|e| Box::new(e))?;

        Ok(response.into_inner())
    }

    pub async fn ping(&mut self, count: i32) -> Result<PongResponse, Box<dyn Error>> {
        let message = PingRequest { count };
        let request = tonic::Request::new(message);
        let response = self.client.ping(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_latest_blockhash(
        &mut self,
        commitment: Option<CommitmentLevel>,
    ) -> Result<GetLatestBlockhashResponse, Box<dyn Error>> {
        let request = tonic::Request::new(GetLatestBlockhashRequest {
            commitment: commitment.map(|value| value as i32),
        });
        let response = self.client.get_latest_blockhash(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_block_height(
        &mut self,
        commitment: Option<CommitmentLevel>,
    ) -> Result<GetBlockHeightResponse, Box<dyn Error>> {
        let request = tonic::Request::new(GetBlockHeightRequest {
            commitment: commitment.map(|value| value as i32),
        });
        let response = self.client.get_block_height(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_slot(
        &mut self,
        commitment: Option<CommitmentLevel>,
    ) -> Result<GetSlotResponse, Box<dyn Error>> {
        let request = tonic::Request::new(GetSlotRequest {
            commitment: commitment.map(|value| value as i32),
        });
        let response = self.client.get_slot(request).await?;

        Ok(response.into_inner())
    }

    pub async fn is_blockhash_valid(
        &mut self,
        blockhash: String,
        commitment: Option<CommitmentLevel>,
    ) -> Result<IsBlockhashValidResponse, Box<dyn Error>> {
        let request = tonic::Request::new(IsBlockhashValidRequest {
            blockhash,
            commitment: commitment.map(|value| value as i32),
        });
        let response = self.client.is_blockhash_valid(request).await?;

        Ok(response.into_inner())
    }

    pub async fn get_version(&mut self) -> Result<GetVersionResponse, Box<dyn Error>> {
        let request = tonic::Request::new(GetVersionRequest {});
        let response = self.client.get_version(request).await?;

        Ok(response.into_inner())
    }
}
