use napi::bindgen_prelude::Uint8Array;
use std::error::Error;

pub fn decode_entries(data: Vec<u8>) -> Result<Vec<solana_entry::entry::Entry>, Box<dyn Error>> {
    let entries: Vec<solana_entry::entry::Entry> = bincode::deserialize(&data)?;

    Ok(entries)
}

#[napi(js_name = "decodeEntries")]
pub fn node_decode_entries(data: Uint8Array) -> Vec<crate::types::Entry> {
    decode_entries(data.to_vec())
        .unwrap()
        .into_iter()
        .map(|entry| entry.into())
        .collect()
}
