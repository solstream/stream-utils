#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod types;

use napi::{
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Error, Result,
};

pub use types::{
    ShredstreamCommitmentLevel, ShredstreamEntriesRequest, ShredstreamEntry,
    ShredstreamFilterAccounts, ShredstreamFilterSlots, ShredstreamFilterTransactions,
};

#[napi]
pub struct ShredstreamProxyClient {
    client: shredstream_proxy_client::ProxyClient,
}

#[napi]
pub async fn connect_shredstream_proxy(endpoint: String) -> Result<ShredstreamProxyClient> {
    let client = shredstream_proxy_client::ProxyClient::connect(endpoint)
        .await
        .map_err(|e| Error::from_reason(format!("Connection error: {}", e)))?;

    Ok(ShredstreamProxyClient { client })
}

#[napi]
impl ShredstreamProxyClient {
    #[napi]
    pub fn subscribe_entries(
        &self,
        subscribe_request: ShredstreamEntriesRequest,
        on_entry: ThreadsafeFunction<ShredstreamEntry>,
    ) -> Result<()> {
        let request = subscribe_request.into();
        let mut client = self.client.clone();

        napi::tokio::spawn(async move {
            if let Err(e) = run_stream(&mut client, request, on_entry).await {
                eprintln!("Stream error: {}", e);
            }
        });

        Ok(())
    }
}

async fn run_stream(
    client: &mut shredstream_proxy_client::ProxyClient,
    request: shredstream_proxy_client::proto::SubscribeEntriesRequest,
    on_entry: ThreadsafeFunction<ShredstreamEntry>,
) -> Result<()> {
    let mut stream = client
        .subscribe_entries(request)
        .await
        .map_err(|e| Error::from_reason(format!("Subscription error: {}", e)))?;

    while let Some(slot_entry) = stream
        .message()
        .await
        .map_err(|e| Error::from_reason(format!("Stream error: {}", e)))?
    {
        on_entry.call(
            Ok(ShredstreamEntry {
                slot: slot_entry.slot as u32,
                entries: slot_entry.entries,
            }),
            ThreadsafeFunctionCallMode::NonBlocking,
        );
    }

    Ok(())
}
