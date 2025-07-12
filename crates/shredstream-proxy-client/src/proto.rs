pub mod shared {
    tonic::include_proto!("shared");
}
pub mod shredstream {
    tonic::include_proto!("shredstream");
}

pub use shredstream::{
    CommitmentLevel, Entry, Heartbeat, HeartbeatResponse, SubscribeEntriesRequest,
    SubscribeRequestFilterAccounts, SubscribeRequestFilterAccountsFilter,
    SubscribeRequestFilterAccountsFilterLamports, SubscribeRequestFilterAccountsFilterMemcmp,
    SubscribeRequestFilterSlots, SubscribeRequestFilterTransactions, TraceShred,
};
