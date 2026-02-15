use blockchain_core::{
    error::MarketError,
    instructions::AddOptionArgs,
};
use pinocchio::{
    account_info::AccountInfo,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;
use pinocchio_token::instructions::InitializeMint2;

use crate::utils::{
    check_associated_token_program, check_token_program, deserialize_and_check_event, must_be_uninit
};

pub fn add_option(accounts: &[AccountInfo], args: &AddOptionArgs) -> ProgramResult {
    let [payer, event, _system_program, token_program, associated_token_program, yes_token, no_token] =
        accounts
    else {
        return Err(MarketError::InvalidAccounts.into());
    };

    // check token program and associated token program
    // pinocchio_log::log!("check token program and associated token program");
    check_token_program(token_program)?;
    check_associated_token_program(associated_token_program)?;

    let rent = Rent::get()?;

    // TODO: check that payer is the admin (admin is not defined yet. for now anyone can create marketplace)

    let mut event_data = deserialize_and_check_event(event, &args.event_uuid)?;
    event_data.options.insert(args.option_uuid, args.option_info.clone());

    // pinocchio_log::log!("process token mints");
    // processs remaining accounts. they must be the yes and no mints specified in the accounts, and must be uninit
    // then create the mint
    // if rest.len() != args.options.len() * 2 {
    //     return Err(MarketError::InvalidAccounts.into());
    // }

    // Mint account size is 82 bytes
    const MINT_SIZE: u64 = 82;

    // for mints in rest.chunks_exact(2).into_iter() {
        let yes_mint = yes_token;
        let no_mint = no_token;

        must_be_uninit(&yes_mint)?;
        must_be_uninit(&no_mint)?;

        // Create yes_mint account
        let yes_mint_rent = rent.minimum_balance(MINT_SIZE as usize);
        CreateAccount {
            from: payer,
            to: yes_mint,
            lamports: yes_mint_rent,
            space: MINT_SIZE,
            owner: token_program.key(),
        }
        .invoke()?;

        // Initialize yes_mint
        InitializeMint2 {
            mint: yes_mint,
            decimals: 6,
            mint_authority: event.key(),
            freeze_authority: None,
        }
        .invoke()?;

        // Create no_mint account
        let no_mint_rent = rent.minimum_balance(MINT_SIZE as usize);
        CreateAccount {
            from: payer,
            to: no_mint,
            lamports: no_mint_rent,
            space: MINT_SIZE,
            owner: token_program.key(),
        }
        .invoke()?;

        // Initialize no_mint
        InitializeMint2 {
            mint: no_mint,
            decimals: 6,
            mint_authority: event.key(),
            freeze_authority: None,
        }
        .invoke()?;
    // }

    Ok(())
}
