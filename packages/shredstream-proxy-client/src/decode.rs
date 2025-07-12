use napi::bindgen_prelude::Uint8Array;
use solana_entry_decoder::decode_entries;

#[napi(js_name = "decodeEntries")]
pub fn node_decode_entries(data: Uint8Array) -> Vec<crate::types::Entry> {
    decode_entries(data.to_vec())
        .unwrap()
        .into_iter()
        .map(|entry| entry.into())
        .collect()
}
