#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::testutils::{Address as _, Events};
    use soroban_sdk::{vec, IntoVal};

    #[test]
    fn test_full_ayuda_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let citizen = Address::generate(&env);
        let token_admin = Address::generate(&env);

        let token_addr = env.register_stellar_asset_contract(token_admin.clone());
        let token_client = token::Client::new(&env, &token_addr);

        let contract_id = env.register_contract(None, AyudaContract);
        let client = AyudaContractClient::new(&env, &contract_id);

        client.init(&admin, &token_addr);

        let citizen_name = String::from_str(&env, "Richie");
        client.register_citizen(&admin, &citizen, &citizen_name);

        let aid_amount = 1000;
        token_client.mint(&contract_id, &aid_amount);

        client.fund_aid(&admin, &citizen, &aid_amount);
        assert_eq!(client.get_balance(&citizen), 1000);

        client.claim_aid(&citizen);

        assert_eq!(token_client.balance(&citizen), 1000);
        assert_eq!(client.get_balance(&citizen), 0);

        let events = env.events().all();
        let last_event = events.last().unwrap();
        assert_eq!(
            last_event,
            (
                contract_id,
                (symbol_short!("paid"), citizen.clone()).into_val(&env),
                aid_amount.into_val(&env)
            )
        );
    }
}

