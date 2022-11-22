use crate::enums::Side;
use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use sokoban::node_allocator::{NodeAllocatorMap, OrderedNodeAllocatorMap, ZeroCopy, SENTINEL};
use sokoban::RedBlackTree;
use solana_sdk::pubkey::Pubkey;
#[cfg(feature = "pyo3")]
use {
    pyo3::prelude::*,
    solders_macros::common_methods,
    solders_traits::{
        py_from_bytes_general_via_bincode, pybytes_general_via_bincode, CommonMethods,
        PyBytesBincode, PyFromBytesBincode,
    },
};

/// Representation of an order on the book.
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LadderOrder {
    /// The limit price of the order, in quote ticks per base unit.
    pub price_in_ticks: u64,

    /// The quantity of the order, in base lots.
    pub size_in_base_lots: u64,
}

#[cfg(feature = "pyo3")]
macro_rules! common_methods_boilerplate {
    ($name:ident) => {
        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}", self)
            }
        }
        pybytes_general_via_bincode!($name);
        py_from_bytes_general_via_bincode!($name);
        impl CommonMethods<'_> for $name {}
    };
}

#[cfg(feature = "pyo3")]
common_methods_boilerplate!(LadderOrder);

#[cfg(feature = "pyo3")]
#[common_methods]
#[pymethods]
impl LadderOrder {
    #[new]
    pub fn new(price_in_ticks: u64, size_in_base_lots: u64) -> Self {
        Self {
            price_in_ticks,
            size_in_base_lots,
        }
    }
}

/// Representation of an order book.
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Ladder {
    /// The bids on the book.
    pub bids: Vec<LadderOrder>,

    /// The asks on the book.
    pub asks: Vec<LadderOrder>,
}

#[cfg(feature = "pyo3")]
common_methods_boilerplate!(Ladder);

#[cfg(feature = "pyo3")]
#[common_methods]
#[pymethods]
impl Ladder {
    #[new]
    pub fn new(bids: Vec<LadderOrder>, asks: Vec<LadderOrder>) -> Self {
        Self { bids, asks }
    }
}

pub trait Market {
    fn get_ladder(&self, levels: u64) -> Ladder {
        let mut bids = vec![];
        let mut asks = vec![];

        if levels == 0 {
            return Ladder { bids, asks };
        }

        for (side, book) in [(Side::Bid, &mut bids), (Side::Ask, &mut asks)].iter_mut() {
            for (key, order) in self.get_book(*side).iter() {
                let price = key.num_quote_ticks_per_base_unit;
                let size = order.num_base_lots;
                if book.is_empty() {
                    book.push(LadderOrder {
                        price_in_ticks: price,
                        size_in_base_lots: size,
                    });
                } else {
                    let last = book.last().unwrap();
                    if last.price_in_ticks == price {
                        book.last_mut().unwrap().size_in_base_lots += size;
                    } else {
                        if book.len() as u64 == levels {
                            break;
                        }
                        book.push(LadderOrder {
                            price_in_ticks: price,
                            size_in_base_lots: size,
                        });
                    }
                }
            }
        }

        Ladder { bids, asks }
    }

    fn get_registered_traders(&self) -> &dyn OrderedNodeAllocatorMap<Pubkey, TraderState>;

    fn get_quote_lots_per_tick(&self) -> u64;

    fn get_base_lots_per_base_unit(&self) -> u64;

    fn get_trader_address(&self, trader: &Pubkey) -> Option<u32>;

    fn get_trader_state(&self, trader: &Pubkey) -> Option<&TraderState>;

    fn get_book(&self, side: Side) -> &dyn OrderedNodeAllocatorMap<FIFOOrderId, FIFORestingOrder>;
}

/// Struct representing a market's header.
#[derive(Debug, Clone, Copy, BorshDeserialize, BorshSerialize, Zeroable, Pod)]
#[repr(C)]
pub struct MarketHeader {
    pub discriminant: u64,

    pub status: u64,

    /// The size params of the market.
    pub market_params: MarketParams,

    /// The specification of the base token of the market.
    pub base_params: TokenParams,

    /// The lot size of the base token of the market, in base atoms.
    base_lot_size: u64,

    /// The specification of the quote token of the market.
    pub quote_params: TokenParams,

    /// The lot size of the quote token of the market, in quote atoms.
    quote_lot_size: u64,

    /// The number of quote lots per tick in the market.
    tick_size: u64,

    /// The Pubkey of the market authority.
    pub authority: Pubkey,

    /// The Pubkey of the fee destination.
    pub fee_destination: Pubkey,

    /// The sequence number of the market.
    pub market_sequence_number: u64,

    pub successor: Pubkey,
    _padding1: u64,
    _padding2: u64,
}
impl ZeroCopy for MarketHeader {}

