use crate::enums::{SelfTradeBehavior, Side};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum OrderPacket {
    /// This order type is used to place a limit order on the book.
    /// It will never be matched against other existing limit orders
    PostOnly {
        side: Side,

        /// The price of the order in quote ticks per base unit
        num_quote_ticks_per_base_unit: u64,

        /// Number of base lots to place on the book
        num_base_lots: u64,

        /// Client order id used to identify the order in the response to the client
        client_order_id: u128,

        /// Flag for whether or not to reject the order if it would immediately match or amend it to the best non-crossing price
        /// Default value is true
        reject_post_only: bool,
    },

    /// This order type is used to place a limit order on the book
    /// It can be matched against other existing limit orders, but will posted at the
    /// specified level if it is not matched
    Limit {
        side: Side,

        /// The price of the order in quote ticks per base unit
        num_quote_ticks_per_base_unit: u64,

        /// Total number of base lots to place on the book or fill at a better price
        num_base_lots: u64,

        /// How the matching engine should handle a self trade
        self_trade_behavior: SelfTradeBehavior,

        /// Number of orders to match against. If this is `None` there is no limit
        match_limit: Option<u64>,

        /// Client order id used to identify the order in the response to the client
        client_order_id: u128,
    },

    /// This order type is used to place an order that will be matched against existing resting orders
    /// If the order matches fewer than `min_lots` lots, it will be cancelled.
    ///
    /// Fill or Kill (FOK) orders are a subset of Immediate or Cancel (IOC) orders where either
    /// the `num_lots` is equal to the `min_lots_to_fill` of the order or the `num_ticks` is
    /// equal to the `min_ticks_to_fill` of the order.
    ImmediateOrCancel {
        side: Side,

        /// The most aggressive price an order can be matched at. For example, if there is an IOC buy order
        /// to purchase 10 lots with the tick_per_lot parameter set to 10, then the order will never
        /// be matched at a price higher than 10 quote ticks per base unit. If this value is None, then the order
        /// is treated as a market order.
        num_quote_ticks_per_base_unit: Option<u64>,

        /// The number of base lots to fill against the order book. Either this parameter or the `num_quote_lots`
        /// parameter must be set to a nonzero value.
        num_base_lots: u64,

        /// The number of quote lots to fill against the order book. Either this parameter or the `num_base_lots`
        /// parameter must be set to a nonzero value.
        num_quote_lots: u64,

        /// The minimum number of base lots to fill against the order book. If the order does not fill
        /// this many base lots, it will be voided.
        min_base_lots_to_fill: u64,

        /// The minimum number of quote lots to fill against the order book. If the order does not fill
        /// this many quote lots, it will be voided.
        min_quote_lots_to_fill: u64,

        /// How the matching engine should handle a self trade.
        self_trade_behavior: SelfTradeBehavior,

        /// Number of orders to match against. If set to `None`, there is no limit.
        match_limit: Option<u64>,

        /// Client order id used to identify the order in the program's inner instruction data.
        client_order_id: u128,
    },
}

impl OrderPacket {
    pub fn new_post_only_default(side: Side, price_in_ticks: u64, num_base_lots: u64) -> Self {
        Self::PostOnly {
            side,
            num_quote_ticks_per_base_unit: price_in_ticks,
            num_base_lots: num_base_lots,
            client_order_id: 0,
            reject_post_only: true,
        }
    }

    pub fn new_post_only_default_with_client_order_id(
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        client_order_id: u128,
    ) -> Self {
        Self::PostOnly {
            side,
            num_quote_ticks_per_base_unit: price_in_ticks,
            num_base_lots: num_base_lots,
            client_order_id,
            reject_post_only: true,
        }
    }

    pub fn new_adjustable_post_only_default_with_client_order_id(
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        client_order_id: u128,
    ) -> Self {
        Self::PostOnly {
            side,
            num_quote_ticks_per_base_unit: price_in_ticks,
            num_base_lots: num_base_lots,
            client_order_id,
            reject_post_only: false,
        }
    }

    pub fn new_post_only(
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        reject_post_only: bool,
    ) -> Self {
        Self::PostOnly {
            side,
            num_quote_ticks_per_base_unit: price_in_ticks,
            num_base_lots: num_base_lots,
            client_order_id: 0,
            reject_post_only,
        }
    }

    pub fn new_limit_order_default(side: Side, price_in_ticks: u64, num_base_lots: u64) -> Self {
        Self::new_limit_order(
            side,
            price_in_ticks,
            num_base_lots,
            SelfTradeBehavior::CancelProvide,
            None,
            0,
        )
    }

    pub fn new_limit_order_default_with_client_order_id(
        side: Side,
        price_in_ticks: u64,
        num_lots: u64,
        client_order_id: u128,
    ) -> Self {
        Self::new_limit_order(
            side,
            price_in_ticks,
            num_lots,
            SelfTradeBehavior::CancelProvide,
            None,
            client_order_id,
        )
    }

