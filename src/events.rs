use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

/// Struct representing metadata about a set of events from a single market instruction.
#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize)]
pub struct AuditLogHeader {
    /// The enum number value of the instruction that generated this log.
    pub instruction: u8,

    /// The market sequence number at the time of the instruction.
    pub market_sequence_number: u64,

    /// The timestamp of the instruction.
    pub timestamp: i64,

    /// The slot of the instruction.
    pub slot: u64,

    /// The Pubkey of the market the log is for.
    pub market: Pubkey,

    /// The Pubkey of the account that generated the log.
    pub signer: Pubkey,

    /// The number of events in the log.
    pub total_events: u16,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct AuditLog {
    pub chunk_index: u8,
    pub num_events: u8,
}

/// Enum representing the different types of events that can be logged.
#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize)]
pub enum MarketEvent {
    Uninitialized,

    /// Header consisting of metadata about a set of events.
    Header {
        header: AuditLogHeader,
    },

    /// Represents a single cross order matching a single resting limit order.
    Fill {
        /// Index of the event in the list of events.
        index: u16,

        /// The Pubkey of the maker whose order was filled.
        maker_id: Pubkey,

        /// The order sequence number of the order that was filled.
        order_sequence_number: u64,

        /// The price of the fill, in ticks.
        price_in_ticks: u64,

        /// The amount filled, in base lots.
        base_lots_filled: u64,

        /// The amount left in the resting order, in base lots.
        base_lots_remaining: u64,
    },

    /// Represents a single limit order being placed.
    Place {
        /// Index of the event in the list of events.
        index: u16,

        /// The order sequence number of the order that was placed.
        order_sequence_number: u64,

        /// The client order id.
        client_order_id: u128,

        /// The price of the fill, in ticks.
        price_in_ticks: u64,

        /// The amount placed, in base lots.
        base_lots_placed: u64,
    },

    /// Represents a single limit order that was reduced or cancelled.
    Reduce {
        /// Index of the event in the list of events.
        index: u16,

        /// The order sequence number of the order that was reduced or cancelled.
        order_sequence_number: u64,

        /// The price of the order that was reduced or cancelled.
        price_in_ticks: u64,

        /// The amount reduced, in base lots.
        base_lots_removed: u64,

        /// The amount left in the resting order, in base lots. 0 if the order was cancelled.
        base_lots_remaining: u64,
    },

    /// Represents a single limit order that was evicted. The least aggressive order in the book is
    /// evicted when a new order is placed that would cause the book to exceed its capacity.
    Evict {
        /// Index of the event in the list of events.
        index: u16,

        /// The Pubkey of the maker whose order was evicted.
        maker_id: Pubkey,

        /// The order sequence number of the order that was evicted.
        order_sequence_number: u64,

        /// The price of the order that was evicted, in ticks.
        price_in_ticks: u64,

        /// The amount of the order that was evicted, in base lots.
        base_lots_evicted: u64,
    },

    /// Represents the total amount filled for a cross order.
    FillSummary {
        /// Index of the event in the list of events.
        index: u16,

        /// The client order id.
        client_order_id: u128,

        /// The total amount filled, in base lots.
        total_base_lots_filled: u64,

        /// The total amount filled, in quote lots.
        total_quote_lots_filled: u64,

        /// The total amount of fees paid, in quote lots.
        total_fee_in_quote_lots: u64,
    },
}
