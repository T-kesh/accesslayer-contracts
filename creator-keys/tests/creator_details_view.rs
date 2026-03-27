//! Tests for the get_creator_details read-only method.

use creator_keys::{CreatorKeysContract, CreatorKeysContractClient};
use soroban_sdk::{testutils::Address as _, Env, String};

#[test]
fn test_get_creator_details_unregistered_returns_defaults() {
    let env = Env::default();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);
    let creator = soroban_sdk::Address::generate(&env);

    let details = client.get_creator_details(&creator);
    assert!(!details.is_registered);
    assert_eq!(details.creator, creator);
    assert_eq!(details.handle, String::from_str(&env, ""));
    assert_eq!(details.supply, 0);
}

#[test]
fn test_get_creator_details_registered_returns_correct_data() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);
    let creator = soroban_sdk::Address::generate(&env);
    let handle = String::from_str(&env, "alice");

    client.register_creator(&creator, &handle);

    let details = client.get_creator_details(&creator);
    assert!(details.is_registered);
    assert_eq!(details.creator, creator);
    assert_eq!(details.handle, handle);
    assert_eq!(details.supply, 0);
}

#[test]
fn test_get_creator_details_updates_after_buy() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);
    let admin = soroban_sdk::Address::generate(&env);
    let creator = soroban_sdk::Address::generate(&env);
    let buyer = soroban_sdk::Address::generate(&env);
    let handle = String::from_str(&env, "alice");

    client.register_creator(&creator, &handle);
    client.set_key_price(&admin, &100i128);

    client.buy_key(&creator, &buyer, &100i128);

    let details = client.get_creator_details(&creator);
    assert_eq!(details.supply, 1);
}
