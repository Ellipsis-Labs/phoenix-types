use crate::enums::Side;
use crate::order_packet::OrderPacket;
use borsh::{BorshDeserialize, BorshSerialize};
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

#[derive(Debug, Copy, Clone, ShankInstruction, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum PhoenixInstruction {
    // Market instructions
    /// Send a swap (no limit orders allowed) order
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, writable, name = "base_account", desc = "Trader base token account")]
    #[account(4, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(5, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(6, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(7, name = "token_program", desc = "Token program")]
    Swap = 0,

    /// Send a swap (no limit orders allowed) order using only deposited funds
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, name = "seat")]
    SwapWithFreeFunds = 1,

    /// Place a limit order on the book. The order can cross if the supplied order type is Limit
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, name = "seat")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    PlaceLimitOrder = 2,

    /// Place a limit order on the book using only deposited funds.
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, name = "seat")]
    PlaceLimitOrderWithFreeFunds = 3,

    /// Reduce the size of an existing order on the book 
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, writable, name = "base_account", desc = "Trader base token account")]
    #[account(4, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(5, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(6, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(7, name = "token_program", desc = "Token program")]
    ReduceOrder = 4,

    /// Reduce the size of an existing order on the book 
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    ReduceOrderWithFreeFunds = 5,


    /// Cancel all orders 
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, writable, name = "base_account", desc = "Trader base token account")]
    #[account(4, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(5, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(6, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(7, name = "token_program", desc = "Token program")]
    CancelAllOrders = 6,

    /// Cancel all orders (no token transfers) 
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    CancelAllOrdersWithFreeFunds = 7,

    /// Cancel all orders more aggressive than a specified price
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, writable, name = "base_account", desc = "Trader base token account")]
    #[account(4, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(5, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(6, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(7, name = "token_program", desc = "Token program")]
    CancelUpTo = 8,


    /// Cancel all orders more aggressive than a specified price (no token transfers) 
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    CancelUpToWithFreeFunds = 9,

    /// Cancel multiple orders by ID 
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, writable, name = "base_account", desc = "Trader base token account")]
    #[account(4, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(5, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(6, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(7, name = "token_program", desc = "Token program")]
    CancelMultipleOrdersById = 10,

    /// Cancel multiple orders by ID (no token transfers) 
    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    CancelMultipleOrdersByIdWithFreeFunds = 11,

    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, writable, name = "base_account", desc = "Trader base token account")]
    #[account(4, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(5, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(6, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(7, name = "token_program", desc = "Token program")]
    WithdrawFunds = 12,

    #[account(0, writable, name = "market", desc = "This account holds the market state")]
    #[account(1, writable, signer, name = "trader")]
    #[account(2, name = "wrapper_program", desc = "No-op wrapper program")]
    #[account(3, name = "seat")]
    #[account(4, writable, name = "base_account", desc = "Trader base token account")]
    #[account(5, writable, name = "quote_account", desc = "Trader quote token account")]
    #[account(6, writable, name = "base_vault", desc = "Base vault PDA, seeds are [b'vault', market_address, base_mint_address]")]
    #[account(7, writable, name = "quote_vault", desc = "Quote vault PDA, seeds are [b'vault', market_address, quote_mint_address]")]
    #[account(8, name = "token_program", desc = "Token program")]
    DepositFunds = 13,

    #[account(0, writable, signer, name = "payer")]
    #[account(1, name = "market")]
    #[account(2, writable, name = "seat")]
    #[account(3, name = "system_program", desc = "System program")]
    RequestSeat = 14,
}

#[derive(BorshDeserialize, BorshSerialize, Clone, Copy)]
pub struct CancelOrderParams {
    pub side: Side,
    pub num_quote_ticks_per_base_unit: u64,
    pub order_id: u64,
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
                AccountMeta::new(*market, false),
                AccountMeta::new(*trader, true),
                AccountMeta::new_readonly(spl_noop::id(), false),
                AccountMeta::new(*base_account, false),
                AccountMeta::new(*quote_account, false),
                AccountMeta::new(base_vault, false),
                AccountMeta::new(quote_vault, false),
                AccountMeta::new_readonly(spl_token::id(), false),
            ],
            data: [
                PhoenixInstruction::Swap.try_to_vec().unwrap(),
                order_type.try_to_vec().unwrap(),
            ]
            .concat(),
        }
    } else {
        let (seat, _) = get_seat_address(market, trader);
        Instruction {
            program_id: crate::id(),
            accounts: vec![
                AccountMeta::new(*market, false),
                AccountMeta::new(*trader, true),
                AccountMeta::new_readonly(spl_noop::id(), false),
                AccountMeta::new_readonly(seat, false),
                AccountMeta::new(*base_account, false),
                AccountMeta::new(*quote_account, false),
                AccountMeta::new(base_vault, false),
                AccountMeta::new(quote_vault, false),
                AccountMeta::new_readonly(spl_token::id(), false),
            ],
            data: [
                PhoenixInstruction::PlaceLimitOrder.try_to_vec().unwrap(),
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
    create_new_order_instruction_with_free_funds_with_custom_token_accounts(
        market, trader, order_type,
    )
}

pub fn create_new_order_instruction_with_free_funds_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    order_type: &OrderPacket,
) -> Instruction {
    let (seat, _) = get_seat_address(market, trader);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
            AccountMeta::new_readonly(seat, false),
        ],
        data: [
            if order_type.is_take_only() {
                PhoenixInstruction::SwapWithFreeFunds.try_to_vec().unwrap()
            } else {
                PhoenixInstruction::PlaceLimitOrderWithFreeFunds
                    .try_to_vec()
                    .unwrap()
            },
            order_type.try_to_vec().unwrap(),
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
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
        ],
        data: PhoenixInstruction::CancelAllOrdersWithFreeFunds
            .try_to_vec()
            .unwrap(),
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
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
        ],
        data: [
            PhoenixInstruction::CancelUpToWithFreeFunds
                .try_to_vec()
                .unwrap(),
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
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
        ],
        data: [
            PhoenixInstruction::CancelMultipleOrdersByIdWithFreeFunds
                .try_to_vec()
                .unwrap(),
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
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
        ],
        data: [
            PhoenixInstruction::ReduceOrderWithFreeFunds
                .try_to_vec()
                .unwrap(),
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
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
            AccountMeta::new(*seat, false),
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [
            PhoenixInstruction::DepositFunds.try_to_vec().unwrap(),
            ix_data,
        ]
        .concat(),
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
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: [ix_id.try_to_vec().unwrap(), ix_data].concat(),
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
            AccountMeta::new(*market, false),
            AccountMeta::new(*trader, true),
            AccountMeta::new_readonly(spl_noop::id(), false),
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
        ],
        data: ix_id.try_to_vec().unwrap(),
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
    _phoenix_instruction_template_no_param(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::WithdrawFunds,
    )
}

pub fn create_request_seat_instruction(payer: &Pubkey, market: &Pubkey) -> Instruction {
    let (seat, _) = get_seat_address(market, payer);
    Instruction {
        program_id: crate::id(),
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(*market, false),
            AccountMeta::new(seat, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
        data: PhoenixInstruction::RequestSeat.try_to_vec().unwrap(),
    }
}
