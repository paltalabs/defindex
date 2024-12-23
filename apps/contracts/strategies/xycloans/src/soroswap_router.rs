use crate::storage::{get_soroswap_factory_address, get_soroswap_router_address};
use soroban_sdk::auth::{ContractContext, InvokerContractAuthEntry, SubContractInvocation};
use soroban_sdk::crypto::Hash;
use soroban_sdk::{vec, Address, Env, IntoVal, Symbol, Val, Vec};
use soroban_sdk::{xdr::ToXdr, Bytes};

soroban_sdk::contractimport!(file = "../external_wasms/soroswap/soroswap_router.optimized.wasm");
pub type SoroswapRouterClient<'a> = Client<'a>;

mod pair {
    soroban_sdk::contractimport!(file = "../external_wasms/soroswap/soroswap_pair.optimized.wasm");
}
use pair::Client as SoroswapPairClient;

/// Generates a unique cryptographic salt value for a pair of token addresses.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `token_a` - The address of the first token.
/// * `token_b` - The address of the second token.
///
/// # Returns
///
/// Returns a `BytesN<32>` representing the salt for the given token pair.
fn pair_salt(e: &Env, token_a: Address, token_b: Address) -> Hash<32> {
    let mut salt = Bytes::new(e);

    // Append the bytes of token_a and token_b to the salt
    salt.append(&token_a.clone().to_xdr(e)); // can be simplified to salt.append(&self.clone().to_xdr(e)); but changes the hash
    salt.append(&token_b.clone().to_xdr(e));

    // Hash the salt using SHA256 to generate a new BytesN<32> value
    e.crypto().sha256(&salt)
}

/// Sorts two token addresses in a consistent order.
///
/// # Arguments
///
/// * `token_a` - The address of the first token.
/// * `token_b` - The address of the second token.
///
/// # Returns
///
/// Returns `Result<(Address, Address), SoroswapLibraryError>` where `Ok` contains a tuple with the sorted token addresses, and `Err` indicates an error such as identical tokens.
pub fn sort_tokens(
    token_a: Address,
    token_b: Address,
) -> Result<(Address, Address), SoroswapLibraryError> {
    if token_a == token_b {
        return Err(SoroswapLibraryError::SortIdenticalTokens);
    }

    if token_a < token_b {
        Ok((token_a, token_b))
    } else {
        Ok((token_b, token_a))
    }
}

/// Calculates the deterministic address for a pair without making any external calls.
/// check <https://github.com/paltalabs/deterministic-address-soroban>
///
/// # Arguments
///
/// * `e` - The environment.
/// * `factory` - The factory address.
/// * `token_a` - The address of the first token.
/// * `token_b` - The address of the second token.
///
/// # Returns
///
/// Returns `Result<Address, SoroswapLibraryError>` where `Ok` contains the deterministic address for the pair, and `Err` indicates an error such as identical tokens or an issue with sorting.
pub fn pair_for(
    e: Env,
    factory: Address,
    token_a: Address,
    token_b: Address,
) -> Result<Address, SoroswapLibraryError> {
    let (token_0, token_1) = sort_tokens(token_a, token_b)?;
    let salt = pair_salt(&e, token_0, token_1);
    let deployer_with_address = e.deployer().with_address(factory.clone(), salt);
    let deterministic_address = deployer_with_address.deployed_address();
    Ok(deterministic_address)
}

/// Fetches and sorts the reserves for a pair of tokens.
///
/// # Arguments
///
/// * `e` - The environment.
/// * `factory` - The factory address.
/// * `token_a` - The address of the first token.
/// * `token_b` - The address of the second token.
///
/// # Returns
///
/// Returns `Result<(i128, i128), SoroswapLibraryError>` where `Ok` contains a tuple of sorted reserves, and `Err` indicates an error such as identical tokens or an issue with sorting.
pub fn get_reserves(
    e: Env,
    factory: Address,
    token_a: Address,
    token_b: Address,
) -> Result<(i128, i128), SoroswapLibraryError> {
    let (token_0, token_1) = sort_tokens(token_a.clone(), token_b.clone())?;
    let pair_address = pair_for(e.clone(), factory, token_0.clone(), token_1.clone())?;
    let pair_client = SoroswapPairClient::new(&e, &pair_address);
    let (reserve_0, reserve_1) = pair_client.get_reserves();

    let (reserve_a, reseve_b) = if token_a == token_0 {
        (reserve_0, reserve_1)
    } else {
        (reserve_1, reserve_0)
    };

    Ok((reserve_a, reseve_b))
}

