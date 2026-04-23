//! Tests for zero-amount quote normalization in buy and sell quote paths.

use creator_keys::{constants, CreatorKeysContract, CreatorKeysContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn store_key_price(env: &Env, contract_id: &Address, price: i128) {
    env.as_contract(contract_id, || {
        env.storage()
            .persistent()
            .set(&constants::storage::KEY_PRICE, &price);
    });
}

#[test]
fn test_get_buy_quote_zero_amount_returns_noop_quote() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let creator = Address::generate(&env);
    client.register_creator(&creator, &String::from_str(&env, "alice"));
    store_key_price(&env, &contract_id, 0);

    let quote = client.get_buy_quote(&creator);
    assert_eq!(quote.price, 0);
    assert_eq!(quote.creator_fee, 0);
    assert_eq!(quote.protocol_fee, 0);
    assert_eq!(quote.total_amount, 0);
}

#[test]
fn test_get_sell_quote_zero_amount_returns_noop_quote() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let creator = Address::generate(&env);
    let holder = Address::generate(&env);

    client.set_key_price(&admin, &100);
    client.register_creator(&creator, &String::from_str(&env, "alice"));
    client.buy_key(&creator, &holder, &100);
    store_key_price(&env, &contract_id, 0);

    let quote = client.get_sell_quote(&creator, &holder);
    assert_eq!(quote.price, 0);
    assert_eq!(quote.creator_fee, 0);
    assert_eq!(quote.protocol_fee, 0);
    assert_eq!(quote.total_amount, 0);
}
