//! Tests verifying buy quote monotonicity over increasing quantities (#157).
//!
//! The key price is fixed, so each successive buy costs exactly the same amount.
//! These tests assert that the quote price is non-decreasing (monotonic) across
//! a range of small and medium purchase scenarios, and that the total_amount
//! ordering is strictly deterministic.

mod contract_test_env;

use contract_test_env::{
    register_creator_keys, register_test_creator, set_pricing_and_fees, test_env_with_auths,
};
use creator_keys::CreatorKeysContractClient;
use soroban_sdk::{testutils::Address as _, Address, Env};

fn setup_with_fees<'a>(env: &'a Env, price: i128) -> (CreatorKeysContractClient<'a>, Address) {
    let (client, _) = register_creator_keys(env);
    set_pricing_and_fees(env, &client, price, 9000, 1000);
    let creator = register_test_creator(env, &client, "alice");
    (client, creator)
}

fn buy_n(
    client: &CreatorKeysContractClient<'_>,
    creator: &Address,
    buyer: &Address,
    n: u32,
    price: i128,
) {
    for _ in 0..n {
        client.buy_key(creator, buyer, &price);
    }
}

// ── Monotonicity: fixed price means each quote is identical ───────────��───

#[test]
fn test_buy_quote_is_identical_across_consecutive_calls() {
    let env = test_env_with_auths();
    let (client, creator) = setup_with_fees(&env, 100);

    let q1 = client.get_buy_quote(&creator).unwrap();
    let q2 = client.get_buy_quote(&creator).unwrap();
    let q3 = client.get_buy_quote(&creator).unwrap();

    assert_eq!(q1.price, q2.price);
    assert_eq!(q2.price, q3.price);
    assert_eq!(q1.total_amount, q2.total_amount);
    assert_eq!(q2.total_amount, q3.total_amount);
}

#[test]
fn test_buy_quote_price_unchanged_after_one_buy() {
    let env = test_env_with_auths();
    let (client, creator) = setup_with_fees(&env, 100);
    let buyer = Address::generate(&env);

    let before = client.get_buy_quote(&creator).unwrap();
    client.buy_key(&creator, &buyer, &100);
    let after = client.get_buy_quote(&creator).unwrap();

    assert_eq!(before.price, after.price);
    assert_eq!(before.total_amount, after.total_amount);
}

#[test]
fn test_buy_quote_price_unchanged_after_five_buys() {
    let env = test_env_with_auths();
    let (client, creator) = setup_with_fees(&env, 500);
    let buyer = Address::generate(&env);

    let before = client.get_buy_quote(&creator).unwrap();
    buy_n(&client, &creator, &buyer, 5, 500);
    let after = client.get_buy_quote(&creator).unwrap();

    assert_eq!(before.price, after.price, "price must be deterministic");
    assert_eq!(before.total_amount, after.total_amount);
}

#[test]
fn test_buy_quote_price_unchanged_across_multiple_buyers_small_range() {
    let env = test_env_with_auths();
    let price = 200_i128;
    let (client, creator) = setup_with_fees(&env, price);

    let q0 = client.get_buy_quote(&creator).unwrap();

    for _ in 0..10 {
        let buyer = Address::generate(&env);
        client.buy_key(&creator, &buyer, &price);
        let q = client.get_buy_quote(&creator).unwrap();
        assert_eq!(
            q.price, q0.price,
            "price must remain constant across buyers"
        );
    }
}

#[test]
fn test_buy_quote_total_amount_ordering_is_deterministic_small_range() {
    let env = test_env_with_auths();
    let (client, creator) = setup_with_fees(&env, 1_000);
    let buyer = Address::generate(&env);

    let q_start = client.get_buy_quote(&creator).unwrap();
    buy_n(&client, &creator, &buyer, 3, 1_000);
    let q_after = client.get_buy_quote(&creator).unwrap();

    // Fixed price: total_amount should be unchanged.
    assert_eq!(q_start.total_amount, q_after.total_amount);
    // Fees must be non-negative.
    assert!(q_after.creator_fee >= 0);
    assert!(q_after.protocol_fee >= 0);
}

#[test]
fn test_buy_quote_fees_sum_to_total_minus_price() {
    let env = test_env_with_auths();
    let (client, creator) = setup_with_fees(&env, 1_000);
    let buyer = Address::generate(&env);

    buy_n(&client, &creator, &buyer, 2, 1_000);
    let q = client.get_buy_quote(&creator).unwrap();

    // total_amount = price + creator_fee + protocol_fee for a buy quote
    assert_eq!(
        q.total_amount,
        q.price + q.creator_fee + q.protocol_fee,
        "buy quote: total_amount must equal price + all fees"
    );
}

// ── Medium input range ────────────────────────────────────────────────────

#[test]
fn test_buy_quote_stable_over_medium_volume_20_buys() {
    let env = test_env_with_auths();
    let price = 5_000_i128;
    let (client, creator) = setup_with_fees(&env, price);

    let base_quote = client.get_buy_quote(&creator).unwrap();

    for i in 0..20_u32 {
        let buyer = Address::generate(&env);
        client.buy_key(&creator, &buyer, &price);
        let q = client.get_buy_quote(&creator).unwrap();
        assert_eq!(
            q.price,
            base_quote.price,
            "quote price must be stable after {} buys",
            i + 1
        );
        assert_eq!(
            q.total_amount,
            base_quote.total_amount,
            "total_amount must be stable after {} buys",
            i + 1
        );
    }
}

#[test]
fn test_buy_quote_total_amount_never_below_price() {
    let env = test_env_with_auths();
    let (client, creator) = setup_with_fees(&env, 10_000);
    let buyer = Address::generate(&env);

    buy_n(&client, &creator, &buyer, 10, 10_000);

    let q = client.get_buy_quote(&creator).unwrap();
    assert!(
        q.total_amount >= q.price,
        "buy quote total_amount must be >= price (fees are additive)"
    );
}

// ── Different price points ────────────────────────────────────────────────

#[test]
fn test_buy_quote_price_point_1_is_stable() {
    let env = test_env_with_auths();
    let (client, creator) = setup_with_fees(&env, 1);

    let q1 = client.get_buy_quote(&creator).unwrap();
    let buyer = Address::generate(&env);
    client.buy_key(&creator, &buyer, &1);
    let q2 = client.get_buy_quote(&creator).unwrap();

    assert_eq!(q1.price, q2.price);
}

#[test]
fn test_buy_quote_price_point_large_is_stable() {
    let env = test_env_with_auths();
    let large_price = 1_000_000_i128;
    let (client, creator) = setup_with_fees(&env, large_price);

    let q_before = client.get_buy_quote(&creator).unwrap();
    let buyer = Address::generate(&env);
    client.buy_key(&creator, &buyer, &large_price);
    let q_after = client.get_buy_quote(&creator).unwrap();

    assert_eq!(q_before.price, q_after.price);
    assert_eq!(q_before.total_amount, q_after.total_amount);
}
