use std::error::Error;

pub fn decode_entries(data: Vec<u8>) -> Result<Vec<solana_entry::entry::Entry>, Box<dyn Error>> {
    let entries: Vec<solana_entry::entry::Entry> = bincode::deserialize(&data)?;

    Ok(entries)
}
