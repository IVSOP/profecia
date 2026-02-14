#![allow(unused)]

use blockchain_core::{
    accounts::event::Event,
    error::{MarketError, MarketResult},
};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::{pubkey_eq, Pubkey},
    ProgramResult,
};
use pinocchio_pubkey::pubkey;
use uuid::Uuid;

use crate::USDC_ADDRESS;

pub const TOKEN_PROGRAM: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
pub const ASSOCIATED_TOKEN_PROGRAM: Pubkey =
    pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

/// Check that account is a signer
pub fn must_be_signer(account: &AccountInfo) -> ProgramResult {
    if !account.is_signer() {
        Err(ProgramError::MissingRequiredSignature)
    } else {
        Ok(())
    }
}

/// Returns Ok is the account does not exist
pub fn must_be_uninit(account: &AccountInfo) -> ProgramResult {
    if account.data_is_empty()
        && account.lamports() == 0
        && pubkey_eq(account.owner(), &pinocchio_system::ID)
    {
        Ok(())
    } else {
        Err(ProgramError::AccountAlreadyInitialized)
    }
}

pub fn check_token_program(account: &AccountInfo) -> ProgramResult {
    if !pubkey_eq(&TOKEN_PROGRAM, account.key()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

pub fn check_associated_token_program(account: &AccountInfo) -> ProgramResult {
    if !pubkey_eq(&ASSOCIATED_TOKEN_PROGRAM, account.key()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

pub fn deserialize_and_check_event(
    event: &AccountInfo,
    uuid: &Uuid,
) -> Result<Event, ProgramError> {
    let deser_event = Event::from_bytes(&event.try_borrow_data()?)?;

    if event.is_owned_by(&crate::ID) && event.lamports() != 0 && uuid == &deser_event.uuid {
        Ok(deser_event)
    } else {
        Err(MarketError::InvalidEvent)?
    }
}

pub fn check_usdc(usdc: &AccountInfo) -> ProgramResult {
    if !pubkey_eq(usdc.key(), &USDC_ADDRESS) {
        return Err(MarketError::UsdcMint)?;
    }

    Ok(())
}

// /// Deserializes a lobby, checking that the account is valid
// /// - Owner must be the game program
// /// - PDA does not need to be checked, as the inner fields are checked
// /// - The lobby ID is checked
// /// The bytes must be passed in from outside so we can use zero-copy
// pub fn deser_and_check_lobby<'a>(
//     bytes: &'a [u8],
//     lobby: &'a AccountInfo,
//     lobby_id: u64,
// ) -> Result<LobbyAccount<'a>, MarkError> {
//     if !lobby.is_owned_by(&crate::ID) {
//         return Err(GameError::InvalidLobby);
//     }

//     if lobby.lamports() == 0 {
//         return Err(GameError::InvalidLobby);
//     }

//     let deser = LobbyAccount::from_bytes(bytes)?;

//     if deser.header.lobby != lobby_id {
//         return Err(GameError::InvalidLobby);
//     }

//     Ok(deser)
// }

// /// Deserializes a player inputs account, checking that it is valid
// /// - Header must have correct account type (checked in from_bytes)
// /// - Owner must be the game program
// /// - PDA does not need to be checked, as the inner fields are checked
// /// - The lobby ID is checked
// /// - The player is checked
// /// The bytes must be passed in from outside so we can use zero-copy
// pub fn deser_and_check_player_inputs<'a>(
//     bytes: &'a [u8],
//     inputs_account: &'a AccountInfo,
//     lobby_id: u64,
//     player: &Pubkey,
// ) -> Result<PlayerInputsAccount<'a>, GameError> {
//     let deser = PlayerInputsAccount::from_bytes(bytes)?;

//     if inputs_account.is_owned_by(&crate::ID)
//         && inputs_account.lamports() != 0
//         && deser.header.lobby == lobby_id
//         && pubkey_eq(&deser.header.player, player)
//     {
//         Ok(deser)
//     } else {
//         Err(GameError::InvalidPlayerInputs)
//     }
// }

/// Faster than using .contains() because this uses pubkey_eq
pub fn pubkey_is_contained_in(pubkey: &Pubkey, keys: &[Pubkey]) -> bool {
    keys.iter().any(|slice_key| pubkey_eq(slice_key, pubkey))
}

// PERF: I am assuming this is zero cost
// I just made this to make zipping more readable with many slices
#[inline(always)]
pub fn zip3<A, B, C, IA, IB, IC>(a: IA, b: IB, c: IC) -> impl Iterator<Item = (A, B, C)>
where
    IA: IntoIterator<Item = A>,
    IB: IntoIterator<Item = B>,
    IC: IntoIterator<Item = C>,
{
    a.into_iter().zip(b).zip(c).map(|((a, b), c)| (a, b, c))
}
