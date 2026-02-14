//! This module defines an error type that can also be used in the program

use pinocchio::program_error::{ProgramError, ToStr};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MarketError {
    EventSer = 0,
    EventDeser = 1,
    OrderSer = 2,
    OrderDeser = 3,
    WincodeSize = 4,
    InstructionDeser = 5,
    InvalidAccounts = 6,
    UsdcMint = 7,
    MarketPDA = 8,
    MarketID = 9,
    InvalidEvent = 10,
    OrderPDA = 11,
    OptionMissmatch = 12,
    TokenMissmatch = 13,
}

pub type MarketResult = Result<(), MarketError>;

impl From<MarketError> for ProgramError {
    fn from(e: MarketError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl ToStr for MarketError {
    fn to_str<E>(&self) -> &'static str
    where
        E: 'static + ToStr + TryFrom<u32>,
    {
        match self {
            Self::EventSer => "Error serializing event",
            Self::EventDeser => "Error deserializing event",
            Self::OrderSer => "Error serializing vault",
            Self::OrderDeser => "Error deserializing vault",
            Self::WincodeSize => "Error getting wincode serialized size",
            Self::InstructionDeser => "Error deserializing instruction",
            Self::InvalidAccounts => "Accounts passed to instruction were invalid",
            Self::UsdcMint => "USDC mint does not match",
            Self::MarketPDA => "Market PDA is wrong",
            Self::MarketID => "Invalid market ID",
            Self::InvalidEvent => "Invalid event account",
            Self::OrderPDA => "Invalid order PDA",
            Self::OptionMissmatch => "Option missmatch",
            Self::TokenMissmatch => "Token missmatch",
        }
    }
}

impl TryFrom<u32> for MarketError {
    type Error = ProgramError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::EventSer),
            1 => Ok(Self::EventDeser),
            2 => Ok(Self::OrderSer),
            3 => Ok(Self::OrderDeser),
            4 => Ok(Self::WincodeSize),
            5 => Ok(Self::InstructionDeser),
            6 => Ok(Self::InvalidAccounts),
            7 => Ok(Self::UsdcMint),
            8 => Ok(Self::MarketPDA),
            9 => Ok(Self::MarketID),
            10 => Ok(Self::InvalidEvent),
            11 => Ok(Self::OrderPDA),
            12 => Ok(Self::OptionMissmatch),
            13 => Ok(Self::TokenMissmatch),
            _ => Err(ProgramError::Custom(u32::MAX)),
        }
    }
}

#[cfg(feature = "client")]
impl std::fmt::Display for MarketError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str::<MarketError>())
    }
}

#[cfg(feature = "client")]
impl std::error::Error for MarketError {}
