use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum FactoryError {
    NotInitialized = 401,
    AlreadyInitialized = 402,
    EmptyMap = 403,
    AssetLengthMismatch = 404,
    IndexDoesNotExist = 405,
    FeeTooHigh = 406,
}
