//! Tests for the `get_protocol_treasury_share_bps` read-only method.

use creator_keys::{CreatorKeysContract, CreatorKeysContractClient};
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_get_protocol_treasury_share_bps_returns_configured_value() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.set_fee_config(&admin, &8000u32, &2000u32);

    assert_eq!(client.get_protocol_treasury_share_bps(), 2000);
}

#[test]
fn test_get_protocol_treasury_share_bps_is_read_only() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(CreatorKeysContract, ());
    let client = CreatorKeysContractClient::new(&env, &contract_id);
    let admin = Address::generate(&env);

    client.set_fee_config(&admin, &7000u32, &3000u32);

    let first = client.get_protocol_treasury_share_bps();
    let second = client.get_protocol_treasury_share_bps();

    assert_eq!(first, second);
    assert_eq!(first, 3000);
}
