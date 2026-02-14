use blockchain_core::{
    accounts::event::EventState, error::MarketError, instructions::CloseEventArgs,
};
use pinocchio::{account_info::AccountInfo, ProgramResult};

use crate::utils::deserialize_and_check_event;

pub fn close_event(accounts: &[AccountInfo], args: &CloseEventArgs) -> ProgramResult {
    let [_payer, event] = accounts else {
        return Err(MarketError::InvalidAccounts.into());
    };

    // TODO: check that payer is admin

    let mut deser_event = deserialize_and_check_event(event, &args.uuid)?;

    deser_event.state = EventState::Finished;

    deser_event.write_into_bytes(&mut event.try_borrow_mut_data()?)?;

    Ok(())
}
