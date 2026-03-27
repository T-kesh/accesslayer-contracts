use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

#[test]
fn test_register_creator() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let handle = String::from_str(&env, "alice");

    client.register_creator(&creator, &handle);

    let profile = client.get_creator(&creator);
    assert_eq!(profile.handle, handle);
    assert_eq!(profile.creator, creator);
    assert_eq!(profile.supply, 0);
}

#[test]
fn test_duplicate_registration_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let handle = String::from_str(&env, "alice");

    client.register_creator(&creator, &handle);

    // Second registration should fail with AlreadyRegistered error
    let result = client.try_register_creator(&creator, &handle);
    assert_eq!(result, Err(Ok(ContractError::AlreadyRegistered)));
}

#[test]
fn test_buy_key_fails_if_not_registered() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_key_price(&admin, &100);

    let creator = Address::generate(&env);
    let buyer = Address::generate(&env);

    let result = client.try_buy_key(&creator, &buyer, &100);
    assert_eq!(result, Err(Ok(ContractError::NotRegistered)));
}

#[test]
fn test_buy_key_success() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_key_price(&admin, &100);

    let creator = Address::generate(&env);
    let handle = String::from_str(&env, "alice");
    client.register_creator(&creator, &handle);

    let buyer = Address::generate(&env);
    let supply = client.buy_key(&creator, &buyer, &100);
    assert_eq!(supply, 1);

    let profile = client.get_creator(&creator);
    assert_eq!(profile.supply, 1);
}

#[test]
fn test_get_creator_fails_if_not_registered() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);

    let result = client.try_get_creator(&creator);
    assert_eq!(result, Err(Ok(ContractError::NotRegistered)));
}

#[test]
fn test_buy_key_insufficient_payment() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    client.set_key_price(&admin, &100);

    let creator = Address::generate(&env);
    let handle = String::from_str(&env, "alice");
    client.register_creator(&creator, &handle);

    let buyer = Address::generate(&env);
    let result = client.try_buy_key(&creator, &buyer, &99);
    assert_eq!(result, Err(Ok(ContractError::InsufficientPayment)));
}

#[test]
fn test_set_key_price_invalid_amount() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let result = client.try_set_key_price(&admin, &0);
    assert_eq!(result, Err(Ok(ContractError::NotPositiveAmount)));

    let result = client.try_set_key_price(&admin, &-1);
    assert_eq!(result, Err(Ok(ContractError::NotPositiveAmount)));
}

#[test]
fn test_get_key_balance_returns_zero_for_unregistered_creator() {
    let env = Env::default();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let unregistered_creator = Address::generate(&env);
    let wallet = Address::generate(&env);

    let balance = client.get_key_balance(&unregistered_creator, &wallet);
    assert_eq!(balance, 0);
}

#[test]
fn test_is_creator_registered_returns_false_for_unregistered() {
    let env = Env::default();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let unregistered_creator = Address::generate(&env);

    let is_registered = client.is_creator_registered(&unregistered_creator);
    assert!(!is_registered);
}

#[test]
fn test_get_total_key_supply_returns_zero_for_unregistered() {
    let env = Env::default();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let unregistered_creator = Address::generate(&env);

    let supply = client.get_total_key_supply(&unregistered_creator);
    assert_eq!(supply, 0);
}

#[test]
fn test_get_key_balance_returns_zero_for_unregistered_wallet() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    let handle = String::from_str(&env, "alice");
    client.register_creator(&creator, &handle);

    let unregistered_wallet = Address::generate(&env);

    let balance = client.get_key_balance(&creator, &unregistered_wallet);
    assert_eq!(balance, 0);
}

#[test]
fn test_get_creator_fee_config_returns_defaults_for_unregistered() {
    let env = Env::default();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let unregistered_creator = Address::generate(&env);

    let fee_view = client.get_creator_fee_config(&unregistered_creator);
    assert!(!fee_view.is_registered);
    assert!(!fee_view.is_configured);
    assert_eq!(fee_view.creator_bps, 0);
    assert_eq!(fee_view.protocol_bps, 0);
}

#[test]
fn test_get_treasury_address_returns_none_initially() {
    let env = Env::default();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let result = client.get_treasury_address();
    assert_eq!(result, None);
}

#[test]
fn test_get_treasury_address_returns_set_address() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);
    client.set_treasury_address(&admin, &treasury);

    let result = client.get_treasury_address();
    assert_eq!(result, Some(treasury));
}

#[test]
fn test_get_treasury_address_persists_across_reads() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let treasury = Address::generate(&env);
    client.set_treasury_address(&admin, &treasury);

    let first_read = client.get_treasury_address();
    let second_read = client.get_treasury_address();
    assert_eq!(first_read, second_read);
    assert_eq!(first_read, Some(treasury));
}
