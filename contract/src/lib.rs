#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn setup_test() -> (
    Env,
    AyudaContractClient<'static>,
    Address,
    Address,
    token::Client<'static>,
) {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::from_string(&String::from_str(
        &env,
        "GCJJ7WCTRWLR7YLOWZH6VGCYKZ62HG2N7US7AUQPT762GDN7HFA4Y7Q5",
    ));
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin);
    let token_client = token::Client::new(&env, &token_addr);

    let contract_id = env.register_contract(None, AyudaContract);
    let client = AyudaContractClient::new(&env, &contract_id);

    (env, client, admin, token_addr, token_client)
}

#[test]
fn test_1_initialization() {
    let (env, client, admin, token_addr, _) = setup_test();
    client.init(&admin, &token_addr);
    // Should fail if initialized twice
    assert!(client.try_init(&admin, &token_addr).is_err());
}

#[test]
fn test_2_admin_registration() {
    let (env, client, admin, _, _) = setup_test();
    client.init(&admin, &Address::generate(&env));

    let user = Address::generate(&env);
    let nfc = String::from_str(&env, "TAG_001");
    client.register_citizen(&admin, &user, &nfc, &String::from_str(&env, "User1"));
}

#[test]
fn test_3_non_admin_registration_fails() {
    let (env, client, admin, _, _) = setup_test();
    client.init(&admin, &Address::generate(&env));

    let hacker = Address::generate(&env);
    let nfc = String::from_str(&env, "TAG_HACK");
    // Should fail because 'hacker' is not the registered admin
    let result =
        client.try_register_citizen(&hacker, &hacker, &nfc, &String::from_str(&env, "Hacker"));
    assert!(result.is_err());
}

#[test]
fn test_4_funding_and_balance_check() {
    let (env, client, admin, _, _) = setup_test();
    client.init(&admin, &Address::generate(&env));
    let user = Address::generate(&env);
    client.register_citizen(
        &admin,
        &user,
        &String::from_str(&env, "TAG1"),
        &String::from_str(&env, "User1"),
    );

    client.fund_aid(&admin, &user, &500);
    assert_eq!(client.get_balance(&user), 500);
}

#[test]
fn test_5_claim_security_id_mismatch() {
    let (env, client, admin, _, _) = setup_test();
    client.init(&admin, &Address::generate(&env));
    let user = Address::generate(&env);
    let nfc = String::from_str(&env, "REAL_TAG");
    client.register_citizen(&admin, &user, &nfc, &String::from_str(&env, "User1"));
    client.fund_aid(&admin, &user, &100);

    // Attempt claim with WRONG NFC ID
    let wrong_nfc = String::from_str(&env, "FAKE_TAG");
    assert!(client.try_claim_aid(&user, &wrong_nfc).is_err());
}

#[test]
fn test_6_successful_claim() {
    let (env, client, admin, token_addr, token_client) = setup_test();
    client.init(&admin, &token_addr);
    let user = Address::generate(&env);
    let nfc = String::from_str(&env, "TAG_OK");

    client.register_citizen(&admin, &user, &nfc, &String::from_str(&env, "User1"));
    client.fund_aid(&admin, &user, &200);

    token_client.mint(&client.address, &200);
    client.claim_aid(&user, &nfc);

    assert_eq!(token_client.balance(&user), 200);
    assert_eq!(client.get_balance(&user), 0);
}

#[test]
fn test_7_duplicate_nfc_prevention() {
    let (env, client, admin, _, _) = setup_test();
    client.init(&admin, &Address::generate(&env));
    let nfc = String::from_str(&env, "ONE_TIME_ID");

    client.register_citizen(
        &admin,
        &Address::generate(&env),
        &nfc,
        &String::from_str(&env, "UserA"),
    );
    // Attempting to link the same NFC to UserB should fail
    let res = client.try_register_citizen(
        &admin,
        &Address::generate(&env),
        &nfc,
        &String::from_str(&env, "UserB"),
    );
    assert!(res.is_err());
}

