use borsh::{BorshDeserialize, BorshSerialize};

/// Struct to send a vector of bids and asks as PostOnly orders in a single packet.
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MultipleOrderPacket {
    /// Bids and asks are in the format (price in ticks, size in base lots)
    pub bids: Vec<(u64, u64)>,
    pub asks: Vec<(u64, u64)>,
    pub client_order_id: Option<u128>,
    pub reject_post_only: bool,
}

impl MultipleOrderPacket {
    pub fn new(
        bids: Vec<(u64, u64)>,
        asks: Vec<(u64, u64)>,
        client_order_id: Option<u128>,
        reject_post_only: bool,
    ) -> Self {
        MultipleOrderPacket {
            bids,
            asks,
            client_order_id,
            reject_post_only,
        }
    }

    pub fn new_default(bids: Vec<(u64, u64)>, asks: Vec<(u64, u64)>) -> Self {
        MultipleOrderPacket {
            bids,
            asks,
            client_order_id: None,
            reject_post_only: true,
        }
    }
}