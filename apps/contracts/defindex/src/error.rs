use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Initialization Errors (1xx)
    NotInitialized = 100,
    AlreadyInitialized = 101,

    // Validation Errors (2xx)
    NegativeNotAllowed = 200,

    // Arithmetic Errors (3xx)
    ArithmeticError = 300,

    // Authorization/Role-based Errors (4xx)
    Unauthorized = 400,
    RoleNotFound = 401,
}
