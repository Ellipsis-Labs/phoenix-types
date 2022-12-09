use crate::market::{FIFOMarket, Market, MarketSizeParams};
use sokoban::node_allocator::ZeroCopy;

/// Struct that holds an object implementing the Market trait.
pub struct MarketWrapperMut<'a> {
    pub inner: &'a mut dyn Market,
}

impl<'a> MarketWrapperMut<'a> {
    pub fn new(market: &'a mut dyn Market) -> Self {
        Self { inner: market }
    }
}

/// Loads a market from a given buffer and known market params.
pub fn load_with_dispatch_mut<'a>(
    market_size_params: &'a MarketSizeParams,
    bytes: &'a mut [u8],
) -> Option<MarketWrapperMut<'a>> {
    dispatch_market_mut(market_size_params, bytes)
}

fn dispatch_market_mut<'a>(
    market_size_params: &'a MarketSizeParams,
    bytes: &'a mut [u8],
) -> Option<MarketWrapperMut<'a>> {
    let market = match (
        market_size_params.bids_size,
        market_size_params.asks_size,
        market_size_params.num_seats,
    ) {
        (512, 512, 256) => FIFOMarket::<512, 512, 256>::load_mut_bytes(bytes)? as &mut dyn Market,
        (2048, 2048, 4096) => {
            FIFOMarket::<2048, 2048, 4096>::load_mut_bytes(bytes)? as &mut dyn Market
        }
        (4096, 4096, 8192) => {
            FIFOMarket::<4096, 4096, 8192>::load_mut_bytes(bytes)? as &mut dyn Market
        }
        (1024, 1024, 128) => {
            FIFOMarket::<1024, 1024, 128>::load_mut_bytes(bytes)? as &mut dyn Market
        }
        (2048, 2048, 128) => {
            FIFOMarket::<2048, 2048, 128>::load_mut_bytes(bytes)? as &mut dyn Market
        }
        (4096, 4096, 128) => {
            FIFOMarket::<4096, 4096, 128>::load_mut_bytes(bytes)? as &mut dyn Market
        }
        _ => {
            println!("Invalid parameters for market");
            return None;
        }
    };
    Some(MarketWrapperMut::new(market))
}

/// Struct that holds an object implementing the Market trait.
pub struct MarketWrapper<'a> {
    pub inner: &'a dyn Market,
}

impl<'a> MarketWrapper<'a> {
    pub fn new(market: &'a dyn Market) -> Self {
        Self { inner: market }
    }
}

/// Loads a market from a given buffer and known market params.
pub fn load_with_dispatch<'a>(
    market_size_params: &'a MarketSizeParams,
    bytes: &'a [u8],
) -> Option<MarketWrapper<'a>> {
    dispatch_market(market_size_params, bytes)
}

fn dispatch_market<'a>(
    market_size_params: &'a MarketSizeParams,
    bytes: &'a [u8],
) -> Option<MarketWrapper<'a>> {
    let market = match (
        market_size_params.bids_size,
        market_size_params.asks_size,
        market_size_params.num_seats,
    ) {
        (512, 512, 256) => FIFOMarket::<512, 512, 256>::load_bytes(bytes)? as &dyn Market,
        (2048, 2048, 4096) => FIFOMarket::<2048, 2048, 4096>::load_bytes(bytes)? as &dyn Market,
        (4096, 4096, 8192) => FIFOMarket::<4096, 4096, 8192>::load_bytes(bytes)? as &dyn Market,
        (1024, 1024, 128) => FIFOMarket::<1024, 1024, 128>::load_bytes(bytes)? as &dyn Market,
        (2048, 2048, 128) => FIFOMarket::<2048, 2048, 128>::load_bytes(bytes)? as &dyn Market,
        (4096, 4096, 128) => FIFOMarket::<4096, 4096, 128>::load_bytes(bytes)? as &dyn Market,
        _ => {
            println!("Invalid parameters for market");
            return None;
        }
    };
    Some(MarketWrapper::new(market))
}

/// Returns the size of a market in bytes, given the market params.
pub fn get_market_size(market_params: &MarketSizeParams) -> Option<usize> {
    let size = match (
        market_params.bids_size,
        market_params.asks_size,
        market_params.num_seats,
    ) {
        (512, 512, 256) => std::mem::size_of::<FIFOMarket<512, 512, 256>>(),
        (2048, 2048, 4096) => std::mem::size_of::<FIFOMarket<2048, 2048, 4096>>(),
        (4096, 4096, 8192) => std::mem::size_of::<FIFOMarket<4096, 4096, 8192>>(),
        (1024, 1024, 128) => std::mem::size_of::<FIFOMarket<1024, 1024, 128>>(),
        (2048, 2048, 128) => std::mem::size_of::<FIFOMarket<2048, 2048, 128>>(),
        (4096, 4096, 128) => std::mem::size_of::<FIFOMarket<4096, 4096, 128>>(),
        _ => return None,
    };
    Some(size)
}
