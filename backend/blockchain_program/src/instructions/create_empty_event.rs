use std::collections::HashMap;

use blockchain_core::{
    accounts::event::{Event, EventState},
    error::MarketError,
    instructions::CreateEmptyEventArgs,
};
use fast_ata_pinocchio::instructions::CreateAta;
use pinocchio::{
    account_info::AccountInfo,
    instruction::Signer,
    pubkey::pubkey_eq,
    seeds,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

use crate::utils::{
    check_associated_token_program, check_token_program, check_usdc, must_be_uninit,
};

pub fn create_empty_event(accounts: &[AccountInfo], args: &CreateEmptyEventArgs) -> ProgramResult {
    let [payer, event, usdc_mint, treasury, system_program, token_program, associated_token_program] =
        accounts
    else {
        return Err(MarketError::InvalidAccounts.into());
    };

    let rent = Rent::get()?;

    // check token program and associated token program
    // pinocchio_log::log!("check token program and associated token program");
    check_token_program(token_program)?;
    check_associated_token_program(associated_token_program)?;

    // TODO: check that payer is the admin (admin is not defined yet. for now anyone can create marketplace)

    // check that marketplace was properly derived and uninit
    // pinocchio_log::log!("check that marketplace was properly derived and uninit");
    let (event_pda, event_bump) = Event::find_program_address(&args.uuid, &crate::ID);
    if !pubkey_eq(event.key(), &event_pda) {
        return Err(MarketError::MarketPDA)?;
    }
    must_be_uninit(event)?;

    // create the marketplace
    // pinocchio_log::log!("create the marketplace");
    let event_data = Event {
        uuid: args.uuid,
        description: args.description.clone(),
        state: EventState::NotFinished,
        options: HashMap::new(),
        bump: event_bump,
    };
    let event_uuid_ref = args.uuid.as_bytes();
    let event_bump_ref = &[event_bump];
    let event_seeds = seeds!(b"event", event_uuid_ref, event_bump_ref);
    let event_len = event_data.len()?;
    let event_rent = rent.minimum_balance(event_len as usize);

    CreateAccount {
        from: payer,
        to: event,
        lamports: event_rent,
        space: event_len,
        owner: &crate::ID,
    }
    .invoke_signed(&[Signer::from(&event_seeds)])?;

    {
        let mut event_bytes = event.try_borrow_mut_data()?;
        event_data.write_into_bytes(&mut event_bytes)?;
    }

    // check USDC mint
    // pinocchio_log::log!("check USDC mint");
    check_usdc(usdc_mint)?;

    // checking that treasury was correctly derived (USDC ATA of the event) is not needed.
    // just check that it is uninit for safety. when actually creating the account, the ATProgram will check the PDA derivation
    // pinocchio_log::log!("check treasury");
    must_be_uninit(treasury)?;

    // create the treasury
    // pinocchio_log::log!("create the treasury");
    CreateAta {
        funding_account: payer,
        ata: treasury,
        owner: event,
        mint: usdc_mint,
        system_program,
        token_program,
        associated_token_program,
    }
    .invoke_signed(&[Signer::from(&event_seeds)])?; // FIX: DO I JUST NEED TO PROVIDE EVENT SEEDS???

    // // pinocchio_log::log!("process token mints");
    // // processs remaining accounts. they must be the yes and no mints specified in the accounts, and must be uninit
    // // then create the mint
    // if rest.len() != args.options.len() * 2 {
    //     return Err(MarketError::InvalidAccounts.into());
    // }

    // // Mint account size is 82 bytes
    // const MINT_SIZE: u64 = 82;

    // for mints in rest.chunks_exact(2).into_iter() {
    //     let yes_mint = &mints[0];
    //     let no_mint = &mints[1];

    //     must_be_uninit(&yes_mint)?;
    //     must_be_uninit(&no_mint)?;

    //     // Create yes_mint account
    //     let yes_mint_rent = rent.minimum_balance(MINT_SIZE as usize);
    //     CreateAccount {
    //         from: payer,
    //         to: yes_mint,
    //         lamports: yes_mint_rent,
    //         space: MINT_SIZE,
    //         owner: token_program.key(),
    //     }
    //     .invoke()?;

    //     // Initialize yes_mint
    //     InitializeMint2 {
    //         mint: yes_mint,
    //         decimals: 6,
    //         mint_authority: event.key(),
    //         freeze_authority: None,
    //     }
    //     .invoke()?;

    //     // Create no_mint account
    //     let no_mint_rent = rent.minimum_balance(MINT_SIZE as usize);
    //     CreateAccount {
    //         from: payer,
    //         to: no_mint,
    //         lamports: no_mint_rent,
    //         space: MINT_SIZE,
    //         owner: token_program.key(),
    //     }
    //     .invoke()?;

    //     // Initialize no_mint
    //     InitializeMint2 {
    //         mint: no_mint,
    //         decimals: 6,
    //         mint_authority: event.key(),
    //         freeze_authority: None,
    //     }
    //     .invoke()?;
    // }

    Ok(())
}
