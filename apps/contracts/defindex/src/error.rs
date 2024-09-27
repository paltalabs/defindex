use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Initialization Errors (10x)
    NotInitialized = 100,
    AlreadyInitialized = 101,
    InvalidRatio = 102,
    StrategyDoesNotSupportAsset=103,

    // Validation Errors (11x)
    NegativeNotAllowed = 110,
    InsufficientBalance = 111,

    // Arithmetic Errors (12x)
    ArithmeticError = 120,

    // Authorization/Role-based Errors (13x)
    Unauthorized = 130,
    RoleNotFound = 131,

}
