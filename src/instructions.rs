use crate::enums::Side;
use crate::order_packet::OrderPacket;
use borsh::{BorshDeserialize, BorshSerialize};
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

#[derive(Debug, Copy, Clone, BorshSerialize, BorshDeserialize)]
#[rustfmt::skip]
pub enum PhoenixInstruction {
    Instruction0,
    /// Send a swap (no limit orders allowed) order
    Swap,
    /// Place a limit order on the book. The order can cross if the supplied order type is Limit
    PlaceLimitOrder,
    /// Reduce the size of an existing order on the book 
    ReduceOrder,
    /// Remove all orders from the book
    CancelAllOrders,
    /// Remove multiple orders from the book based off a price
    CancelMultipleOrders,
    Instruction6,
    /// Remove multiple orders from the book based off id
    CancelMultipleOrdersById,
    /// Withdraw funds from the vault 
    WithdrawFunds,
    /// Deposit funds into the vault 
    DepositFunds,
    Instruction10,
    Instruction11,
    Instruction12,
    Instruction13,
    Instruction14,
    Instruction15,
    /// Request a seat on the market. The exchange authority will need to approve your seat
    RequestSeat,
    Instruction16,
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
pub struct CancelMultipleOrdersParams {
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
    if let OrderPacket::ImmediateOrCancel { .. } = order_type {
        Instruction {
            program_id: crate::id(),
            accounts: vec![
                AccountMeta::new(*market, false),
                AccountMeta::new(*trader, true),
                AccountMeta::new(*base_account, false),
                AccountMeta::new(*quote_account, false),
                AccountMeta::new(base_vault, false),
                AccountMeta::new(quote_vault, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(spl_noop::id(), false),
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
                AccountMeta::new(*base_account, false),
                AccountMeta::new(*quote_account, false),
                AccountMeta::new(base_vault, false),
                AccountMeta::new(quote_vault, false),
                AccountMeta::new_readonly(spl_token::id(), false),
                AccountMeta::new_readonly(spl_noop::id(), false),
                AccountMeta::new_readonly(seat, false),
            ],
            data: [
                PhoenixInstruction::PlaceLimitOrder.try_to_vec().unwrap(),
                order_type.try_to_vec().unwrap(),
            ]
            .concat(),
        }
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
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_noop::id(), false),
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
            AccountMeta::new(*base_account, false),
            AccountMeta::new(*quote_account, false),
            AccountMeta::new(base_vault, false),
            AccountMeta::new(quote_vault, false),
            AccountMeta::new_readonly(spl_token::id(), false),
            AccountMeta::new_readonly(spl_noop::id(), false),
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

pub fn create_cancel_multiple_orders_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &CancelMultipleOrdersParams,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_cancel_multiple_orders_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        params,
    )
}

pub fn create_cancel_multiple_orders_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &CancelMultipleOrdersParams,
) -> Instruction {
    _phoenix_instruction_template::<CancelMultipleOrdersParams>(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::CancelMultipleOrders,
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

pub fn create_deposit_funds_instruction(
    market: &Pubkey,
    trader: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &DepositParams,
) -> Instruction {
    let base_account = get_associated_token_address(trader, base);
    let quote_account = get_associated_token_address(trader, quote);
    create_deposit_funds_instruction_with_custom_token_accounts(
        market,
        trader,
        &base_account,
        &quote_account,
        base,
        quote,
        params,
    )
}

pub fn create_deposit_funds_instruction_with_custom_token_accounts(
    market: &Pubkey,
    trader: &Pubkey,
    base_account: &Pubkey,
    quote_account: &Pubkey,
    base: &Pubkey,
    quote: &Pubkey,
    params: &DepositParams,
) -> Instruction {
    _phoenix_instruction_template(
        market,
        trader,
        base_account,
        quote_account,
        base,
        quote,
        PhoenixInstruction::DepositFunds,
        Some(params),
    )
}
