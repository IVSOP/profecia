pub mod accounts;
pub mod error;
pub mod instructions;

// WARN: I don't know if wincode will shit itself because of this but it should be fine
#[cfg(feature = "client")]
pub type Pubkey = solana_client::rpc_request::Address;
#[cfg(not(feature = "client"))]
pub type Pubkey = pinocchio::pubkey::Pubkey;
