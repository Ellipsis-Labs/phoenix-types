use crate::order_packet::OrderPacket;
use crate::{enums::Side, phoenix_log_authority};
use borsh::{BorshDeserialize, BorshSerialize};
use num_enum::TryFromPrimitive;
use shank::ShankInstruction;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};
use spl_associated_token_account::get_associated_token_address;

pub fn get_vault_address(market: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"vault", market.as_ref(), mint.as_ref()], &crate::ID)
}

pub fn get_seat_address(market: &Pubkey, trader: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[b"seat", market.as_ref(), trader.as_ref()], &crate::ID)
}

#[repr(u8)]
#[derive(TryFromPrimitive, Debug, Copy, Clone, ShankInstruction, PartialEq, Eq)]
#[rustfmt::skip]
pub enum PhoenixInstruction {
    // Market instructions
    /// Send a swap (no limit orders allowed) order
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    Swap = 0,

    /// Send a swap (no limit orders allowed) order using only deposited funds
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, name = "seat")]
    SwapWithFreeFunds = 1,

    /// Place a limit order on the book. The order can cross if the supplied order type is Limit
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, name = "seat")]
    #[account(5, writable, name = "base_account", desc = "Trader base token account")]
    #[account(6, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(7, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(8, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(9, name = "token_program", desc = "Token program")]
    PlaceLimitOrder = 2,

    /// Place a limit order on the book using only deposited funds.
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, name = "seat")]
    PlaceLimitOrderWithFreeFunds = 3,

    /// Reduce the size of an existing order on the book 
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    ReduceOrder = 4,

    /// Reduce the size of an existing order on the book 
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    ReduceOrderWithFreeFunds = 5,


    /// Cancel all orders 
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    CancelAllOrders = 6,

    /// Cancel all orders (no token transfers) 
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    CancelAllOrdersWithFreeFunds = 7,

    /// Cancel all orders more aggressive than a specified price
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    CancelUpTo = 8,


    /// Cancel all orders more aggressive than a specified price (no token transfers) 
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    CancelUpToWithFreeFunds = 9,

    /// Cancel multiple orders by ID 
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    CancelMultipleOrdersById = 10,

    /// Cancel multiple orders by ID (no token transfers) 
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    CancelMultipleOrdersByIdWithFreeFunds = 11,

    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    WithdrawFunds = 12,

    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, name = "seat")]
    #[account(5, writable, name = "base_account", desc = "Trader base token account")]
    #[account(6, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(7, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(8, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(9, name = "token_program", desc = "Token program")]
    DepositFunds = 13,

    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "payer")]
    #[account(4, writable, name = "seat")]
    #[account(5, name = "system_program", desc = "System program")]
    RequestSeat = 14,

    #[account(0, signer, name = "log_authority", desc = "Log authority")]
    Log = 15,

    /// Place multiple post only orders on the book.
    /// Similar to single post only orders, these can either be set to be rejected or amended to top of book if they cross.
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, name = "seat")]
    #[account(5, writable, name = "base_account", desc = "Trader base token account")]
    #[account(6, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(7, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(8, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(9, name = "token_program", desc = "Token program")]
    PlaceMultiplePostOnlyOrders = 16,
        
    /// Place multiple post only orders on the book using only deposited funds.
    /// Similar to single post only orders, these can either be set to be rejected or amended to top of book if they cross.
    #[account(0, name = "phoenix_program", desc = "Phoenix program")]
    #[account(1, name = "log_authority", desc = "Phoenix log authority")]
    #[account(2, writable, name = "market", desc = "This account holds the market state")]
    #[account(3, writable, signer, name = "trader")]
    #[account(4, name = "seat")]
    PlaceMultiplePostOnlyOrdersWithFreeFunds = 17,
}

