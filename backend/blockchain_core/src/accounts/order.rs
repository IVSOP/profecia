use uuid::Uuid;
use wincode::{SchemaRead, SchemaWrite};

use crate::{
    error::{MarketError, MarketResult},
    Pubkey,
};

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct Order {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    #[wincode(with = "wincode::containers::Pod<_>")]
    // when compiling with pubkey from solana client, cant derive wincode on it
    pub user: Pubkey,
    pub token: TokenOption,
    pub seed: Uuid,
}

#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub enum TokenOption {
    Yes,
    No,
}

impl Order {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, MarketError> {
        wincode::deserialize(bytes).map_err(|_| MarketError::OrderDeser)
    }

    pub fn len(&self) -> Result<u64, MarketError> {
        wincode::serialized_size(self).map_err(|_| MarketError::WincodeSize)
    }

    pub fn write_into_bytes(&self, bytes: &mut [u8]) -> MarketResult {
        // if you don't use a variable this no longer works, wtf??
        let mut buffer = &mut bytes[..];
        wincode::serialize_into(&mut buffer, self).map_err(|_| MarketError::OrderSer)
    }

    pub fn find_program_address(
        event_id: &Uuid,
        option_id: &Uuid,
        seed: &Uuid,
        user: &Pubkey,
        token: &Pubkey,
        marketplace_program: &Pubkey,
    ) -> (Pubkey, u8) {
        let seeds: &[&[u8]] = &[
            b"order",
            &event_id.into_bytes(),
            &option_id.into_bytes(),
            user.as_ref(),
            token.as_ref(),
            &seed.into_bytes(),
        ];

        #[cfg(not(feature = "client"))]
        return pinocchio::pubkey::find_program_address(&seeds, marketplace_program);

        #[cfg(feature = "client")]
        return solana_client::rpc_request::Address::find_program_address(
            seeds,
            marketplace_program,
        );
    }
}
