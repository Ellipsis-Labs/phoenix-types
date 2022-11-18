use borsh::{BorshDeserialize, BorshSerialize};
#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum SelfTradeBehavior {
    Abort,
    CancelProvide,
    DecrementTake,
}

#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Side {
    Bid,
    Ask,
}

impl Side {
    pub fn from_order_sequence_number(order_id: u64) -> Self {
        match order_id.leading_zeros() {
            0 => Side::Bid,
            _ => Side::Ask,
        }
    }
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl Side {
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
    pub fn py_from_order_sequence_number(order_id: u64) -> Self {
        Self::from_order_sequence_number(order_id)
    }
}
