#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, token, Env};

fn create_token<'a>(env: &Env, admin: &Address) -> (Address, token::StellarAssetClient<'a>, token::Client<'a>) {
    let contract_address = env.register_stellar_asset_contract_v2(admin.clone()).address();
    let admin_client = token::StellarAssetClient::new(env, &contract_address);
    let client = token::Client::new(env, &contract_address);
    (contract_address, admin_client, client)
}

#[test]
fn create_and_withdraw() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let (token_address, token_admin, _token_client) = create_token(&env, &admin);
    token_admin.mint(&sender, &10_000);

    let contract_id = env.register(StreamFundContract, ());
    let client = StreamFundContractClient::new(&env, &contract_id);

    let id = client.create_stream(&sender, &recipient, &token_address, &1_000, &10);
    assert_eq!(client.balance(&id), 1_000);

    // advance ledger time by 50 seconds
    env.ledger().with_mut(|l| l.timestamp += 50);

    let withdrawn = client.withdraw(&id);
    assert_eq!(withdrawn, 500); // 50s * 10/s
    assert_eq!(client.balance(&id), 500);
}

#[test]
fn withdraw_caps_at_remaining_balance() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let recipient = Address::generate(&env);

    let (token_address, token_admin, _token_client) = create_token(&env, &admin);
    token_admin.mint(&sender, &10_000);

    let contract_id = env.register(StreamFundContract, ());
    let client = StreamFundContractClient::new(&env, &contract_id);

    let id = client.create_stream(&sender, &recipient, &token_address, &100, &10);

    // advance ledger time far beyond what the balance can cover
    env.ledger().with_mut(|l| l.timestamp += 1000);

    let withdrawn = client.withdraw(&id);
    assert_eq!(withdrawn, 100); // capped, not 10,000
    assert_eq!(client.balance(&id), 0);
}
