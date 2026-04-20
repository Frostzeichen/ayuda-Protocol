#![no_std]
use soroban_sdk::{
    contract, contracterror, contractimpl, contracttype, token, Address, Env, String,
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
    IdAlreadyLinked = 7,
    InvalidIdMapping = 8,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct CitizenData {
    pub name: String,
    pub aid_balance: i128,
    pub linked_nfc: String,
}

#[contracttype]
pub enum DataKey {
    Admin,
    TokenAddr,
    Citizen(Address),
    NfcMapping(String),
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
        Ok(())
    }

    pub fn register_citizen(
        env: Env,
        admin: Address,
        citizen_addr: Address,
        nfc_id: String,
        name: String,
    ) -> Result<(), AyudaError> {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(AyudaError::NotAdmin)?;

        if admin != stored_admin {
            return Err(AyudaError::NotAdmin);
        }

        if env
            .storage()
            .persistent()
            .has(&DataKey::NfcMapping(nfc_id.clone()))
        {
            return Err(AyudaError::IdAlreadyLinked);
        }

        let data = CitizenData {
            name: name.clone(),
            aid_balance: 0,
            linked_nfc: nfc_id.clone(),
        };

        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);
        env.storage()
            .persistent()
            .set(&DataKey::NfcMapping(nfc_id), &citizen_addr);

        Ok(())
    }

    pub fn fund_aid(
        env: Env,
        admin: Address,
        citizen_addr: Address,
        amount: i128,
    ) -> Result<(), AyudaError> {
        admin.require_auth();

        let stored_admin: Address = env
            .storage()
            .instance()
            .get(&DataKey::Admin)
            .ok_or(AyudaError::NotAdmin)?;

        if admin != stored_admin {
            return Err(AyudaError::NotAdmin);
        }

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
            .set(&DataKey::Citizen(citizen_addr), &data);

        Ok(())
    }

    pub fn claim_aid(env: Env, citizen_addr: Address, nfc_id: String) -> Result<(), AyudaError> {
        citizen_addr.require_auth();

        let mapped_addr: Address = env
            .storage()
            .persistent()
            .get(&DataKey::NfcMapping(nfc_id.clone()))
            .ok_or(AyudaError::InvalidIdMapping)?;

        if mapped_addr != citizen_addr {
            return Err(AyudaError::InvalidIdMapping);
        }

        let mut data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr.clone()))
            .ok_or(AyudaError::CitizenNotRegistered)?;

        let amount = data.aid_balance;
        if amount <= 0 {
            return Err(AyudaError::NoAidAvailable);
        }

        let token_addr: Address = env
            .storage()
            .instance()
            .get(&DataKey::TokenAddr)
            .ok_or(AyudaError::NoAidAvailable)?;

        let client = token::Client::new(&env, &token_addr);

        let contract_balance = client.balance(&env.current_contract_address());
        if contract_balance < amount {
            return Err(AyudaError::InsufficientContractBalance);
        }

        client.transfer(&env.current_contract_address(), &citizen_addr, &amount);

        data.aid_balance = 0;
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr), &data);

        Ok(())
    }

    pub fn get_balance(env: Env, citizen_addr: Address) -> i128 {
        env.storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr))
            .map(|d: CitizenData| d.aid_balance)
            .unwrap_or(0)
    }
}

