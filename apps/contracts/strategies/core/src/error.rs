use soroban_sdk::{self, contracterror};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum StrategyError {
    // Initialization Errors
    NotInitialized = 401,

    // Validation Errors
    NegativeNotAllowed = 410,
    InvalidArgument = 411,
    InsufficientBalance = 412,
    UnderflowOverflow= 413,
    ArithmeticError = 414,
    DivisionByZero = 415,
    InvalidSharesMinted= 416,
    OnlyPositiveAmountAllowed = 417,
    NotAuthorized = 418,

    // Protocol Errors
    ProtocolAddressNotFound = 420,
    DeadlineExpired = 421,
    ExternalError = 422,
    SoroswapPairError = 423,

    //Blend Errors
    AmountBelowMinDust = 451,
    UnderlyingAmountBelowMin = 452,
    BTokensAmountBelowMin = 453,
    InternalSwapError = 454,
    SupplyNotFound = 455,

}
