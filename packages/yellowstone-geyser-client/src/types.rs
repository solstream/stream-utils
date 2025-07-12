use std::{collections::HashMap, str::FromStr};

use napi::bindgen_prelude::BigInt;
use tonic::metadata::MetadataValue;
use yellowstone_geyser_client::proto::geyser::subscribe_update::UpdateOneof;

#[napi(object)]
pub struct ClientConfig {
    pub x_token: Option<String>,
    pub x_request_snapshot: Option<bool>,
    pub max_decoding_message_size: Option<u32>,
    pub max_encoding_message_size: Option<u32>,
}

impl From<ClientConfig> for yellowstone_geyser_client::GeyserClientConfig {
    fn from(config: ClientConfig) -> Self {
        yellowstone_geyser_client::GeyserClientConfig {
            x_token: config.x_token.map(|x| MetadataValue::from_str(&x).unwrap()),
            x_request_snapshot: config.x_request_snapshot.unwrap_or(false),
            send_compressed: None,
            accept_compressed: None,
            max_decoding_message_size: config.max_decoding_message_size.map(|x| x as usize),
            max_encoding_message_size: config.max_encoding_message_size.map(|x| x as usize),
        }
    }
}

#[napi(object)]
pub struct ConfirmedBlock {
    pub previous_blockhash: String,
    pub blockhash: String,
    pub parent_slot: BigInt,
    pub transactions: Vec<ConfirmedTransaction>,
    pub rewards: Vec<Reward>,
    pub block_time: Option<u32>,
    pub block_height: Option<BlockHeight>,
    pub num_partitions: Option<NumPartitions>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::ConfirmedBlock> for ConfirmedBlock {
    fn from(block: yellowstone_geyser_client::proto::solana_storage::ConfirmedBlock) -> Self {
        ConfirmedBlock {
            previous_blockhash: block.previous_blockhash,
            blockhash: block.blockhash,
            parent_slot: block.parent_slot.into(),
            transactions: block.transactions.into_iter().map(|x| x.into()).collect(),
            rewards: block.rewards.into_iter().map(|x| x.into()).collect(),
            block_time: block.block_time.map(|x| x.timestamp as u32),
            block_height: block.block_height.map(|x| x.into()),
            num_partitions: block.num_partitions.map(|x| x.into()),
        }
    }
}

#[napi(object)]
pub struct ConfirmedTransaction {
    pub transaction: Option<Transaction>,
    pub meta: Option<TransactionStatusMeta>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::ConfirmedTransaction>
    for ConfirmedTransaction
{
    fn from(
        transaction: yellowstone_geyser_client::proto::solana_storage::ConfirmedTransaction,
    ) -> Self {
        ConfirmedTransaction {
            transaction: transaction.transaction.map(|x| x.into()),
            meta: transaction.meta.map(|x| x.into()),
        }
    }
}

#[napi(object)]
pub struct Transaction {
    pub signatures: Vec<Vec<u8>>,
    pub message: Option<Message>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::Transaction> for Transaction {
    fn from(transaction: yellowstone_geyser_client::proto::solana_storage::Transaction) -> Self {
        Transaction {
            signatures: transaction.signatures,
            message: transaction.message.map(|x| x.into()),
        }
    }
}

#[napi(object)]
pub struct Message {
    pub header: Option<MessageHeader>,
    pub account_keys: Vec<Vec<u8>>,
    pub recent_blockhash: Vec<u8>,
    pub instructions: Vec<CompiledInstruction>,
    pub versioned: bool,
    pub address_table_lookups: Vec<MessageAddressTableLookup>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::Message> for Message {
    fn from(message: yellowstone_geyser_client::proto::solana_storage::Message) -> Self {
        Message {
            header: message.header.map(|x| x.into()),
            account_keys: message.account_keys,
            recent_blockhash: message.recent_blockhash,
            instructions: message.instructions.into_iter().map(|x| x.into()).collect(),
            versioned: message.versioned,
            address_table_lookups: message
                .address_table_lookups
                .into_iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

#[napi(object)]
pub struct MessageHeader {
    pub num_required_signatures: u32,
    pub num_readonly_signed_accounts: u32,
    pub num_readonly_unsigned_accounts: u32,
}

impl From<yellowstone_geyser_client::proto::solana_storage::MessageHeader> for MessageHeader {
    fn from(header: yellowstone_geyser_client::proto::solana_storage::MessageHeader) -> Self {
        MessageHeader {
            num_required_signatures: header.num_required_signatures,
            num_readonly_signed_accounts: header.num_readonly_signed_accounts,
            num_readonly_unsigned_accounts: header.num_readonly_unsigned_accounts,
        }
    }
}

#[napi(object)]
pub struct MessageAddressTableLookup {
    pub account_key: Vec<u8>,
    pub writable_indexes: Vec<u8>,
    pub readonly_indexes: Vec<u8>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::MessageAddressTableLookup>
    for MessageAddressTableLookup
{
    fn from(
        lookup: yellowstone_geyser_client::proto::solana_storage::MessageAddressTableLookup,
    ) -> Self {
        MessageAddressTableLookup {
            account_key: lookup.account_key,
            writable_indexes: lookup.writable_indexes,
            readonly_indexes: lookup.readonly_indexes,
        }
    }
}

#[napi(object)]
pub struct TransactionStatusMeta {
    pub err: Option<TransactionError>,
    pub fee: BigInt,
    pub pre_balances: Vec<BigInt>,
    pub post_balances: Vec<BigInt>,
    pub inner_instructions: Vec<InnerInstructions>,
    pub inner_instructions_none: bool,
    pub log_messages: Vec<String>,
    pub log_messages_none: bool,
    pub pre_token_balances: Vec<TokenBalance>,
    pub post_token_balances: Vec<TokenBalance>,
    pub rewards: Vec<Reward>,
    pub loaded_writable_addresses: Vec<Vec<u8>>,
    pub loaded_readonly_addresses: Vec<Vec<u8>>,
    pub return_data: Option<ReturnData>,
    pub return_data_none: bool,
    /// Sum of compute units consumed by all instructions.
    /// Available since Solana v1.10.35 / v1.11.6.
    /// Set to `None` for txs executed on earlier versions.
    pub compute_units_consumed: Option<BigInt>,
    /// Total transaction cost
    pub cost_units: Option<BigInt>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::TransactionStatusMeta>
    for TransactionStatusMeta
{
    fn from(meta: yellowstone_geyser_client::proto::solana_storage::TransactionStatusMeta) -> Self {
        TransactionStatusMeta {
            err: meta.err.map(|x| x.into()),
            fee: meta.fee.into(),
            pre_balances: meta.pre_balances.into_iter().map(|x| x.into()).collect(),
            post_balances: meta.post_balances.into_iter().map(|x| x.into()).collect(),
            inner_instructions: meta
                .inner_instructions
                .into_iter()
                .map(|x| x.into())
                .collect(),
            inner_instructions_none: meta.inner_instructions_none,
            log_messages: meta.log_messages,
            log_messages_none: meta.log_messages_none,
            pre_token_balances: meta
                .pre_token_balances
                .into_iter()
                .map(|x| x.into())
                .collect(),
            post_token_balances: meta
                .post_token_balances
                .into_iter()
                .map(|x| x.into())
                .collect(),
            rewards: meta.rewards.into_iter().map(|x| x.into()).collect(),
            loaded_writable_addresses: meta.loaded_writable_addresses,
            loaded_readonly_addresses: meta.loaded_readonly_addresses,
            return_data: meta.return_data.map(|x| x.into()),
            return_data_none: meta.return_data_none,
            compute_units_consumed: meta.compute_units_consumed.map(|x| x.into()),
            cost_units: meta.cost_units.map(|x| x.into()),
        }
    }
}

#[napi(object)]
pub struct TransactionError {
    pub err: Vec<u8>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::TransactionError> for TransactionError {
    fn from(error: yellowstone_geyser_client::proto::solana_storage::TransactionError) -> Self {
        TransactionError { err: error.err }
    }
}

#[napi(object)]
pub struct InnerInstructions {
    pub index: u32,
    pub instructions: Vec<InnerInstruction>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::InnerInstructions>
    for InnerInstructions
{
    fn from(
        inner_instructions: yellowstone_geyser_client::proto::solana_storage::InnerInstructions,
    ) -> Self {
        InnerInstructions {
            index: inner_instructions.index,
            instructions: inner_instructions
                .instructions
                .into_iter()
                .map(|x| x.into())
                .collect(),
        }
    }
}

#[napi(object)]
pub struct InnerInstruction {
    pub program_id_index: u32,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
    /// Invocation stack height of an inner instruction.
    /// Available since Solana v1.14.6
    /// Set to `None` for txs executed on earlier versions.
    pub stack_height: Option<u32>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::InnerInstruction> for InnerInstruction {
    fn from(
        inner_instruction: yellowstone_geyser_client::proto::solana_storage::InnerInstruction,
    ) -> Self {
        InnerInstruction {
            program_id_index: inner_instruction.program_id_index,
            accounts: inner_instruction.accounts,
            data: inner_instruction.data,
            stack_height: inner_instruction.stack_height,
        }
    }
}

#[napi(object)]
pub struct CompiledInstruction {
    pub program_id_index: u32,
    pub accounts: Vec<u8>,
    pub data: Vec<u8>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::CompiledInstruction>
    for CompiledInstruction
{
    fn from(
        compiled_instruction: yellowstone_geyser_client::proto::solana_storage::CompiledInstruction,
    ) -> Self {
        CompiledInstruction {
            program_id_index: compiled_instruction.program_id_index,
            accounts: compiled_instruction.accounts,
            data: compiled_instruction.data,
        }
    }
}

#[napi(object)]
pub struct TokenBalance {
    pub account_index: u32,
    pub mint: String,
    pub ui_token_amount: Option<UiTokenAmount>,
    pub owner: String,
    pub program_id: String,
}

impl From<yellowstone_geyser_client::proto::solana_storage::TokenBalance> for TokenBalance {
    fn from(token_balance: yellowstone_geyser_client::proto::solana_storage::TokenBalance) -> Self {
        TokenBalance {
            account_index: token_balance.account_index,
            mint: token_balance.mint,
            ui_token_amount: token_balance.ui_token_amount.map(|x| x.into()),
            owner: token_balance.owner,
            program_id: token_balance.program_id,
        }
    }
}

#[napi(object)]
pub struct UiTokenAmount {
    pub ui_amount: f64,
    pub decimals: u32,
    pub amount: String,
    pub ui_amount_string: String,
}

impl From<yellowstone_geyser_client::proto::solana_storage::UiTokenAmount> for UiTokenAmount {
    fn from(
        ui_token_amount: yellowstone_geyser_client::proto::solana_storage::UiTokenAmount,
    ) -> Self {
        UiTokenAmount {
            ui_amount: ui_token_amount.ui_amount,
            decimals: ui_token_amount.decimals,
            amount: ui_token_amount.amount,
            ui_amount_string: ui_token_amount.ui_amount_string,
        }
    }
}

#[napi(object)]
pub struct ReturnData {
    pub program_id: Vec<u8>,
    pub data: Vec<u8>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::ReturnData> for ReturnData {
    fn from(return_data: yellowstone_geyser_client::proto::solana_storage::ReturnData) -> Self {
        ReturnData {
            program_id: return_data.program_id,
            data: return_data.data,
        }
    }
}

#[napi(object)]
pub struct Reward {
    pub pubkey: String,
    pub lamports: BigInt,
    pub post_balance: BigInt,
    pub reward_type: RewardType,
    pub commission: String,
}

impl From<yellowstone_geyser_client::proto::solana_storage::Reward> for Reward {
    fn from(reward: yellowstone_geyser_client::proto::solana_storage::Reward) -> Self {
        Reward {
            pubkey: reward.pubkey,
            lamports: reward.lamports.into(),
            post_balance: reward.post_balance.into(),
            reward_type: reward.reward_type.into(),
            commission: reward.commission,
        }
    }
}

#[napi(object)]
pub struct Rewards {
    pub rewards: Vec<Reward>,
    pub num_partitions: Option<NumPartitions>,
}

impl From<yellowstone_geyser_client::proto::solana_storage::Rewards> for Rewards {
    fn from(rewards: yellowstone_geyser_client::proto::solana_storage::Rewards) -> Self {
        Rewards {
            rewards: rewards.rewards.into_iter().map(|x| x.into()).collect(),
            num_partitions: rewards.num_partitions.map(|x| x.into()),
        }
    }
}

#[napi(object)]
pub struct UnixTimestamp {
    pub timestamp: i64,
}

impl From<yellowstone_geyser_client::proto::solana_storage::UnixTimestamp> for UnixTimestamp {
    fn from(timestamp: yellowstone_geyser_client::proto::solana_storage::UnixTimestamp) -> Self {
        UnixTimestamp {
            timestamp: timestamp.timestamp,
        }
    }
}

#[napi(object)]
pub struct BlockHeight {
    pub block_height: BigInt,
}

impl From<yellowstone_geyser_client::proto::solana_storage::BlockHeight> for BlockHeight {
    fn from(block_height: yellowstone_geyser_client::proto::solana_storage::BlockHeight) -> Self {
        BlockHeight {
            block_height: block_height.block_height.into(),
        }
    }
}

#[napi(object)]
pub struct NumPartitions {
    pub num_partitions: BigInt,
}

impl From<yellowstone_geyser_client::proto::solana_storage::NumPartitions> for NumPartitions {
    fn from(
        num_partitions: yellowstone_geyser_client::proto::solana_storage::NumPartitions,
    ) -> Self {
        NumPartitions {
            num_partitions: num_partitions.num_partitions.into(),
        }
    }
}

#[napi]
pub enum RewardType {
    Unspecified = 0,
    Fee = 1,
    Rent = 2,
    Staking = 3,
    Voting = 4,
}

impl From<i32> for RewardType {
    fn from(reward_type: i32) -> Self {
        match reward_type {
            0 => Self::Unspecified,
            1 => Self::Fee,
            2 => Self::Rent,
            3 => Self::Staking,
            4 => Self::Voting,
            _ => panic!("Invalid reward type: {}", reward_type),
        }
    }
}

#[napi]
pub enum CommitmentLevel {
    Processed = 0,
    Confirmed = 1,
    Finalized = 2,
}

impl From<CommitmentLevel> for i32 {
    fn from(commitment: CommitmentLevel) -> Self {
        match commitment {
            CommitmentLevel::Processed => 0,
            CommitmentLevel::Confirmed => 1,
            CommitmentLevel::Finalized => 2,
        }
    }
}

#[napi(object)]
pub struct SubscribeRequest {
    pub accounts: Option<HashMap<String, SubscribeRequestFilterAccounts>>,
    pub slots: Option<HashMap<String, SubscribeRequestFilterSlots>>,
    pub transactions: Option<HashMap<String, SubscribeRequestFilterTransactions>>,
    pub transactions_status: Option<HashMap<String, SubscribeRequestFilterTransactions>>,
    pub blocks: Option<HashMap<String, SubscribeRequestFilterBlocks>>,
    pub blocks_meta: Option<HashMap<String, SubscribeRequestFilterBlocksMeta>>,
    pub entry: Option<HashMap<String, SubscribeRequestFilterEntry>>,
    pub commitment: Option<CommitmentLevel>,
    pub accounts_data_slice: Option<Vec<SubscribeRequestAccountsDataSlice>>,
    pub ping: Option<SubscribeRequestPing>,
    pub from_slot: Option<BigInt>,
}

impl From<SubscribeRequest> for yellowstone_geyser_client::proto::geyser::SubscribeRequest {
    fn from(request: SubscribeRequest) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequest {
            accounts: request
                .accounts
                .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            slots: request
                .slots
                .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            transactions: request
                .transactions
                .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            transactions_status: request
                .transactions_status
                .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            blocks: request
                .blocks
                .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            blocks_meta: request
                .blocks_meta
                .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            entry: request
                .entry
                .map(|x| x.into_iter().map(|(k, v)| (k, v.into())).collect())
                .unwrap_or_default(),
            commitment: request.commitment.map(|x| x.into()),
            accounts_data_slice: request
                .accounts_data_slice
                .map(|x| x.into_iter().map(|x| x.into()).collect())
                .unwrap_or_default(),
            ping: request.ping.map(|x| x.into()),
            from_slot: request.from_slot.map(|x| x.get_u64().1),
        }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterAccounts {
    pub account: Vec<String>,
    pub owner: Vec<String>,
    pub filters: Vec<SubscribeRequestFilterAccountsFilter>,
    pub nonempty_txn_signature: Option<bool>,
}

impl From<SubscribeRequestFilterAccounts>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterAccounts
{
    fn from(request: SubscribeRequestFilterAccounts) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterAccounts {
            account: request.account,
            owner: request.owner,
            filters: request.filters.into_iter().map(|x| x.into()).collect(),
            nonempty_txn_signature: request.nonempty_txn_signature,
        }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterAccountsFilter {
    pub memcmp: Option<SubscribeRequestFilterAccountsFilterMemcmp>,
    pub datasize: Option<BigInt>,
    pub token_account_state: Option<bool>,
    pub lamports: Option<SubscribeRequestFilterAccountsFilterLamports>,
}

impl From<SubscribeRequestFilterAccountsFilter>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterAccountsFilter
{
    fn from(request: SubscribeRequestFilterAccountsFilter) -> Self {
        if let Some(memcmp) = request.memcmp {
            return Self {
                filter: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter::Filter::Memcmp(
                    memcmp.into(),
                )),
            };
        }
        if let Some(datasize) = request.datasize {
            return Self {
                filter: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter::Filter::Datasize(
                    datasize.get_u64().1,
                )),
            };
        }
        if let Some(token_account_state) = request.token_account_state {
            return Self {
                filter: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter::Filter::TokenAccountState(
                    token_account_state,
                )),
            };
        }
        if let Some(lamports) = request.lamports {
            return Self {
                filter: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter::Filter::Lamports(
                    lamports.into(),
                )),
            };
        }
        Self { filter: None }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterAccountsFilterMemcmp {
    pub offset: BigInt,
    pub bytes: Option<Vec<u8>>,
    pub base58: Option<String>,
    pub base64: Option<String>,
}

impl From<SubscribeRequestFilterAccountsFilterMemcmp>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterAccountsFilterMemcmp
{
    fn from(request: SubscribeRequestFilterAccountsFilterMemcmp) -> Self {
        let offset = request.offset.get_u64().1;
        if let Some(bytes) = request.bytes {
            return Self {
                offset,
                data: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter_memcmp::Data::Bytes(bytes)),
            };
        }
        if let Some(base58) = request.base58 {
            return Self {
                offset,
                data: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter_memcmp::Data::Base58(base58)),
            };
        }
        if let Some(base64) = request.base64 {
            return Self {
                offset,
                data: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter_memcmp::Data::Base64(base64)),
            };
        }
        Self { offset, data: None }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterAccountsFilterLamports {
    pub eq: Option<BigInt>,
    pub ne: Option<BigInt>,
    pub lt: Option<BigInt>,
    pub gt: Option<BigInt>,
}

impl From<SubscribeRequestFilterAccountsFilterLamports>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterAccountsFilterLamports
{
    fn from(request: SubscribeRequestFilterAccountsFilterLamports) -> Self {
        if let Some(eq) = request.eq {
            return Self {
                cmp: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter_lamports::Cmp::Eq(
                    eq.get_u64().1,
                )),
            };
        }
        if let Some(ne) = request.ne {
            return Self {
                cmp: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter_lamports::Cmp::Ne(
                    ne.get_u64().1,
                )),
            };
        }
        if let Some(lt) = request.lt {
            return Self {
                cmp: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter_lamports::Cmp::Lt(
                    lt.get_u64().1,
                )),
            };
        }
        if let Some(gt) = request.gt {
            return Self {
                cmp: Some(yellowstone_geyser_client::proto::geyser::subscribe_request_filter_accounts_filter_lamports::Cmp::Gt(
                    gt.get_u64().1,
                )),
            };
        }
        Self { cmp: None }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterSlots {
    pub filter_by_commitment: Option<bool>,
    pub interslot_updates: Option<bool>,
}

impl From<SubscribeRequestFilterSlots>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterSlots
{
    fn from(request: SubscribeRequestFilterSlots) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterSlots {
            filter_by_commitment: request.filter_by_commitment,
            interslot_updates: request.interslot_updates,
        }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterTransactions {
    pub vote: Option<bool>,
    pub failed: Option<bool>,
    pub signature: Option<String>,
    pub account_include: Vec<String>,
    pub account_exclude: Vec<String>,
    pub account_required: Vec<String>,
}

impl From<SubscribeRequestFilterTransactions>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterTransactions
{
    fn from(request: SubscribeRequestFilterTransactions) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterTransactions {
            vote: request.vote,
            failed: request.failed,
            signature: request.signature,
            account_include: request.account_include,
            account_exclude: request.account_exclude,
            account_required: request.account_required,
        }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterBlocks {
    pub account_include: Vec<String>,
    pub include_transactions: Option<bool>,
    pub include_accounts: Option<bool>,
    pub include_entries: Option<bool>,
}

impl From<SubscribeRequestFilterBlocks>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterBlocks
{
    fn from(request: SubscribeRequestFilterBlocks) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterBlocks {
            account_include: request.account_include,
            include_transactions: request.include_transactions,
            include_accounts: request.include_accounts,
            include_entries: request.include_entries,
        }
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterBlocksMeta {}

impl From<SubscribeRequestFilterBlocksMeta>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterBlocksMeta
{
    fn from(_request: SubscribeRequestFilterBlocksMeta) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterBlocksMeta {}
    }
}

#[napi(object)]
pub struct SubscribeRequestFilterEntry {}

impl From<SubscribeRequestFilterEntry>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterEntry
{
    fn from(_request: SubscribeRequestFilterEntry) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestFilterEntry {}
    }
}

#[napi(object)]
pub struct SubscribeRequestAccountsDataSlice {
    pub offset: BigInt,
    pub length: BigInt,
}

impl From<SubscribeRequestAccountsDataSlice>
    for yellowstone_geyser_client::proto::geyser::SubscribeRequestAccountsDataSlice
{
    fn from(request: SubscribeRequestAccountsDataSlice) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestAccountsDataSlice {
            offset: request.offset.get_u64().1,
            length: request.length.get_u64().1,
        }
    }
}

#[napi(object)]
pub struct SubscribeRequestPing {
    pub id: i32,
}

impl From<SubscribeRequestPing> for yellowstone_geyser_client::proto::geyser::SubscribeRequestPing {
    fn from(request: SubscribeRequestPing) -> Self {
        yellowstone_geyser_client::proto::geyser::SubscribeRequestPing { id: request.id }
    }
}

#[napi(object)]
pub struct SubscribeUpdate {
    pub filters: Vec<String>,
    pub created_at: Option<u32>,
    pub account: Option<SubscribeUpdateAccount>,
    pub slot: Option<SubscribeUpdateSlot>,
    pub transaction: Option<SubscribeUpdateTransaction>,
    pub transaction_status: Option<SubscribeUpdateTransactionStatus>,
    pub block: Option<SubscribeUpdateBlock>,
    pub block_meta: Option<SubscribeUpdateBlockMeta>,
    pub entry: Option<SubscribeUpdateEntry>,
    pub ping: Option<SubscribeUpdatePing>,
    pub pong: Option<SubscribeUpdatePong>,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdate> for SubscribeUpdate {
    fn from(update: yellowstone_geyser_client::proto::geyser::SubscribeUpdate) -> Self {
        let mut new_update = SubscribeUpdate {
            filters: update.filters,
            created_at: update.created_at.map(|x| x.seconds as u32),
            account: None,
            slot: None,
            transaction: None,
            transaction_status: None,
            block: None,
            block_meta: None,
            entry: None,
            ping: None,
            pong: None,
        };

        match update.update_oneof {
            Some(UpdateOneof::Account(account)) => {
                new_update.account = Some(account.into());
            }
            Some(UpdateOneof::Slot(slot)) => {
                new_update.slot = Some(slot.into());
            }
            Some(UpdateOneof::Transaction(transaction)) => {
                new_update.transaction = Some(transaction.into());
            }
            Some(UpdateOneof::TransactionStatus(transaction_status)) => {
                new_update.transaction_status = Some(transaction_status.into());
            }
            Some(UpdateOneof::Block(block)) => {
                new_update.block = Some(block.into());
            }
            Some(UpdateOneof::BlockMeta(block_meta)) => {
                new_update.block_meta = Some(block_meta.into());
            }
            Some(UpdateOneof::Entry(entry)) => {
                new_update.entry = Some(entry.into());
            }
            Some(UpdateOneof::Ping(ping)) => {
                new_update.ping = Some(ping.into());
            }
            Some(UpdateOneof::Pong(pong)) => {
                new_update.pong = Some(pong.into());
            }
            None => {}
        }

        new_update
    }
}

#[napi(object)]
pub struct SubscribeUpdateAccount {
    pub account: Option<SubscribeUpdateAccountInfo>,
    pub slot: BigInt,
    pub is_startup: bool,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateAccount>
    for SubscribeUpdateAccount
{
    fn from(update: yellowstone_geyser_client::proto::geyser::SubscribeUpdateAccount) -> Self {
        SubscribeUpdateAccount {
            account: update.account.map(|x| x.into()),
            slot: update.slot.into(),
            is_startup: update.is_startup,
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateAccountInfo {
    pub pubkey: Vec<u8>,
    pub lamports: BigInt,
    pub owner: Vec<u8>,
    pub executable: bool,
    pub rent_epoch: BigInt,
    pub data: Vec<u8>,
    pub write_version: BigInt,
    pub txn_signature: Option<Vec<u8>>,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateAccountInfo>
    for SubscribeUpdateAccountInfo
{
    fn from(info: yellowstone_geyser_client::proto::geyser::SubscribeUpdateAccountInfo) -> Self {
        SubscribeUpdateAccountInfo {
            pubkey: info.pubkey,
            lamports: info.lamports.into(),
            owner: info.owner,
            executable: info.executable,
            rent_epoch: info.rent_epoch.into(),
            data: info.data,
            write_version: info.write_version.into(),
            txn_signature: info.txn_signature,
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateSlot {
    pub slot: BigInt,
    pub parent: Option<BigInt>,
    pub status: i32,
    pub dead_error: Option<String>,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateSlot> for SubscribeUpdateSlot {
    fn from(slot: yellowstone_geyser_client::proto::geyser::SubscribeUpdateSlot) -> Self {
        SubscribeUpdateSlot {
            slot: slot.slot.into(),
            parent: slot.parent.map(|x| x.into()),
            status: slot.status,
            dead_error: slot.dead_error,
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateTransaction {
    pub transaction: Option<SubscribeUpdateTransactionInfo>,
    pub slot: BigInt,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateTransaction>
    for SubscribeUpdateTransaction
{
    fn from(
        transaction: yellowstone_geyser_client::proto::geyser::SubscribeUpdateTransaction,
    ) -> Self {
        SubscribeUpdateTransaction {
            transaction: transaction.transaction.map(|x| x.into()),
            slot: transaction.slot.into(),
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateTransactionInfo {
    pub signature: Vec<u8>,
    pub is_vote: bool,
    pub transaction: Option<Transaction>,
    pub meta: Option<TransactionStatusMeta>,
    pub index: BigInt,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateTransactionInfo>
    for SubscribeUpdateTransactionInfo
{
    fn from(
        info: yellowstone_geyser_client::proto::geyser::SubscribeUpdateTransactionInfo,
    ) -> Self {
        SubscribeUpdateTransactionInfo {
            signature: info.signature,
            is_vote: info.is_vote,
            transaction: info.transaction.map(|x| x.into()),
            meta: info.meta.map(|x| x.into()),
            index: info.index.into(),
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateTransactionStatus {
    pub slot: BigInt,
    pub signature: Vec<u8>,
    pub is_vote: bool,
    pub index: BigInt,
    pub err: Option<TransactionError>,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateTransactionStatus>
    for SubscribeUpdateTransactionStatus
{
    fn from(
        status: yellowstone_geyser_client::proto::geyser::SubscribeUpdateTransactionStatus,
    ) -> Self {
        SubscribeUpdateTransactionStatus {
            slot: status.slot.into(),
            signature: status.signature,
            is_vote: status.is_vote,
            index: status.index.into(),
            err: status.err.map(|x| x.into()),
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateBlock {
    pub slot: BigInt,
    pub blockhash: String,
    pub rewards: Option<Rewards>,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<BlockHeight>,
    pub parent_slot: BigInt,
    pub parent_blockhash: String,
    pub executed_transaction_count: BigInt,
    pub transactions: Vec<SubscribeUpdateTransactionInfo>,
    pub updated_account_count: BigInt,
    pub accounts: Vec<SubscribeUpdateAccountInfo>,
    pub entries_count: BigInt,
    pub entries: Vec<SubscribeUpdateEntry>,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateBlock> for SubscribeUpdateBlock {
    fn from(block: yellowstone_geyser_client::proto::geyser::SubscribeUpdateBlock) -> Self {
        SubscribeUpdateBlock {
            slot: block.slot.into(),
            blockhash: block.blockhash,
            rewards: block.rewards.map(|x| x.into()),
            block_time: block.block_time.map(|x| x.into()),
            block_height: block.block_height.map(|x| x.into()),
            parent_slot: block.parent_slot.into(),
            parent_blockhash: block.parent_blockhash,
            executed_transaction_count: block.executed_transaction_count.into(),
            transactions: block.transactions.into_iter().map(|x| x.into()).collect(),
            updated_account_count: block.updated_account_count.into(),
            accounts: block.accounts.into_iter().map(|x| x.into()).collect(),
            entries_count: block.entries_count.into(),
            entries: block.entries.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateBlockMeta {
    pub slot: BigInt,
    pub blockhash: String,
    pub rewards: Option<Rewards>,
    pub block_time: Option<UnixTimestamp>,
    pub block_height: Option<BlockHeight>,
    pub parent_slot: BigInt,
    pub parent_blockhash: String,
    pub executed_transaction_count: BigInt,
    pub entries_count: BigInt,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateBlockMeta>
    for SubscribeUpdateBlockMeta
{
    fn from(meta: yellowstone_geyser_client::proto::geyser::SubscribeUpdateBlockMeta) -> Self {
        SubscribeUpdateBlockMeta {
            slot: meta.slot.into(),
            blockhash: meta.blockhash,
            rewards: meta.rewards.map(|x| x.into()),
            block_time: meta.block_time.map(|x| x.into()),
            block_height: meta.block_height.map(|x| x.into()),
            parent_slot: meta.parent_slot.into(),
            parent_blockhash: meta.parent_blockhash,
            executed_transaction_count: meta.executed_transaction_count.into(),
            entries_count: meta.entries_count.into(),
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdateEntry {
    pub slot: BigInt,
    pub index: BigInt,
    pub num_hashes: BigInt,
    pub hash: Vec<u8>,
    pub executed_transaction_count: BigInt,
    /// added in v1.18, for solana 1.17 value is always 0
    pub starting_transaction_index: BigInt,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdateEntry> for SubscribeUpdateEntry {
    fn from(entry: yellowstone_geyser_client::proto::geyser::SubscribeUpdateEntry) -> Self {
        SubscribeUpdateEntry {
            slot: entry.slot.into(),
            index: entry.index.into(),
            num_hashes: entry.num_hashes.into(),
            hash: entry.hash,
            executed_transaction_count: entry.executed_transaction_count.into(),
            starting_transaction_index: entry.starting_transaction_index.into(),
        }
    }
}

#[napi(object)]
pub struct SubscribeUpdatePing;

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdatePing> for SubscribeUpdatePing {
    fn from(_ping: yellowstone_geyser_client::proto::geyser::SubscribeUpdatePing) -> Self {
        SubscribeUpdatePing {}
    }
}

#[napi(object)]
pub struct SubscribeUpdatePong {
    pub id: i32,
}

impl From<yellowstone_geyser_client::proto::geyser::SubscribeUpdatePong> for SubscribeUpdatePong {
    fn from(pong: yellowstone_geyser_client::proto::geyser::SubscribeUpdatePong) -> Self {
        SubscribeUpdatePong { id: pong.id }
    }
}
