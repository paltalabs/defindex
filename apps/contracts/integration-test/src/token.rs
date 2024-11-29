use soroban_sdk::{token::{TokenClient as SorobanTokenClient, StellarAssetClient as SorobanTokenAdminClient}, Address, Env};

fn create_token_contract<'a>(e: &Env, admin: &Address) -> SorobanTokenClient<'a> {
    SorobanTokenClient::new(e, &e.register_stellar_asset_contract_v2(admin.clone()).address())
}

fn get_token_admin_client<'a>(
    e: &Env,
    address: &Address,
) -> SorobanTokenAdminClient<'a> {
    SorobanTokenAdminClient::new(e, address)
}

pub fn create_token<'a>(e: &Env, admin: &Address) -> (SorobanTokenClient<'a>, SorobanTokenAdminClient<'a>) {
    let token = create_token_contract(e, admin);
        
    let token_admin_client = get_token_admin_client(e, &token.address);

    (token, token_admin_client)
}