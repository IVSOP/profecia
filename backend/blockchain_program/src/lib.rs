#![allow(unexpected_cfgs)]

use blockchain_core::{error::MarketError, instructions::MarketInstruction};
use pinocchio::{
    account_info::AccountInfo, default_allocator, default_panic_handler, program_entrypoint,
    pubkey::Pubkey, ProgramResult,
};
use pinocchio_pubkey::pubkey;

use crate::instructions::{
    add_option::add_option, close_event::close_event, create_empty_event::create_empty_event, create_event::create_event, fake_cancel_order::fake_cancel_order, fake_create_order::fake_create_order, fake_get_reward::fake_get_reward, fake_match_order::fake_match_order
};

pinocchio_pubkey::declare_id!("ProirMXDTFF4AEqGyZVKPhWte4chANDd1c4Y8w7Nsd4");

mod instructions;
mod utils;

program_entrypoint!(process_instruction);

// no_allocator!();
default_allocator!();

// nostd_panic_handler!();
default_panic_handler!();

pub const USDC_ADDRESS: Pubkey = pubkey!("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: MarketInstruction =
        wincode::deserialize(instruction_data).map_err(|_| MarketError::InstructionDeser)?;

    match instruction {
        MarketInstruction::CreateEvent(ref args) => {
            create_event(accounts, args)?;
        }
        MarketInstruction::CreateEmptyEvent(ref args) => {
            create_empty_event(accounts, args)?;
        }
        MarketInstruction::CloseEvent(ref args) => {
            close_event(accounts, args)?;
        }
        MarketInstruction::FakeMatchOrder(ref args) => {
            fake_match_order(accounts, args)?;
        }
        MarketInstruction::FakeCreateOrder(ref args) => {
            fake_create_order(accounts, args)?;
        }
        MarketInstruction::FakeCancelOrder(ref args) => {
            fake_cancel_order(accounts, args)?;
        }
        MarketInstruction::FakeGetReward(ref args) => {
            fake_get_reward(accounts, args)?;
        }
        MarketInstruction::AddOption(ref args) => {
            add_option(accounts, args)?;
        }
        _ => {}
    }

    Ok(())
}