impl PhoenixInstruction {
    pub fn to_vec(&self) -> Vec<u8> {
        vec![*self as u8]
    }
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct CancelOrderParams {
    pub side: Side,
    pub price_in_ticks: u64,
    pub order_sequence_number: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct ReduceOrderParams {
    base_params: CancelOrderParams,
    size: u64,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct CancelUpToParams {
    pub side: Side,
    pub tick_limit: Option<u64>,
    pub num_orders_to_search: Option<u32>,
    pub num_orders_to_cancel: Option<u32>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub struct CancelMultipleOrdersByIdParams {
    pub orders: Vec<CancelOrderParams>,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct DepositParams {
    pub quote_lots: u64,
    pub base_lots: u64,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, BorshDeserialize, BorshSerialize)]
pub struct WithdrawParams {
    pub quote_lots_to_withdraw: Option<u64>,
    pub base_lots_to_withdraw: Option<u64>,
}

/// Struct to send a vector of bids and asks as PostOnly orders in a single packet.
#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct MultipleOrderPacket {
    pub bids: Vec<CondensedOrder>,
    pub asks: Vec<CondensedOrder>,
    pub client_order_id: Option<u128>,
    pub reject_post_only: bool,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct CondensedOrder {
    pub price_in_ticks: u64,
    pub size_in_base_lots: u64,
}
/// Helpers for creating MultipleOrderPacket from vectors of u64 (price in ticks, size in base lots)
impl MultipleOrderPacket {
    pub fn new(
        bids: Vec<(u64, u64)>,
        asks: Vec<(u64, u64)>,
        client_order_id: Option<u128>,
        reject_post_only: bool,
    ) -> Self {
        MultipleOrderPacket {
            bids: bids
                .iter()
                .map(|(p, s)| CondensedOrder {
                    price_in_ticks: *p,
                    size_in_base_lots: *s,
                })
                .collect(),
            asks: asks
                .iter()
                .map(|(p, s)| CondensedOrder {
                    price_in_ticks: *p,
                    size_in_base_lots: *s,
                })
                .collect(),
            client_order_id,
            reject_post_only,
        }
    }

    pub fn new_default(bids: Vec<(u64, u64)>, asks: Vec<(u64, u64)>) -> Self {
        MultipleOrderPacket {
            bids: bids
                .iter()
                .map(|(p, s)| CondensedOrder {
                    price_in_ticks: *p,
                    size_in_base_lots: *s,
                })
                .collect(),
            asks: asks
                .iter()
                .map(|(p, s)| CondensedOrder {
                    price_in_ticks: *p,
                    size_in_base_lots: *s,
                })
                .collect(),
            client_order_id: None,
            reject_post_only: true,
        }
    }
}

pub fn create_new_order_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    order_type: &OrderPacket,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_new_order_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        order_type,
    )
}

pub fn create_new_order_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    order_type: &OrderPacket,
) -> Instruction {
    let (base_vault, _) = get_vault_address(market, base);
    let (quote_vault, _) = get_vault_address(market, quote);
    if order_type.is_take_only() {
        Instruction {
            program_id: crate::id(),
            accounts: vec![
                AccountMeta::new_readonly(crate::id(), false),
                AccountMeta::new_readonly(phoenix_log_authority::id(), false),
                AccountMeta::new(*market, false),
                AccountMeta::new(*trader, true),
                AccountMeta::new(*base_account, false),
                AccountMeta::new(*quote_account, false),
                AccountMeta::new(base_vault, false),
                AccountMeta::new(quote_vault, false),
                AccountMeta::new_readonly(spl_token::id(), false),
            ],
            data: [
                PhoenixInstruction::Swap.to_vec(),
                order_type.try_to_vec().unwrap(),
            ]
            .concat(),
        }
    } else {
        let (seat, _) = get_seat_address(market, trader);
        Instruction {
            program_id: crate::id(),
            accounts: vec![
                AccountMeta::new_readonly(crate::id(), false),
                AccountMeta::new_readonly(phoenix_log_authority::id(), false),
                AccountMeta::new(*market, false),
                AccountMeta::new(*trader, true),
                AccountMeta::new_readonly(seat, false),
                AccountMeta::new(*base_account, false),
                AccountMeta::new(*quote_account, false),
                AccountMeta::new(base_vault, false),
                AccountMeta::new(quote_vault, false),
                AccountMeta::new_readonly(spl_token::id(), false),
            ],
            data: [
                PhoenixInstruction::PlaceLimitOrder.to_vec(),
                order_type.try_to_vec().unwrap(),
            ]
            .concat(),
        }
    }
}

pub fn create_new_order_with_free_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    order_type: &OrderPacket,
) -> Instruction {
    let (seat, _) = get_seat_address(market, trader);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(seat, false),
        ],
        data: [
            if order_type.is_take_only() {
                PhoenixInstruction::SwapWithFreeFunds.to_vec()
            } else {
                PhoenixInstruction::PlaceLimitOrderWithFreeFunds.to_vec()
            },
            order_type.try_to_vec().unwrap(),
        ]
        .concat(),
    }
}

pub fn create_new_multiple_order_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    multiple_order_packet: &MultipleOrderPacket,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_new_multiple_order_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        multiple_order_packet,
    )
}

pub fn create_new_multiple_order_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    multiple_order_packet: &MultipleOrderPacket,
) -> Instruction {
    let (base_vault, _) = get_vault_address(market, base);
    let (quote_vault, _) = get_vault_address(market, quote);
    let (seat, _) = get_seat_address(market, trader);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(seat, false),
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [
            PhoenixInstruction::PlaceMultiplePostOnlyOrders.to_vec(),
            multiple_order_packet.try_to_vec().unwrap(),
        ]
        .concat(),
    }
}

pub fn create_new_multiple_order_with_free_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    multiple_order_packet: &MultipleOrderPacket,
) -> Instruction {
    let (seat, _) = get_seat_address(market, trader);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(seat, false),
        ],
        data: [
            PhoenixInstruction::PlaceMultiplePostOnlyOrdersWithFreeFunds.to_vec(),
            multiple_order_packet.try_to_vec().unwrap(),
        ]
        .concat(),
    }
}

pub fn create_cancel_all_order_with_free_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
        ],
        data: PhoenixInstruction::CancelAllOrdersWithFreeFunds.to_vec(),
    }
}