impl MarketHeader {
    pub fn price_in_ticks(&self, price: u64) -> u64 {
        price / self.tick_size
    }

    pub fn get_base_lot_size(&self) -> u64 {
        self.base_lot_size
    }

    pub fn get_quote_lot_size(&self) -> u64 {
        self.quote_lot_size
    }

    pub fn get_tick_size(&self) -> u64 {
        self.tick_size
    }
}

/// Struct representing a market that matches by price-time priority.
#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable)]
pub struct FIFOMarket<const BIDS_SIZE: usize, const ASKS_SIZE: usize, const NUM_SEATS: usize> {
    /// Number of base lots in a base unit. For example, if the lot size is 0.001 SOL, then base_lots_per_base_unit is 1000.
    pub base_lots_per_base_unit: u64,

    /// Tick size in terms of quote lots. For example, if the tick size is 0.01 USDC and the quote lot size is 0.001 USDC, then quote_lots_per_tick is 10.
    pub quote_lots_per_tick: u64,

    /// The sequence number of the next event.
    order_sequence_number: u64,

    /// There are no maker fees. Taker fees are charged on the amount of the trade in basis points.
    pub taker_fee_bps: u64,

    /// Amount of fees collected from the market in its lifetime, in adjusted quote lots. Adjusted quote lots = quote lots * base lots per base unit.
    collected_adjusted_quote_lot_fees: u64,

    /// Amount of unclaimed fees accrued to the market, in adjusted quote lots. Adjusted quote lots = quote lots * base lots per base unit.
    unclaimed_adjusted_quote_lot_fees: u64,

    /// Red-black tree representing the bids in the order book.
    pub bids: RedBlackTree<FIFOOrderId, FIFORestingOrder, BIDS_SIZE>,

    /// Red-black tree representing the asks in the order book.
    pub asks: RedBlackTree<FIFOOrderId, FIFORestingOrder, ASKS_SIZE>,

    /// Red-black tree representing the authorized makers in the market.
    pub traders: RedBlackTree<Pubkey, TraderState, NUM_SEATS>,
}

unsafe impl<const BIDS_SIZE: usize, const ASKS_SIZE: usize, const NUM_SEATS: usize> Pod
    for FIFOMarket<BIDS_SIZE, ASKS_SIZE, NUM_SEATS>
{
}

impl<const BIDS_SIZE: usize, const ASKS_SIZE: usize, const NUM_SEATS: usize> ZeroCopy
    for FIFOMarket<BIDS_SIZE, ASKS_SIZE, NUM_SEATS>
{
}

impl<const BIDS_SIZE: usize, const ASKS_SIZE: usize, const NUM_SEATS: usize> Market
    for FIFOMarket<BIDS_SIZE, ASKS_SIZE, NUM_SEATS>
{
    #[inline(always)]
    fn get_trader_address(&self, trader: &Pubkey) -> Option<u32> {
        let addr = self.traders.get_addr(trader);
        if addr == SENTINEL {
            None
        } else {
            Some(addr)
        }
    }

    #[inline(always)]
    fn get_trader_state(&self, trader: &Pubkey) -> Option<&TraderState> {
        self.traders.get(trader)
    }

    #[inline(always)]
    fn get_book(&self, side: Side) -> &dyn OrderedNodeAllocatorMap<FIFOOrderId, FIFORestingOrder> {
        match side {
            Side::Bid => &self.bids as &dyn OrderedNodeAllocatorMap<FIFOOrderId, FIFORestingOrder>,
            Side::Ask => &self.asks as &dyn OrderedNodeAllocatorMap<FIFOOrderId, FIFORestingOrder>,
        }
    }

    fn get_base_lots_per_base_unit(&self) -> u64 {
        self.base_lots_per_base_unit
    }

    fn get_quote_lots_per_tick(&self) -> u64 {
        self.quote_lots_per_tick
    }

    fn get_registered_traders(&self) -> &dyn OrderedNodeAllocatorMap<Pubkey, TraderState> {
        &self.traders as &dyn OrderedNodeAllocatorMap<Pubkey, TraderState>
    }
}

/// Struct representing the size parameters of a market.
#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize, Zeroable, Pod)]
#[repr(C)]
pub struct MarketParams {
    pub bids_size: u64,
    pub asks_size: u64,
    pub num_seats: u64,
}
impl ZeroCopy for MarketParams {}

/// Struct representing the parameters for a token.
#[derive(Debug, Copy, Clone, BorshDeserialize, BorshSerialize, Zeroable, Pod)]
#[repr(C)]
pub struct TokenParams {
    /// Number of decimals for the token (e.g. 9 for SOL, 6 for USDC).
    pub decimals: u32,

    /// Bump used for generating the PDA for the market's token vault.
    pub vault_bump: u32,

