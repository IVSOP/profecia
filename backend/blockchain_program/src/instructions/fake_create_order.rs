use blockchain_core::{error::MarketError, instructions::FakeCreateOrderArgs};
use pinocchio::{
    account_info::AccountInfo, ProgramResult,
};
use pinocchio_token::instructions::TransferChecked;

use crate::utils::{
    check_associated_token_program, check_existing_ata, check_token_program, check_usdc, deserialize_and_check_event
};

pub fn fake_create_order(accounts: &[AccountInfo], args: &FakeCreateOrderArgs) -> ProgramResult {
    let [user, user_usdc_ata, event, treasury, usdc, _system_program, token_program, associated_token_program] =
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
    let _event_data = deserialize_and_check_event(event, &args.event_uuid)?;

    // check treasury
    check_existing_ata(treasury, usdc.key(), event.key())?;

    // send usdc TODO: checks missing???
    TransferChecked {
        from: user_usdc_ata,
        mint: usdc,
        to: treasury,
        authority: user,
        amount: args.num_shares * args.price_per_share, // TODO: overflow
        decimals: 6,
    }
    .invoke()?;



    // // check that option exists and check that token mints are correct
    // let option = event_data
    //     .options
    //     .get(&args.option_uuid)
    //     .ok_or(MarketError::OptionMissmatch)?;

    // if !pubkey_eq(token_yes.key(), &option.yes_mint) {
    //     return Err(MarketError::TokenMissmatch)?;
    // }
    // if !pubkey_eq(token_no.key(), &option.no_mint) {
    //     return Err(MarketError::TokenMissmatch)?;
    // }

    // let event_uuid_ref = args.event_uuid.as_bytes();
    // let event_bump_ref = &[event_data.bump];
    // let event_seeds = seeds!(b"event", event_uuid_ref, event_bump_ref);

    // pinocchio_log::log!("check or init user YES token ata");

    // // check or init user A and user B
    // let user_yes_ata_args = CreateOrCheckAtaArgs {
    //     ata: user_yes_token_ata,
    //     owner: user_yes,
    //     mint: token_yes,
    //     funding_account: user_yes,
    //     system_program,
    //     token_program,
    //     associated_token_program
    // };
    // create_or_check_ata(&user_yes_ata_args, &[])?;

    // pinocchio_log::log!("check or init user NO token ata");

    // let user_no_ata_args = CreateOrCheckAtaArgs {
    //     ata: user_no_token_ata,
    //     owner: user_no,
    //     mint: token_no,
    //     funding_account: user_no,
    //     system_program,
    //     token_program,
    //     associated_token_program
    // };
    // create_or_check_ata(&user_no_ata_args, &[])?;

    // pinocchio_log::log!("minting");

    // // mint to user A and user B
    // MintToChecked {
    //     mint: token_yes,
    //     account: user_yes_token_ata,
    //     mint_authority: event,
    //     amount: args.num_shares * 1000000, // 1 share will result in 1000000 tokens, as they have 6 decimals
    //     decimals: 6,
    // }
    // .invoke_signed(&[Signer::from(&event_seeds)])?;

    // MintToChecked {
    //     mint: token_no,
    //     account: user_no_token_ata,
    //     mint_authority: event,
    //     amount: args.num_shares * 1000000, // 1 share will result in 1000000 tokens, as they have 6 decimals
    //     decimals: 6,
    // }
    // .invoke_signed(&[Signer::from(&event_seeds)])?;

    // // check treasury
    // check_existing_ata(treasury, usdc.key(), event.key())?;

    // // transfer USDC from A and B to treasury
    // TransferChecked {
    //     from: user_yes_usdc_ata,
    //     mint: usdc,
    //     to: treasury,
    //     authority: user_yes,
    //     amount: args.num_shares * args.yes_price, // TODO: overflow,
    //     decimals: 6,
    // }
    // .invoke()?;

    // TransferChecked {
    //     from: user_no_usdc_ata,
    //     mint: usdc,
    //     to: treasury,
    //     authority: user_no,
    //     amount: args.num_shares * args.no_price, // TODO: overflow,
    //     decimals: 6,
    // }
    // .invoke()?;

    Ok(())
}
