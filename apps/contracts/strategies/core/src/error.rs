use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum StrategyError {
    // Initialization Errors
    NotInitialized = 401,
    AlreadyInitialized = 402,

    // Validation Errors
    NegativeNotAllowed = 410,
    InvalidArgument = 411,

    // Protocol Errors
    ProtocolAddressNotFound = 420,
    DeadlineExpired = 421,
    ExternalError = 422,
}

