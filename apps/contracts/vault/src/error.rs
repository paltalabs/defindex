use soroban_sdk::{self, contracterror};
use soroswap_library::SoroswapLibraryError;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum ContractError {
    // Initialization Errors (10x)
    NotInitialized = 100,
    InvalidRatio = 101,
    StrategyDoesNotSupportAsset = 102,
    NoAssetAllocation = 103,
    RolesIncomplete = 104,
    MetadataIncomplete = 105,

    // Validation Errors (11x)
    NegativeNotAllowed = 110,
    InsufficientBalance = 111,
    WrongAmountsLength = 112,
    NotEnoughIdleFunds = 113,
    InsufficientManagedFunds = 114,
    MissingInstructionData = 115,
    UnsupportedAsset = 116,
    InsufficientAmount = 117,
    NoOptimalAmounts = 118, //this should not happen
    WrongInvestmentLength = 119,
    WrongAssetAddress = 122,
    WrongStrategiesLength = 123,
    AmountOverTotalSupply = 124,
    NoInstructions = 125,
    NotUpgradable = 126,

    // Arithmetic Errors (12x)
    ArithmeticError = 120,
    Overflow = 121,

    // Authorization/Role-based Errors (13x)
    Unauthorized = 130,
    RoleNotFound = 131,

    // Strategy Errors (14x)
    StrategyNotFound = 140,
    StrategyPausedOrNotFound = 141,
    StrategyWithdrawError = 142,
    StrategyInvestError = 143,
    StrategyPaused = 144,

    // Asset Errors (15x)
    AssetNotFound = 150,
    NoAssetsProvided = 151,

    // Add mappings for SoroswapLibraryError
    LibrarySortIdenticalTokens = 190,
}

impl From<SoroswapLibraryError> for ContractError {
    fn from(err: SoroswapLibraryError) -> Self {
        match err {
            SoroswapLibraryError::SortIdenticalTokens => ContractError::LibrarySortIdenticalTokens,
            _ => panic!("Unhandled SoroswapLibraryError variant: {:?}", err),
        }
    }
}