    /// Pubkey of the token mint.
    pub mint_key: Pubkey,

    /// Pubkey of the token vault.
    pub vault_key: Pubkey,
}
impl ZeroCopy for TokenParams {}

/// Struct representing the state of a trader's seat in a market.
#[derive(Debug, Clone, Copy, BorshDeserialize, BorshSerialize, Zeroable, Pod)]
#[repr(C)]
pub struct Seat {
    pub discriminant: u64,
    pub market: Pubkey,
    pub trader: Pubkey,
    pub approval_status: u64,
}

impl ZeroCopy for Seat {}

/// Struct representing an order's key in the order book. It is a combination of the order's price and the order's sequence number.
#[cfg_attr(feature = "pyo3", pyclass(get_all, set_all))]
#[repr(C)]
#[derive(Eq, PartialEq, Debug, Default, Copy, Clone, Zeroable, Pod, Serialize, Deserialize)]
pub struct FIFOOrderId {
    /// This is equivalent to price of an order, in quote ticks per base unit. Each market has a designated
    /// tick size (some number of quote lots) that is used to convert the price to quote ticks per base unit.
    /// For example, if the tick size is 0.01, then a price of 1.23 is converted to 123 quote ticks per
    /// base unit. If the quote lot size is 0.001, this means that there is a spacing of 10 quote lots
    /// in between each tick.
    pub num_quote_ticks_per_base_unit: u64,

    /// This is the unique identifier of the order, which is used to determine the side of the order.
    /// It is derived from the sequence number of the market.
    ///
    /// If the order is a bid, the sequence number will have its bits inverted, and if it is an ask,
    /// the sequence number will be used as is.
    ///
    /// The way to identify the side of the order is to check the leading bit of `order_id`.
    /// A leading bit of 0 indicates an ask, and a leading bit of 1 indicates a bid. See Side::from_order_id.
    pub order_sequence_number: u64,
}

#[cfg(feature = "pyo3")]
common_methods_boilerplate!(FIFOOrderId);

#[cfg(feature = "pyo3")]
#[common_methods]
#[pymethods]
impl FIFOOrderId {
    #[pyo3(name = "new_from_untyped")]
    #[staticmethod]
    pub fn py_new_from_untyped(
        num_quote_ticks_per_base_unit: u64,
        order_sequence_number: u64,
    ) -> Self {
        Self::new_from_untyped(num_quote_ticks_per_base_unit, order_sequence_number)
    }

    #[new]
    pub fn py_new(num_quote_ticks_per_base_unit: u64, order_sequence_number: u64) -> Self {
        Self::new(num_quote_ticks_per_base_unit, order_sequence_number)
    }
}

impl FIFOOrderId {
    pub fn new_from_untyped(
        num_quote_ticks_per_base_unit: u64,
        order_sequence_number: u64,
    ) -> Self {
        FIFOOrderId {
            num_quote_ticks_per_base_unit,
            order_sequence_number,
        }
    }

    pub fn new(num_quote_ticks_per_base_unit: u64, order_sequence_number: u64) -> Self {
        FIFOOrderId {
            num_quote_ticks_per_base_unit,
            order_sequence_number,
        }
    }
}

impl PartialOrd for FIFOOrderId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // The order of the orders is determined by the price of the order. If the price is the same,
        // then the order with the lower sequence number is considered to be the lower order.
        //
        // Asks are sorted in ascending order, and bids are sorted in descending order.
        let (tick_cmp, seq_cmp) = match Side::from_order_sequence_number(self.order_sequence_number)
        {
            Side::Bid => (
                other
                    .num_quote_ticks_per_base_unit
                    .partial_cmp(&self.num_quote_ticks_per_base_unit)?,
                other
                    .order_sequence_number
                    .partial_cmp(&self.order_sequence_number)?,
            ),
            Side::Ask => (
                self.num_quote_ticks_per_base_unit
                    .partial_cmp(&other.num_quote_ticks_per_base_unit)?,
                self.order_sequence_number
                    .partial_cmp(&other.order_sequence_number)?,
            ),
        };
        if tick_cmp == std::cmp::Ordering::Equal {
            Some(seq_cmp)
        } else {
            Some(tick_cmp)
        }
    }
}

impl Ord for FIFOOrderId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct FIFORestingOrder {
    pub trader_index: u64,
    pub num_base_lots: u64,
}

impl FIFORestingOrder {
    pub fn new(trader_index: u64, num_base_lots: u64) -> Self {
        FIFORestingOrder {
            trader_index,
            num_base_lots,
        }
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Zeroable, Pod)]
pub struct TraderState {
    pub adjusted_quote_lots_locked: u64,
    pub adjusted_quote_lots_free: u64,
    pub base_lots_locked: u64,
    pub base_lots_free: u64,
}
