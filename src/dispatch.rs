use crate::market::{FIFOMarket, Market, MarketParams};
use sokoban::node_allocator::ZeroCopy;

pub struct MarketWrapper<'a> {
    pub inner: &'a mut dyn Market,
}

impl<'a> MarketWrapper<'a> {
    pub fn new(market: &'a mut dyn Market) -> Self {
        Self { inner: market }
    }
}

pub fn load_with_dispatch_mut<'a>(
    header: &'a MarketParams,
    bytes: &'a mut [u8],
) -> Option<MarketWrapper> {
    dispatch_market_mut(header, bytes)
}

pub fn dispatch_market_mut<'a>(
    header: &'a MarketParams,
    bytes: &'a mut [u8],
) -> Option<MarketWrapper> {
    let market = match (header.bids_size, header.asks_size, header.num_seats) {
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
    Some(MarketWrapper::new(market))
}

pub fn get_market_size(header: &MarketParams) -> Option<usize> {
    let size = match (header.bids_size, header.asks_size, header.num_seats) {
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
