use blockchain_core::{error::MarketError, instructions::FakeMatchOrderArgs};
use pinocchio::{
    account_info::AccountInfo, instruction::Signer, pubkey::pubkey_eq, seeds, ProgramResult,
};
use pinocchio_token::instructions::{MintToChecked, TransferChecked};

use crate::utils::{
    check_associated_token_program, check_token_program, check_usdc, deserialize_and_check_event,
};

pub fn fake_match_order(accounts: &[AccountInfo], args: &FakeMatchOrderArgs) -> ProgramResult {
    let [user_yes, user_yes_usdc_ata, user_yes_token_ata, user_no, user_no_usdc_ata, user_no_token_ata, event, treasury, token_yes, token_no, usdc, _system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(MarketError::InvalidAccounts.into());
    };

    // check token program and associated token program
    check_token_program(token_program)?;
    check_associated_token_program(associated_token_program)?;

    // check usdc
    check_usdc(usdc)?;

    // deser and check event
    let event_data = deserialize_and_check_event(event, &args.event_uuid)?;

    // check that option exists and check that token mints are correct
    let option = event_data
        .options
        .get(&args.option_uuid)
        .ok_or(MarketError::OptionMissmatch)?;

    if !pubkey_eq(token_yes.key(), &option.yes_mint) {
        return Err(MarketError::TokenMissmatch)?;
    }
    if !pubkey_eq(token_no.key(), &option.no_mint) {
        return Err(MarketError::TokenMissmatch)?;
    }

    let event_uuid_ref = args.event_uuid.as_bytes();
    let event_bump_ref = &[event_data.bump];
    let event_seeds = seeds!(b"event", event_uuid_ref, event_bump_ref);

    // mint to user A and user B
    MintToChecked {
        mint: token_yes,
        account: user_yes_token_ata,
        mint_authority: event,
        amount: args.num_shares,
        decimals: 6,
    }
    .invoke_signed(&[Signer::from(&event_seeds)])?;

    MintToChecked {
        mint: token_no,
        account: user_no_token_ata,
        mint_authority: event,
        amount: args.num_shares,
        decimals: 6,
    }
    .invoke_signed(&[Signer::from(&event_seeds)])?;

    // check treasury
    // TODO:

    // transfer USDC from A and B to treasury
    TransferChecked {
        from: user_yes_usdc_ata,
        mint: usdc,
        to: treasury,
        authority: user_yes,
        amount: args.num_shares * args.yes_price, // TODO: overflow,
        decimals: 6,
    }
    .invoke()?;

    TransferChecked {
        from: user_no_usdc_ata,
        mint: usdc,
        to: treasury,
        authority: user_no,
        amount: args.num_shares * args.no_price, // TODO: overflow,
        decimals: 6,
    }
    .invoke()?;

    Ok(())
}
