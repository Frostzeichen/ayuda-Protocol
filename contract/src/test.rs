use super::*;
use soroban_sdk::{
    testutils::{Address as _, Events},
    Address, Env, String,
};

#[test]
fn test_ayuda_flow() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let malicious_user = Address::generate(&env);

    // 1. Setup Mock Token (XLM/Custom)
    let token_admin = Address::generate(&env);
    let token_addr = env.register_stellar_asset_contract(token_admin.clone());
    let token_client = token::Client::new(&env, &token_addr);

    // 2. Register and Initialize Contract
    let contract_id = env.register_contract(None, AyudaContract);
    let client = AyudaContractClient::new(&env, &contract_id);

    client.init(&admin, &token_addr);

    // 3. Fund the Contract (The Pool)
    token_client.mint(&contract_id, &1000);

    let nfc_id = String::from_str(&env, "NFC_TAG_001");
    let name = String::from_str(&env, "Richie");

    // 4. Test Registration
    client.register_citizen(&admin, &beneficiary, &nfc_id, &name);

    // 5. Test Funding Citizen Aid
    client.fund_aid(&admin, &beneficiary, &100);
    assert_eq!(client.get_balance(&beneficiary), 100);

    // 6. Test Security: Malicious user tries to claim with correct NFC ID
    // This should fail because the NFC mapping belongs to 'beneficiary'
    let claim_result_fail = client.try_claim_aid(&malicious_user, &nfc_id);
    assert!(claim_result_fail.is_err());

    // 7. Test Success: Real beneficiary claims aid
    client.claim_aid(&beneficiary, &nfc_id);

    // 8. Verify Balances
    assert_eq!(token_client.balance(&beneficiary), 100); // Beneficiary got the money
    assert_eq!(client.get_balance(&beneficiary), 0); // Contract record cleared
    assert_eq!(token_client.balance(&contract_id), 900); // Pool deducted
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #7)")] // IdAlreadyLinked
fn test_prevent_duplicate_nfc_link() {
    let env = Env::default();
    env.mock_all_auths();

    let admin = Address::generate(&env);
    let user_a = Address::generate(&env);
    let user_b = Address::generate(&env);

    let token_addr = env.register_stellar_asset_contract(Address::generate(&env));
    let contract_id = env.register_contract(None, AyudaContract);
    let client = AyudaContractClient::new(&env, &contract_id);

    client.init(&admin, &token_addr);

    let nfc_id = String::from_str(&env, "SAME_TAG");
    let name = String::from_str(&env, "User");

    client.register_citizen(&admin, &user_a, &nfc_id, &name);

    // This should panic because "SAME_TAG" is already linked to user_a
    client.register_citizen(&admin, &user_b, &nfc_id, &name);
}

#![cfg(test)]

