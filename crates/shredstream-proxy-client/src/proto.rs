pub mod shared {
    tonic::include_proto!("shared");
}
pub mod shredstream {
    tonic::include_proto!("shredstream");
}

pub use shredstream::{
    CommitmentLevel as ShredstreamCommitmentLevel, Entry as ShredstreamEntry,
    Heartbeat as ShredstreamHeartbeat, HeartbeatResponse as ShredstreamHeartbeatResponse,
    SubscribeEntriesRequest as ShredstreamSubscribeEntriesRequest,
    SubscribeRequestFilterAccounts as ShredstreamSubscribeRequestFilterAccounts,
    SubscribeRequestFilterAccountsFilter as ShredstreamSubscribeRequestFilterAccountsFilter,
    SubscribeRequestFilterAccountsFilterLamports as ShredstreamSubscribeRequestFilterAccountsFilterLamports,
    SubscribeRequestFilterAccountsFilterMemcmp as ShredstreamSubscribeRequestFilterAccountsFilterMemcmp,
    SubscribeRequestFilterSlots as ShredstreamSubscribeRequestFilterSlots,
    SubscribeRequestFilterTransactions as ShredstreamSubscribeRequestFilterTransactions,
    TraceShred as ShredstreamTraceShred,
};