/// Given an input amount of an asset and pair reserves, returns the maximum output amount of the other asset.
///
/// # Arguments
///
/// * `amount_in` - The input amount of the asset.
/// * `reserve_in` - Reserves of the input asset in the pair.
/// * `reserve_out` - Reserves of the output asset in the pair.
///
/// # Returns
///
/// Returns `Result<i128, SoroswapLibraryError>` where `Ok` contains the calculated maximum output amount, and `Err` indicates an error such as insufficient input amount or liquidity.
pub fn get_amount_out(
    amount_in: i128,
    reserve_in: i128,
    reserve_out: i128,
) -> Result<i128, SoroswapLibraryError> {
    if amount_in <= 0 {
        return Err(SoroswapLibraryError::InsufficientInputAmount);
    }
    if reserve_in <= 0 || reserve_out <= 0 {
        return Err(SoroswapLibraryError::InsufficientLiquidity);
    }

    let fee = (amount_in.checked_mul(3).unwrap())
        .checked_ceiling_div(1000)
        .unwrap();

    let amount_in_less_fee = amount_in.checked_sub(fee).unwrap();
    let numerator = amount_in_less_fee.checked_mul(reserve_out).unwrap();

    let denominator = reserve_in.checked_add(amount_in_less_fee).unwrap();

    Ok(numerator.checked_div(denominator).unwrap())
}

pub trait CheckedCeilingDiv {
    fn checked_ceiling_div(self, divisor: i128) -> Option<i128>;
}

impl CheckedCeilingDiv for i128 {
    fn checked_ceiling_div(self, divisor: i128) -> Option<i128> {
        let result = self.checked_div(divisor)?;
        if self % divisor != 0 {
            result.checked_add(1)
        } else {
            Some(result)
        }
    }
}

pub fn swap(
    e: &Env,
    from: &Address,
    token_in: &Address,
    token_out: &Address,
    amount_in: &i128,
) -> i128 {
    // Setting up Soroswap router client
    let soroswap_router_address = get_soroswap_router_address(e);
    let soroswap_router_client = SoroswapRouterClient::new(e, &soroswap_router_address);

    // This could be hardcoded to perform less instructions
    let soroswap_pair_address = get_soroswap_factory_address(e);
    let pair_address = pair_for(
        e.clone(),
        soroswap_pair_address.clone(),
        token_out.clone(),
        token_in.clone(),
    );

    let mut path: Vec<Address> = Vec::new(e);
    path.push_back(token_in.clone());
    path.push_back(token_out.clone());

    let mut args: Vec<Val> = vec![e];
    args.push_back(from.into_val(e));
    args.push_back(pair_address.into_val(e));
    args.push_back(amount_in.into_val(e));

    e.authorize_as_current_contract(vec![
        &e,
        InvokerContractAuthEntry::Contract(SubContractInvocation {
            context: ContractContext {
                contract: token_in.clone(),
                fn_name: Symbol::new(&e, "transfer"),
                args: args.clone(),
            },
            sub_invocations: vec![&e],
        }),
    ]);

    // let swap_amount = amount_in/2;
    // e.current_contract_address().require_auth();
    let res = soroswap_router_client.swap_exact_tokens_for_tokens(
        &amount_in,
        &0,
        &path,
        &from,
        &u64::MAX,
    );

    let total_swapped_amount = res.last().unwrap_or(0);

    total_swapped_amount
}