pub fn create_cancel_up_to_with_free_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    params: &CancelUpToParams,
) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
        ],
        data: [
            PhoenixInstruction::CancelUpToWithFreeFunds.to_vec(),
            params.try_to_vec().unwrap(),
        ]
        .concat(),
    }
}

pub fn create_cancel_multiple_orders_by_id_with_free_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    params: &CancelMultipleOrdersByIdParams,
) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
        ],
        data: [
            PhoenixInstruction::CancelMultipleOrdersByIdWithFreeFunds.to_vec(),
            params.try_to_vec().unwrap(),
        ]
        .concat(),
    }
}

pub fn create_reduce_order_with_free_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    params: &ReduceOrderParams,
) -> Instruction {
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
        ],
        data: [
            PhoenixInstruction::ReduceOrderWithFreeFunds.to_vec(),
            params.try_to_vec().unwrap(),
        ]
        .concat(),
    }
}

pub fn create_deposit_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &DepositParams,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    let (seat, _) = get_seat_address(market, trader);
    create_deposit_funds_instruction_with_custom_token_accounts(
        market,
        trader,
        &seat,
        &base_account,
        &quote_account,
        base,
        quote,
        params,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn create_deposit_funds_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    seat: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &DepositParams,
) -> Instruction {
    let (base_vault, _) = get_vault_address(market, base);
    let (quote_vault, _) = get_vault_address(market, quote);
    let ix_data = params.try_to_vec().unwrap();
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new(*seat, false),
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [PhoenixInstruction::DepositFunds.to_vec(), ix_data].concat(),
    }
}

#[allow(clippy::too_many_arguments)]
fn _phoenix_instruction_template<T: BorshSerialize>(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    ix_id: PhoenixInstruction,
    params: Option<&T>,
) -> Instruction {
    let (base_vault, _) = get_vault_address(market, base);
    let (quote_vault, _) = get_vault_address(market, quote);
    let ix_data = match params {
        Some(i) => i.try_to_vec().unwrap(),
        None => vec![],
    };
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [[ix_id as u8].to_vec(), ix_data].concat(),
    }
}

fn _phoenix_instruction_template_no_param(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    ix_id: PhoenixInstruction,
) -> Instruction {
    let (base_vault, _) = get_vault_address(market, base);
    let (quote_vault, _) = get_vault_address(market, quote);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [ix_id as u8].to_vec(),
    }
}

pub fn reduce_order_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &ReduceOrderParams,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_reduce_order_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        params,
    )
}

pub fn create_reduce_order_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &ReduceOrderParams,
) -> Instruction {
    _phoenix_instruction_template::<ReduceOrderParams>(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::ReduceOrder,
        Some(params),
    )
}

pub fn create_cancel_all_orders_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_cancel_all_orders_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
    )
}

pub fn create_cancel_all_orders_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
) -> Instruction {
    _phoenix_instruction_template_no_param(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::CancelAllOrders,
    )
}

pub fn create_cancel_up_to_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &CancelUpToParams,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_cancel_up_to_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        params,
    )
}

pub fn create_cancel_up_to_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &CancelUpToParams,
) -> Instruction {
    _phoenix_instruction_template::<CancelUpToParams>(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::CancelUpTo,
        Some(params),
    )
}

pub fn create_cancel_multiple_orders_by_id_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &CancelMultipleOrdersByIdParams,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_cancel_multiple_orders_by_id_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        params,
    )
}

pub fn create_cancel_multiple_orders_by_id_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &CancelMultipleOrdersByIdParams,
) -> Instruction {
    _phoenix_instruction_template::<CancelMultipleOrdersByIdParams>(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::CancelMultipleOrdersById,
        Some(params),
    )
}

pub fn create_withdraw_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_withdraw_funds_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
    )
}

pub fn create_withdraw_funds_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
) -> Instruction {
    _phoenix_instruction_template::<WithdrawParams>(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::WithdrawFunds,
        Some(&WithdrawParams {
            quote_lots_to_withdraw: None,
            base_lots_to_withdraw: None,
        }),
    )
}

pub fn create_withdraw_funds_with_custom_amounts_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    base_lots: u64,
    quote_lots: u64,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_withdraw_funds_with_custom_amounts_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        &WithdrawParams {
            quote_lots_to_withdraw: Some(quote_lots),
            base_lots_to_withdraw: Some(base_lots),
        },
    )
}

pub fn create_withdraw_funds_with_custom_amounts_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &WithdrawParams,
) -> Instruction {
    _phoenix_instruction_template::<WithdrawParams>(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::WithdrawFunds,
        Some(params),
    )
}

pub fn create_request_seat_instruction(payer: &Pubkey, market: &Pubkey) -> Instruction {
    let (seat, _) = get_seat_address(market, payer);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new_readonly(crate::id(), false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market, false),
            AccountMeta::new(*payer, true),
            AccountMeta::new(seat, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: PhoenixInstruction::RequestSeat.to_vec(),
    }
}
