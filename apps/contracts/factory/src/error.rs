use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FactoryError {
    NotInitialized = 401,
    AssetLengthMismatch = 404,
    IndexDoesNotExist = 405,
    FeeTooHigh = 406,
}