    pub fn new_limit_order(
        side: Side,
        price_in_ticks: u64,
        num_base_lots: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
    ) -> Self {
        Self::Limit {
            side,
            num_quote_ticks_per_base_unit: price_in_ticks,
            num_base_lots: num_base_lots,
            self_trade_behavior,
            match_limit,
            client_order_id,
        }
    }

    pub fn new_fok_sell_with_limit_price(
        target_price_in_ticks: u64,
        base_lot_budget: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
    ) -> Self {
        Self::new_ioc(
            Side::Ask,
            Some(target_price_in_ticks),
            base_lot_budget,
            0,
            base_lot_budget,
            0,
            self_trade_behavior,
            match_limit,
            client_order_id,
        )
    }

    pub fn new_fok_buy_with_limit_price(
        target_price_in_ticks: u64,
        quote_lot_budget: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
    ) -> Self {
        Self::new_ioc(
            Side::Bid,
            Some(target_price_in_ticks),
            0,
            quote_lot_budget,
            0,
            quote_lot_budget,
            self_trade_behavior,
            match_limit,
            client_order_id,
        )
    }

    pub fn new_ioc_sell_with_limit_price(
        price_in_ticks: u64,
        num_base_lots: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
    ) -> Self {
        Self::new_ioc(
            Side::Ask,
            Some(price_in_ticks),
            num_base_lots,
            0,
            0,
            0,
            self_trade_behavior,
            match_limit,
            client_order_id,
        )
    }

    pub fn new_ioc_buy_with_limit_price(
        price_in_ticks: u64,
        num_quote_lots: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
    ) -> Self {
        Self::new_ioc(
            Side::Bid,
            Some(price_in_ticks),
            0,
            num_quote_lots,
            0,
            0,
            self_trade_behavior,
            match_limit,
            client_order_id,
        )
    }

    pub fn new_ioc_by_lots(
        side: Side,
        price_in_ticks: u64,
        base_lot_budget: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
    ) -> Self {
        Self::new_ioc(
            side,
            Some(price_in_ticks),
            base_lot_budget,
            0,
            0,
            0,
            self_trade_behavior,
            match_limit,
            client_order_id,
        )
    }

    pub fn new_fok_buy_with_slippage_with_client_order_id(
        quote_lots_in: u64,
        min_base_lots_out: u64,
        client_order_id: u128,
    ) -> Self {
        Self::new_ioc(
            Side::Bid,
            None,
            0,
            quote_lots_in,
            min_base_lots_out,
            quote_lots_in,
            SelfTradeBehavior::CancelProvide,
            None,
            client_order_id,
        )
    }

    pub fn new_fok_sell_with_slippage_with_client_order_id(
        base_lots_in: u64,
        min_quote_lots_out: u64,
        client_order_id: u128,
    ) -> Self {
        Self::new_ioc(
            Side::Ask,
            None,
            base_lots_in,
            0,
            base_lots_in,
            min_quote_lots_out,
            SelfTradeBehavior::CancelProvide,
            None,
            client_order_id,
        )
    }

    pub fn new_fok_buy_with_slippage(quote_lots_in: u64, min_base_lots_out: u64) -> Self {
        Self::new_ioc(
            Side::Bid,
            None,
            0,
            quote_lots_in,
            min_base_lots_out,
            quote_lots_in,
            SelfTradeBehavior::CancelProvide,
            None,
            0,
        )
    }

    pub fn new_fok_sell_with_slippage(base_lots_in: u64, min_quote_lots_out: u64) -> Self {
        Self::new_ioc(
            Side::Ask,
            None,
            base_lots_in,
            0,
            base_lots_in,
            min_quote_lots_out,
            SelfTradeBehavior::CancelProvide,
            None,
            0,
        )
    }

    pub fn new_ioc_buy_with_slippage(quote_lots_in: u64, min_base_lots_out: u64) -> Self {
        Self::new_ioc(
            Side::Bid,
            None,
            0,
            quote_lots_in,
            min_base_lots_out,
            0,
            SelfTradeBehavior::CancelProvide,
            None,
            0,
        )
    }

    pub fn new_ioc_sell_with_slippage(base_lots_in: u64, min_quote_lots_out: u64) -> Self {
        Self::new_ioc(
            Side::Ask,
            None,
            base_lots_in,
            0,
            0,
            min_quote_lots_out,
            SelfTradeBehavior::CancelProvide,
            None,
            0,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_ioc(
        side: Side,
        price_in_ticks: Option<u64>,
        num_base_lots: u64,
        num_quote_lots: u64,
        min_base_lots_to_fill: u64,
        min_quote_lots_to_fill: u64,
        self_trade_behavior: SelfTradeBehavior,
        match_limit: Option<u64>,
        client_order_id: u128,
    ) -> Self {
        Self::ImmediateOrCancel {
            side,
            num_quote_ticks_per_base_unit: price_in_ticks,
            num_base_lots: num_base_lots,
            num_quote_lots: num_quote_lots,
            min_base_lots_to_fill: min_base_lots_to_fill,
            min_quote_lots_to_fill: min_quote_lots_to_fill,
            self_trade_behavior,
            match_limit,
            client_order_id,
        }
    }
}
