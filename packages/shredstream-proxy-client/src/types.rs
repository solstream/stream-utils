use std::collections::HashMap;

use serde::Deserialize;

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

impl From<ShredstreamCommitmentLevel> for shredstream_proxy_client::proto::CommitmentLevel {
    fn from(level: ShredstreamCommitmentLevel) -> Self {
        match level {
            ShredstreamCommitmentLevel::Processed => {
                shredstream_proxy_client::proto::CommitmentLevel::Processed
            }
            ShredstreamCommitmentLevel::Confirmed => {
                shredstream_proxy_client::proto::CommitmentLevel::Confirmed
            }
            ShredstreamCommitmentLevel::Finalized => {
                shredstream_proxy_client::proto::CommitmentLevel::Finalized
            }
        }
    }
}

impl From<ShredstreamEntriesRequest> for shredstream_proxy_client::proto::SubscribeEntriesRequest {
    fn from(request: ShredstreamEntriesRequest) -> Self {
        shredstream_proxy_client::proto::SubscribeEntriesRequest {
            accounts: request
                .accounts
                .unwrap_or_default()
                .into_iter()
                .map(|(key, value)| {
                    (
                        key,
                        shredstream_proxy_client::proto::SubscribeRequestFilterAccounts {
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
                        shredstream_proxy_client::proto::SubscribeRequestFilterTransactions {
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
                        shredstream_proxy_client::proto::SubscribeRequestFilterSlots {
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
}
