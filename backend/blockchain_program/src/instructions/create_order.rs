use blockchain_core::{
    accounts::
        order::{Order, TokenOption}
    ,
    error::MarketError,
    instructions::CreateOrderArgs,
};
use pinocchio::{account_info::AccountInfo, instruction::Signer, pubkey::pubkey_eq, seeds, sysvars::{rent::Rent, Sysvar}, ProgramResult};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::MintToChecked;

use crate::utils::{check_associated_token_program, check_token_program, check_usdc, deserialize_and_check_event, must_be_uninit};

pub fn create_order(accounts: &[AccountInfo], args: &CreateOrderArgs) -> ProgramResult {
    let [user, event, order, token_account, usdc, user_usdc_ata, user_token_ata, order_ata, _system_program, token_program, associated_token_program] = accounts else {
        return Err(MarketError::InvalidAccounts.into());
    };

    // check token program and associated token program
    check_token_program(token_program)?;
    check_associated_token_program(associated_token_program)?;

    // deser and check event
    let event_data = deserialize_and_check_event(event, &args.event_uuid)?;

    // check that option exists and check that token is correct
    let option = event_data.options.get(&args.option_uuid).ok_or(MarketError::OptionMissmatch)?;
    let token_mint = &match args.token {
        TokenOption::Yes => {
            option.yes_mint
        },
        TokenOption::No => {
            option.no_mint
        }
    };
    if !pubkey_eq(token_mint, token_account.key()) {
        return Err(MarketError::TokenMissmatch)?;
    }

    // check order account derivation
    let (order_pda, _) = Order::find_program_address(
        &args.event_uuid,
        &args.option_uuid,
        &args.seed,
        user.key(),
        token_account.key(),
        &crate::ID,
    );

    if !pubkey_eq(&order_pda, order.key()) {
        return Err(MarketError::OrderPDA)?;
    }

    let user_key_bytes: &[u8] = user.key().as_slice();
    let token_key_bytes: &[u8] = token_account.key().as_slice();
    let seed_bytes: &[u8] = &args.seed.into_bytes();
    let order_seeds = seeds!(
        b"order",
        user_key_bytes,
        token_key_bytes,
        user_key_bytes,
        token_key_bytes,
        seed_bytes
    );

    // create the order account
    let order_data = Order {
        event_uuid: args.event_uuid,
        option_uuid: args.option_uuid,
        user: *user.key(),
        token: args.token,
        seed: args.seed,
    };

    let rent = Rent::get()?;
    let order_len = order_data.len()?;
    let order_rent = rent.minimum_balance(order_len as usize);

    CreateAccount {
        from: user,
        to: order,
        lamports: order_rent,
        space: order_len,
        owner: &crate::ID
    }.invoke_signed(&[Signer::from(&order_seeds)])?;

    {
        let mut order_bytes = order.try_borrow_mut_data()?;
        order_data.write_into_bytes(&mut order_bytes)?;
    }

    // check USDC
    check_usdc(usdc)?;

    // check vault
    must_be_uninit(order_ata)?;

    // TODO: CHECK OVERFLOW
    let total_usdc = args.num_shares * args.token_per_share;

    // deposit USDC into vault, from user's ATA
    // TODO: does not need to be checked
    pinocchio_token::instructions::TransferChecked {
        from: user_usdc_ata,
        mint: usdc,
        to: order_ata,
        authority: order,
        amount: total_usdc,
        decimals: 6
    }.invoke()?;

    let event_uuid_ref = args.event_uuid.as_bytes();
    let event_bump_ref = &[event_data.bump];
    let event_seeds = seeds!(b"event", event_uuid_ref, event_bump_ref);

    // mint tokens to user
    MintToChecked {
        mint: token_account,
        account: user_token_ata,
        mint_authority: event,
        amount: args.num_shares,
        decimals: 6
    }
    .invoke_signed(&[Signer::from(&event_seeds)])?;

    Ok(())
}
