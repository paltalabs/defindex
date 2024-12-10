use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// The error codes for the contract.
pub enum PriceOracleError {
    /// The config assets don't contain persistent asset. Delete assets is not supported.
    AssetMissing = 2,
}
