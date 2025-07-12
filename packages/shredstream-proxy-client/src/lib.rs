#![deny(clippy::all)]

use std::collections::HashMap;

use napi::{
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
    Error, Result,
};
use serde::Deserialize;

#[macro_use]
extern crate napi_derive;

#[napi(object)]
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShredstreamFilterAccounts {
    #[serde(default)]
    pub account: Vec<String>,
    #[serde(default)]
    pub owner: Vec<String>,
    #[serde(default)]
    pub filters: Vec<String>,
    #[serde(default)]
    pub nonempty_txn_signature: Option<bool>,
}

#[napi(object)]
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShredstreamFilterTransactions {
    #[serde(default)]
    pub account_include: Vec<String>,
    #[serde(default)]
    pub account_exclude: Vec<String>,
    #[serde(default)]
    pub account_required: Vec<String>,
}

#[napi(object)]
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ShredstreamFilterSlots {
    #[serde(default)]
    pub filter_by_commitment: Option<bool>,
    #[serde(default)]
    pub interslot_updates: Option<bool>,
}

#[napi(object)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShredstreamEntriesRequest {
    #[serde(default)]
    pub accounts: Option<HashMap<String, ShredstreamFilterAccounts>>,
    #[serde(default)]
    pub transactions: Option<HashMap<String, ShredstreamFilterTransactions>>,
    #[serde(default)]
    pub slots: Option<HashMap<String, ShredstreamFilterSlots>>,
    pub commitment: Option<ShredstreamCommitmentLevel>,
}

#[napi(object)]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShredstreamEntry {
    pub slot: u32,
    pub entries: Vec<u8>,
}

#[napi]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ShredstreamCommitmentLevel {
    Finalized,
    Confirmed,
    Processed,
}

impl From<ShredstreamCommitmentLevel>
    for shredstream_proxy_client::proto::ShredstreamCommitmentLevel
{
    fn from(level: ShredstreamCommitmentLevel) -> Self {
        match level {
            ShredstreamCommitmentLevel::Processed => {
                shredstream_proxy_client::proto::ShredstreamCommitmentLevel::Processed
            }
            ShredstreamCommitmentLevel::Confirmed => {
                shredstream_proxy_client::proto::ShredstreamCommitmentLevel::Confirmed
            }
            ShredstreamCommitmentLevel::Finalized => {
                shredstream_proxy_client::proto::ShredstreamCommitmentLevel::Finalized
            }
        }
    }
}

fn node_subscribe_request_to_subscribe_request(
    request: ShredstreamEntriesRequest,
) -> shredstream_proxy_client::proto::ShredstreamSubscribeEntriesRequest {
    shredstream_proxy_client::proto::ShredstreamSubscribeEntriesRequest {
        accounts: request
            .accounts
            .unwrap_or_default()
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    shredstream_proxy_client::proto::ShredstreamSubscribeRequestFilterAccounts {
                        account: value.account,
                        owner: value.owner,
                        filters: vec![],
                        nonempty_txn_signature: Some(
                            value.nonempty_txn_signature.unwrap_or_default(),
                        ),
                    },
                )
            })
            .collect(),

        transactions: request
            .transactions
            .unwrap_or_default()
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    shredstream_proxy_client::proto::ShredstreamSubscribeRequestFilterTransactions {
                        account_include: value.account_include,
                        account_exclude: value.account_exclude,
                        account_required: value.account_required,
                    },
                )
            })
            .collect(),

        slots: request
            .slots
            .unwrap_or_default()
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    shredstream_proxy_client::proto::ShredstreamSubscribeRequestFilterSlots {
                        filter_by_commitment: value.filter_by_commitment,
                        interslot_updates: value.interslot_updates,
                    },
                )
            })
            .collect(),

        commitment: request
            .commitment
            .map(|c| ShredstreamCommitmentLevel::from(c) as i32),
    }
}

#[napi]
pub struct ShredstreamProxyClient {
    client: shredstream_proxy_client::ShredstreamProxyClient,
}

#[napi]
pub async fn create_shredstream_proxy_client(endpoint: String) -> Result<ShredstreamProxyClient> {
    let client = shredstream_proxy_client::ShredstreamProxyClient::connect(endpoint)
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

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
        let request = node_subscribe_request_to_subscribe_request(subscribe_request);
        let mut client = self.client.clone();

        napi::tokio::spawn(async move {
            let mut stream = client
                .subscribe_entries(request)
                .await
                .map_err(|e| Error::from_reason(format!("Subscription error: {}", e)))?;

            while let Ok(slot_entry) = stream.message().await {
                match slot_entry {
                    Some(data) => {
                        on_entry.call(
                            Ok(ShredstreamEntry {
                                slot: data.slot as u32,
                                entries: data.entries,
                            }),
                            ThreadsafeFunctionCallMode::NonBlocking,
                        );
                    }
                    None => {
                        break;
                    }
                }
            }

            Ok::<(), Error>(())
        });

        Ok(())
    }
}
