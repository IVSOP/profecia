use blockchain_core::{
    error::MarketError,
    instructions::TransferSharesArgs,
};
use pinocchio::{
    account_info::AccountInfo,
    ProgramResult,
};
use pinocchio_token::instructions::TransferChecked;

use crate::utils::{
    CreateOrCheckAtaArgs, check_associated_token_program, check_token_program, check_usdc, create_or_check_ata, deserialize_and_check_event
};

// transfer token from user A to B
// transfer usdc from user B to A
pub fn transfer_shares(accounts: &[AccountInfo], args: &TransferSharesArgs) -> ProgramResult {
    let [user_a, user_a_token_ata, user_a_usdc_ata, user_b, user_b_token_ata, user_b_usdc_ata, event, token, usdc, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(MarketError::InvalidAccounts.into());
    };

    // check token program and associated token program
    // pinocchio_log::log!("check token program and associated token program");
    check_token_program(token_program)?;
    check_associated_token_program(associated_token_program)?;

    // TODO: check if token belongs to that option etc

    check_usdc(usdc)?;

    let _event_data = deserialize_and_check_event(event, &args.event_uuid)?;

    // create user a usdc ata, if needed
    let create_or_check_ata_args = CreateOrCheckAtaArgs {
        ata: user_a_token_ata,
        owner: user_a,
        mint: token,
        funding_account: user_a,
        system_program,
        token_program,
        associated_token_program
    };
    create_or_check_ata(&create_or_check_ata_args, &[])?;

    let create_or_check_ata_args = CreateOrCheckAtaArgs {
        ata: user_a_usdc_ata,
        owner: user_a,
        mint: usdc,
        funding_account: user_a,
        system_program,
        token_program,
        associated_token_program
    };
    create_or_check_ata(&create_or_check_ata_args, &[])?;

    let create_or_check_ata_args = CreateOrCheckAtaArgs {
        ata: user_b_token_ata,
        owner: user_b,
        mint: token,
        funding_account: user_a,
        system_program,
        token_program,
        associated_token_program
    };
    create_or_check_ata(&create_or_check_ata_args, &[])?;

    let create_or_check_ata_args = CreateOrCheckAtaArgs {
        ata: user_b_usdc_ata,
        owner: user_b,
        mint: usdc,
        funding_account: user_a,
        system_program,
        token_program,
        associated_token_program
    };
    create_or_check_ata(&create_or_check_ata_args, &[])?;

    // transfer shares from user a to b
    TransferChecked {
        from: user_a_token_ata,
        to: user_b_token_ata,
        mint: token,
        authority: user_a,
        amount: args.num_shares * 1000000,
        decimals: 6
    }.invoke()?;

    // transfer usdc from user b to a    
    TransferChecked {
        from: user_b_usdc_ata,
        to: user_a_usdc_ata,
        mint: usdc,
        authority: user_b,
        amount: args.num_shares * args.price_per_share,
        decimals: 6
    }.invoke()?;

    Ok(())
}
