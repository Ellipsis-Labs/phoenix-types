use borsh::{BorshDeserialize, BorshSerialize};
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

/// Options for an order's self trade behavior.
#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum SelfTradeBehavior {
    /// If an order would cross a limit order with the same maker, the crossing order will be rejected.
    Abort,

    /// If an order would cross a limit order with the same maker, the resting limit order will be cancelled.
    CancelProvide,

    /// If an order would cross a limit order with the same maker, the crossing order and resting limit order
    /// will be decreased in size by the smaller of the two quantities.
    DecrementTake,
}

/// Options for an order's side.
#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Side {
    Bid,
    Ask,
}

impl Side {
    /// Returns the side of an order, given the order_sequence_number.
    pub fn from_order_sequence_number(order_sequence_number: u64) -> Self {
        match order_sequence_number.leading_zeros() {
            0 => Side::Bid,
            _ => Side::Ask,
        }
    }
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl Side {
    /// Returns the opposite side.
    pub fn opposite(&self) -> Self {
        match *self {
            Side::Bid => Side::Ask,
            Side::Ask => Side::Bid,
        }
    }
    // cfg_attr doesn't work for staticmethod yet
    #[cfg(feature = "pyo3")]
    #[pyo3(name = "from_order_sequence_number")]
    #[staticmethod]
    pub fn py_from_order_sequence_number(order_sequence_number: u64) -> Self {
        Self::from_order_sequence_number(order_sequence_number)
    }
}
