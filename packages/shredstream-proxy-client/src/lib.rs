#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

mod decode;
mod types;

use crate::types::DecodedShredstreamEntry;
use decode::decode_entries;
use napi::{
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Error, Result,
};

pub use decode::node_decode_entries;
pub use types::{
    ShredstreamCommitmentLevel, ShredstreamEntriesRequest, ShredstreamEntry,
    ShredstreamFilterAccounts, ShredstreamFilterSlots, ShredstreamFilterTransactions,
};

#[napi]
pub struct ShredstreamClient {
    client: shredstream_proxy_client::ShredstreamClient,
}

#[napi]
pub async fn connect_shredstream(endpoint: String) -> Result<ShredstreamClient> {
    let client = shredstream_proxy_client::ShredstreamClient::connect(endpoint)
        .await
        .map_err(|e| Error::from_reason(format!("Connection error: {}", e)))?;

    Ok(ShredstreamClient { client })
}

#[napi]
impl ShredstreamClient {
    #[napi]
    pub fn subscribe_entries(
        &self,
        subscribe_request: ShredstreamEntriesRequest,
        on_entry: ThreadsafeFunction<ShredstreamEntry>,
    ) -> Result<()> {
        let request = subscribe_request.into();
        let mut client = self.client.clone();

        napi::tokio::spawn(async move {
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

            Ok::<(), Error>(())
        });

        Ok(())
    }

    #[napi]
    pub fn subscribe_decoded_entries(
        &self,
        subscribe_request: ShredstreamEntriesRequest,
        on_entry: ThreadsafeFunction<DecodedShredstreamEntry>,
    ) -> Result<()> {
        let request = subscribe_request.into();
        let mut client = self.client.clone();

        napi::tokio::spawn(async move {
            let mut stream = client
                .subscribe_entries(request)
                .await
                .map_err(|e| Error::from_reason(format!("Subscription error: {}", e)))?;

            while let Some(entry) = stream
                .message()
                .await
                .map_err(|e| Error::from_reason(format!("Stream error: {}", e)))?
            {
                let decoded_entry = decode_entries(entry.entries)
                    .map_err(|e| Error::from_reason(format!("Decoding error: {}", e)))?;

                on_entry.call(
                    Ok(DecodedShredstreamEntry {
                        slot: entry.slot.into(),
                        entries: decoded_entry
                            .into_iter()
                            .map(|entry| entry.into())
                            .collect(),
                    }),
                    ThreadsafeFunctionCallMode::NonBlocking,
                );
            }

            Ok::<(), Error>(())
        });

        Ok(())
    }
}
