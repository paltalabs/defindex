use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    NotInitialized = 401,
    AlreadyInitialized = 402,
    NegativeNotAllowed = 403,
    ProtocolAddressNotFound = 404,
    DeadlineExpired = 405,
    NeedToWait = 406,
    ArithmeticError = 407,
    LengthMismatch = 408,
}
