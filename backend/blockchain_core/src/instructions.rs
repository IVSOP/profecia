use std::collections::HashMap;

use uuid::Uuid;
use wincode::{SchemaRead, SchemaWrite};

use crate::accounts::{event::EventOption, order::TokenOption};

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub enum MarketInstruction {
    CreateEvent(CreateEventArgs),
    CreateEmptyEvent(CreateEmptyEventArgs),
    CloseEvent(CloseEventArgs),
    // CreateOrder(CreateOrderArgs),
    // MatchOrder(MatchOrderArgs),
    // CancelOrder(CancelOrderArgs),
    FakeMatchOrder(FakeMatchOrderArgs),
    GetReward(GetRewardArgs),
    FakeCreateOrder(FakeCreateOrderArgs),
    FakeCancelOrder(FakeCancelOrderArgs),
    FakeGetReward(FakeGetRewardArgs),
    AddOption(AddOptionArgs),
    TransferShares(TransferSharesArgs),
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct CreateEventArgs {
    pub uuid: Uuid,
    pub description: String,
    pub options: HashMap<Uuid, EventOption>,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct CreateEmptyEventArgs {
    pub uuid: Uuid,
    pub description: String,
}

#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct CloseEventArgs {
    pub uuid: Uuid,
}

#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct CreateOrderArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub num_shares: u64,
    pub token: TokenOption,
    pub seed: Uuid,
    pub token_per_share: u64,
}

/// PRICE IN MICRO USDC!!!
#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct FakeMatchOrderArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub num_shares: u64,
}

/// PRICE IN MICRO USDC!!!
#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct FakeCreateOrderArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub num_shares: u64,
    pub price_per_share: u64,
}

/// PRICE IN MICRO USDC!!!
#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct FakeCancelOrderArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub num_shares: u64,
    pub price_per_share: u64,
}

#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct MatchOrderArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub num_shares: u64,
}

#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct CancelOrderArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub seed: u64,
}

#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct GetRewardArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub seed: u64,
}

#[derive(SchemaWrite, SchemaRead, Debug, Copy, Clone)]
pub struct FakeGetRewardArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub num_shares: u64,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct AddOptionArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub option_info: EventOption,
}

#[derive(SchemaWrite, SchemaRead, Debug, Clone)]
pub struct TransferSharesArgs {
    pub event_uuid: Uuid,
    pub option_uuid: Uuid,
    pub token_option: TokenOption,
    pub num_shares: u64,
    pub price_per_share: u64,
}
