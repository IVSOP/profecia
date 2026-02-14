use std::collections::HashMap;

use uuid::Uuid;
use wincode::{SchemaRead, SchemaWrite};

use crate::{
    error::{MarketError, MarketResult},
    Pubkey,
};

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct Event {
    pub uuid: Uuid,
    pub description: String,
    pub state: EventState,
    // #[wincode(with = "std::collections::HashMap<wincode::containers::Pod<_>, _>")]
    pub options: HashMap<Uuid, EventOption>,
    pub bump: u8,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct EventOption {
    pub option_desc: String,
    // when compiling with pubkey from solana client, cant derive wincode on it
    #[wincode(with = "wincode::containers::Pod<_>")]
    pub yes_mint: Pubkey,
    // when compiling with pubkey from solana client, cant derive wincode on it
    #[wincode(with = "wincode::containers::Pod<_>")]
    pub no_mint: Pubkey,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub enum EventState {
    NotFinished,
    Finished,
}

impl Event {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MarketError> {
        wincode::deserialize(bytes).map_err(|_| MarketError::EventDeser)
    }

    pub fn len(&self) -> Result<u64, MarketError> {
        wincode::serialized_size(self).map_err(|_| MarketError::WincodeSize)
    }

    pub fn write_into_bytes(&self, bytes: &mut [u8]) -> MarketResult {
        // if you don't use a variable this no longer works, wtf??
        let mut buffer = &mut bytes[..];
        wincode::serialize_into(&mut buffer, self).map_err(|_| MarketError::EventSer)
    }

    pub fn find_program_address(uuid: &Uuid, marketplace_program: &Pubkey) -> (Pubkey, u8) {
        let seeds: &[&[u8]] = &[b"event", uuid.as_bytes()];

        #[cfg(not(feature = "client"))]
        return pinocchio::pubkey::find_program_address(&seeds, marketplace_program);

        #[cfg(feature = "client")]
        return solana_client::rpc_request::Address::find_program_address(
            seeds,
            marketplace_program,
        );
    }
}
