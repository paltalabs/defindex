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
    WrongAmuntsLength = 112,
    NotEnoughIdleFunds = 113,
    InsufficientManagedFunds = 114,

    // Arithmetic Errors (12x)
    ArithmeticError = 120,
    Overflow = 121,

    // Authorization/Role-based Errors (13x)
    Unauthorized = 130,
    RoleNotFound = 131,

    // Strategy Errors (14x)
    StrategyNotFound = 140,
    StrategyPausedOrNotFound = 141,

    // Asset Errors (15x)
    AssetNotFound = 150,
    NoAssetsProvided = 151,
}
