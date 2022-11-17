use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize)]
pub struct AuditLogHeader {
    pub instruction: u8,
    pub sequence_number: u64,
    pub timestamp: i64,
    pub slot: u64,
    pub market: Pubkey,
    pub signer: Pubkey,
    pub total_events: u16,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AuditLog {
    pub chunk_index: u8,
    pub num_events: u8,
}

#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize)]
pub enum MarketEvent {
    Uninitialized,
    Header {
        header: AuditLogHeader,
    },
    Fill {
        index: u16,
        maker_id: Pubkey,
        order_sequence_number: u64,
        price_in_ticks: u64,
        base_lots_filled: u64,
        base_lots_remaining: u64,
    },
    Place {
        index: u16,
        order_sequence_number: u64,
        client_order_id: u128,
        price_in_ticks: u64,
        base_lots_placed: u64,
    },
    Reduce {
        index: u16,
        order_sequence_number: u64,
        price_in_ticks: u64,
        base_lots_removed: u64,
        base_lots_remaining: u64,
    },
    Evict {
        index: u16,
        maker_id: Pubkey,
        order_sequence_number: u64,
        price_in_ticks: u64,
        base_lots_evicted: u64,
    },
    FillSummary {
        index: u16,
        client_order_id: u128,
        total_base_lots_filled: u64,
        total_quote_lots_filled: u64,
        total_fee_in_quote_lots: u64,
    },
}
