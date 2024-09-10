use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Initialization Errors (11xx)
    NotInitialized = 1100,
    AlreadyInitialized = 1101,
    InvalidRatio = 1102,

    // Validation Errors (12xx)
    NegativeNotAllowed = 1200,

    // Arithmetic Errors (13xx)
    ArithmeticError = 1300,

    // Authorization/Role-based Errors (14xx)
    Unauthorized = 1400,
    RoleNotFound = 1401,
}
