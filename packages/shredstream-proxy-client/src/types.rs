use std::collections::HashMap;

use napi::bindgen_prelude::BigInt;

#[napi(object)]
pub struct ShredstreamFilterAccounts {
    pub account: Vec<String>,
    pub owner: Vec<String>,
    pub filters: Vec<String>,
    pub nonempty_txn_signature: Option<bool>,
}

#[napi(object)]
pub struct ShredstreamFilterTransactions {
    pub account_include: Vec<String>,
    pub account_exclude: Vec<String>,
    pub account_required: Vec<String>,
}

#[napi(object)]
pub struct ShredstreamFilterSlots {
    pub filter_by_commitment: Option<bool>,
    pub interslot_updates: Option<bool>,
}

#[napi(object)]
pub struct ShredstreamEntriesRequest {
    pub accounts: Option<HashMap<String, ShredstreamFilterAccounts>>,
    pub transactions: Option<HashMap<String, ShredstreamFilterTransactions>>,
    pub slots: Option<HashMap<String, ShredstreamFilterSlots>>,
    pub commitment: Option<ShredstreamCommitmentLevel>,
}

#[napi(object)]
pub struct ShredstreamEntry {
    pub slot: u32,
    pub entries: Vec<u8>,
}

#[napi]
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

#[napi(object)]
pub struct MessageHeader {
    /// The number of signatures required for this message to be considered
    /// valid. The signers of those signatures must match the first
    /// `num_required_signatures` of [`Message::account_keys`].
    // NOTE: Serialization-related changes must be paired with the direct read at sigverify.
    pub num_required_signatures: u8,

    /// The last `num_readonly_signed_accounts` of the signed keys are read-only
    /// accounts.
    pub num_readonly_signed_accounts: u8,

    /// The last `num_readonly_unsigned_accounts` of the unsigned keys are
    /// read-only accounts.
    pub num_readonly_unsigned_accounts: u8,
}

impl From<solana_message::MessageHeader> for MessageHeader {
    fn from(header: solana_message::MessageHeader) -> Self {
        MessageHeader {
            num_required_signatures: header.num_required_signatures,
            num_readonly_signed_accounts: header.num_readonly_signed_accounts,
            num_readonly_unsigned_accounts: header.num_readonly_unsigned_accounts,
        }
    }
}

#[napi(object)]
pub struct CompiledInstruction {
    /// Index into the transaction keys array indicating the program account that executes this instruction.
    pub program_id_index: u8,
    /// Ordered indices into the transaction keys array indicating which accounts to pass to the program.
    pub accounts: Vec<u8>,
    /// The program input data.
    pub data: Vec<u8>,
}

impl From<solana_message::compiled_instruction::CompiledInstruction> for CompiledInstruction {
    fn from(instruction: solana_message::compiled_instruction::CompiledInstruction) -> Self {
        CompiledInstruction {
            program_id_index: instruction.program_id_index,
            accounts: instruction.accounts,
            data: instruction.data,
        }
    }
}

#[napi(object)]
pub struct MessageAddressTableLookup {
    /// Address lookup table account key
    pub account_key: String,
    /// List of indexes used to load writable account addresses
    pub writable_indexes: Vec<u8>,
    /// List of indexes used to load readonly account addresses
    pub readonly_indexes: Vec<u8>,
}

impl From<solana_message::v0::MessageAddressTableLookup> for MessageAddressTableLookup {
    fn from(lookup: solana_message::v0::MessageAddressTableLookup) -> Self {
        MessageAddressTableLookup {
            account_key: lookup.account_key.to_string(),
            writable_indexes: lookup.writable_indexes,
            readonly_indexes: lookup.readonly_indexes,
        }
    }
}

#[napi]
pub enum MessageVersion {
    Legacy,
    V0,
}

#[napi(object)]
pub struct VersionedMessage {
    /// The message header, identifying signed and read-only `account_keys`.
    /// Header values only describe static `account_keys`, they do not describe
    /// any additional account keys loaded via address table lookups.
    pub header: MessageHeader,

    /// List of accounts loaded by this transaction.
    pub account_keys: Vec<String>,

    /// The blockhash of a recent block.
    pub recent_blockhash: String,

    /// Instructions that invoke a designated program, are executed in sequence,
    /// and committed in one atomic transaction if all succeed.
    ///
    /// # Notes
    ///
    /// Program indexes must index into the list of message `account_keys` because
    /// program id's cannot be dynamically loaded from a lookup table.
    ///
    /// Account indexes must index into the list of addresses
    /// constructed from the concatenation of three key lists:
    ///   1) message `account_keys`
    ///   2) ordered list of keys loaded from `writable` lookup table indexes
    ///   3) ordered list of keys loaded from `readable` lookup table indexes
    pub instructions: Vec<CompiledInstruction>,

    /// List of address table lookups used to load additional accounts
    /// for this transaction.
    pub address_table_lookups: Option<Vec<MessageAddressTableLookup>>,

    pub version: MessageVersion,
}

impl From<solana_message::VersionedMessage> for VersionedMessage {
    fn from(message: solana_message::VersionedMessage) -> Self {
        let mut new_message = VersionedMessage {
            header: message.header().clone().into(),
            account_keys: message
                .static_account_keys()
                .into_iter()
                .map(|key| key.to_string())
                .collect(),
            recent_blockhash: message.recent_blockhash().to_string(),
            instructions: message
                .instructions()
                .into_iter()
                .map(|instruction| instruction.clone().into())
                .collect(),
            address_table_lookups: None,
            version: MessageVersion::V0,
        };

        match message {
            solana_message::VersionedMessage::Legacy(_message) => {
                new_message.version = MessageVersion::Legacy;
            }
            solana_message::VersionedMessage::V0(message) => {
                new_message.address_table_lookups = Some(
                    message
                        .address_table_lookups
                        .into_iter()
                        .map(|lookup| lookup.into())
                        .collect(),
                );
                new_message.version = MessageVersion::V0;
            }
        }

        new_message
    }
}

#[napi(object)]
pub struct VersionedTransaction {
    /// List of signatures
    pub signatures: Vec<String>,
    /// Message to sign.
    pub message: VersionedMessage,
}

impl From<solana_transaction::versioned::VersionedTransaction> for VersionedTransaction {
    fn from(transaction: solana_transaction::versioned::VersionedTransaction) -> Self {
        VersionedTransaction {
            signatures: transaction
                .signatures
                .into_iter()
                .map(|signature| signature.to_string())
                .collect(),
            message: transaction.message.into(),
        }
    }
}

#[napi(object)]
pub struct Entry {
    pub num_hashes: BigInt,
    pub hash: String,
    pub transactions: Vec<VersionedTransaction>,
}

impl From<solana_entry::entry::Entry> for Entry {
    fn from(entry: solana_entry::entry::Entry) -> Self {
        Entry {
            num_hashes: entry.num_hashes.into(),
            hash: entry.hash.to_string(),
            transactions: entry
                .transactions
                .into_iter()
                .map(|transaction| transaction.into())
                .collect(),
        }
    }
}

#[napi(object)]
pub struct DecodedShredstreamEntry {
    pub slot: BigInt,
    pub entries: Vec<Entry>,
}
