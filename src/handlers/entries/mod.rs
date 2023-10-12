use serde::{Deserialize, Serialize};
use starknet::core::types::FieldElement;

pub use create_entry::create_entry;
pub use get_entry::get_entry;

mod create_entry;
mod get_entry;

#[derive(Debug, Deserialize)]
pub struct CreateEntryRequest {
    signature: Vec<FieldElement>,
    publisher: String,
    source: String,
    pair_id: String,
    timestamp: u64,
    price: u128,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntryResponse {
    pair_id: String,
    timestamp: u64,
    num_sources_aggregated: u8,
    price: u128,
}
