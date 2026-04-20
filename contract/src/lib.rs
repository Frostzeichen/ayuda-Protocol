#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, log, symbol_short, token, Address, Env,
    String,
};

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum AyudaError {
    AlreadyInitialized = 1,
    NotAdmin = 2,
    CitizenNotRegistered = 3,
    NoAidAvailable = 4,
    InvalidAmount = 5,
    InsufficientContractBalance = 6,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct CitizenData {
    pub name: String,
    pub aid_balance: i128,
}

#[contracttype]
pub enum DataKey {
    Admin,
    TokenAddr,
    Citizen(Address),
}

#[contract]
pub struct AyudaContract;

#[contractimpl]
impl AyudaContract {
    pub fn init(env: Env, admin: Address, token_addr: Address) -> Result<(), AyudaError> {
        if env.storage().instance().has(&DataKey::Admin) {
            return Err(AyudaError::AlreadyInitialized);
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::TokenAddr, &token_addr);

        log!(&env, "PROTOCOL_INIT: Admin set to {}", admin);
        Ok(())
    }

    pub fn register_citizen(
        env: Env,
        admin: Address,
        citizen_addr: Address,
        name: String,
    ) -> Result<(), AyudaError> {
        admin.require_auth();

        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        if admin != stored_admin {
            return Err(AyudaError::NotAdmin);
        }

        let data = CitizenData {
            name: name.clone(),
            aid_balance: 0,
        };

        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);

        log!(&env, "CITIZEN_REG: {} registered to {}", name, citizen_addr);
        env.events()
            .publish((symbol_short!("reg"), citizen_addr), name);
        Ok(())
    }

    pub fn fund_aid(
        env: Env,
        admin: Address,
        citizen_addr: Address,
        amount: i128,
    ) -> Result<(), AyudaError> {
        admin.require_auth();

        if amount <= 0 {
            return Err(AyudaError::InvalidAmount);
        }

        let mut data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr.clone()))
            .ok_or(AyudaError::CitizenNotRegistered)?;

        data.aid_balance += amount;

        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);

        log!(&env, "AID_FUNDED: {} added to {}", amount, citizen_addr);
        env.events()
            .publish((symbol_short!("funded"), citizen_addr), amount);
        Ok(())
    }

    pub fn claim_aid(env: Env, citizen_addr: Address) -> Result<(), AyudaError> {
        citizen_addr.require_auth();

        let mut data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr.clone()))
            .ok_or(AyudaError::CitizenNotRegistered)?;

        let amount = data.aid_balance;
        if amount <= 0 {
            return Err(AyudaError::NoAidAvailable);
        }

        let token_addr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let client = token::Client::new(&env, &token_addr);

        let contract_balance = client.balance(&env.current_contract_address());
        if contract_balance < amount {
            log!(
                &env,
                "CRITICAL: Contract balance {} is less than claim {}",
                contract_balance,
                amount
            );
            return Err(AyudaError::InsufficientContractBalance);
        }

        client.transfer(&env.current_contract_address(), &citizen_addr, &amount);

        data.aid_balance = 0;
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);

        log!(&env, "AID_CLAIMED: {} sent to {}", amount, citizen_addr);
        env.events()
            .publish((symbol_short!("paid"), citizen_addr), amount);
        Ok(())
    }

    pub fn get_balance(env: Env, citizen_addr: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr))
            .map(|data: CitizenData| data.aid_balance)
            .unwrap_or(0)
    }
}

#[cfg(test)]
mod test;
