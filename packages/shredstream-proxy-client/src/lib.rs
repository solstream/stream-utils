#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

pub mod decode;
pub mod types;

use crate::types::{DecodedShredstreamEntry, ShredstreamClientConfig};
use napi::{
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    tokio::task::JoinHandle,
};
use solana_entry_decoder::decode_entries;
use types::{ShredstreamEntriesRequest, ShredstreamEntry};

#[napi]
pub struct ShredstreamSubscription {
    task_handle: JoinHandle<napi::Result<()>>,
    on_close: Option<ThreadsafeFunction<()>>,
}

#[napi]
impl ShredstreamSubscription {
    #[napi]
    pub fn close(&mut self) {
        self.on_close
            .as_mut()
            .map(|on_close| on_close.call(Ok(()), ThreadsafeFunctionCallMode::NonBlocking));
        self.task_handle.abort();
    }
}

#[napi]
pub struct ShredstreamClient {
    client: shredstream_proxy_client::ShredstreamClient,
}

#[napi]
pub fn create_shredstream_client(
    endpoint: String,
    config: Option<ShredstreamClientConfig>,
) -> napi::Result<ShredstreamClient> {
    let client =
        shredstream_proxy_client::ShredstreamClient::new(endpoint, config.map(|x| x.into()))
            .map_err(|e| napi::Error::from_reason(e.to_string()))?;

    Ok(ShredstreamClient { client })
}

#[napi]
impl ShredstreamClient {
    #[napi]
    pub fn subscribe_entries(
        &self,
        subscribe_request: Option<ShredstreamEntriesRequest>,
        on_entry: ThreadsafeFunction<ShredstreamEntry>,
        on_close: Option<ThreadsafeFunction<()>>,
    ) -> napi::Result<ShredstreamSubscription> {
        let request = subscribe_request.map(|request| request.into());
        let mut client = self.client.clone();
        let on_close_clone = on_close.clone();

        let task_handle = napi::tokio::spawn(async move {
            let result = async {
                let mut stream = client
                    .subscribe_entries(request.unwrap_or_default())
                    .await
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

                while let Some(slot_entry) = stream
                    .message()
                    .await
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?
                {
                    on_entry.call(
                        Ok(ShredstreamEntry {
                            slot: slot_entry.slot as u32,
                            entries: slot_entry.entries,
                        }),
                        ThreadsafeFunctionCallMode::NonBlocking,
                    );
                }

                Ok::<(), napi::Error>(())
            }
            .await;

            on_close_clone.map(|on_close| {
                on_close.call(result.clone(), ThreadsafeFunctionCallMode::NonBlocking)
            });

            result
        });

        Ok(ShredstreamSubscription {
            task_handle,
            on_close,
        })
    }

    #[napi]
    pub fn subscribe_decoded_entries(
        &self,
        subscribe_request: ShredstreamEntriesRequest,
        on_entry: ThreadsafeFunction<DecodedShredstreamEntry>,
        on_close: Option<ThreadsafeFunction<()>>,
    ) -> napi::Result<ShredstreamSubscription> {
        let request = subscribe_request.into();
        let mut client = self.client.clone();
        let on_close_clone = on_close.clone();

        let task_handle = napi::tokio::spawn(async move {
            let result = async {
                let mut stream = client
                    .subscribe_entries(request)
                    .await
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?;

                while let Some(entry) = stream
                    .message()
                    .await
                    .map_err(|e| napi::Error::from_reason(e.to_string()))?
                {
                    let decoded_entry = decode_entries(entry.entries)
                        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

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

                Ok::<(), napi::Error>(())
            }
            .await;

            on_close_clone.map(|on_close| {
                on_close.call(result.clone(), ThreadsafeFunctionCallMode::NonBlocking)
            });

            result
        });

        Ok(ShredstreamSubscription {
            task_handle,
            on_close,
        })
    }
}
