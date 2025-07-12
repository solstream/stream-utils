use std::error::Error;

pub fn decode_shredstream_entry(
    data: crate::proto::Entry,
) -> Result<DecodedShredstreamEntry, Box<dyn Error>> {
    let entries: Vec<solana_entry::entry::Entry> = bincode::deserialize(&data.entries)?;

    Ok(DecodedShredstreamEntry {
        slot: data.slot,
        entries,
    })
}

pub struct DecodedShredstreamEntry {
    pub slot: u64,
    pub entries: Vec<solana_entry::entry::Entry>,
}
