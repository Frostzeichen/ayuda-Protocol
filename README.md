AYUDA Protocol 🛡️
A decentralized identity and transparent fund distribution protocol leveraging NFC and Soroban Smart Contracts.
🏗 System Evolution & Demo
1. Smart Contract Deployment
The foundation of the Ayuda protocol is built on the Stellar Testnet. This image confirms the successful deployment of the Soroban Wasm file to the network, ensuring the distribution logic is immutable and public.
2. Local Protocol Testing
Before going live, the core logic including register_citizen, fund_aid, and claim_aid was rigorously tested in a local environment to ensure state integrity and duplicate prevention—stopping "ghost" claims before they happen.
3. On-Chain Verification (Explorer)
Every transaction is publicly auditable. You can track exactly when aid is claimed and by whom via the Stellar Expert Explorer, removing the "black box" of traditional distribution.
Contract ID: CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S
4. Admin Management Dashboard
The final user interface for GIST administrators. This minimalist dashboard bridges the NFC sensor data with the blockchain, allowing for seamless student registration and real-time tracking of aid claims.
📌 Project Overview
Ayuda is a decentralized identity and resource distribution system designed for institutional environments. It solves the accountability crisis in aid distribution by using physical NFC cards as a "Proof-of-Presence" key.
The Problem
Distribution Leakage: Manual aid distribution is prone to errors, "ghost" recipients, and a lack of real-time auditing.
The Unclaimed Mystery: Currently, there is no transparency regarding what happens to aid if it goes unclaimed or if it was actually collected by the correct beneficiary.
Technical Barriers: Students find blockchain wallets and gas fees too complex for simple resource access.
The Solution
Hardware-Bound Identity: NFC tags act as a secure, physical identifier for students that cannot be easily spoofed or shared remotely.
Immutable Tracking: Every claim is recorded on the Stellar blockchain. If aid goes unclaimed, the ledger shows exactly where the funds remain.
Proof-of-Presence: By requiring a physical NFC tap, the protocol ensures the correct beneficiary is physically present to receive their aid.
🛠 Tech Stack
LayerTechnologySmart ContractRust (Soroban SDK), Stellar NetworkBackendRust (Axum), Stellar CLI, DockerFrontendNext.js 14, Tailwind CSS, Web NFC APIInfrastructureRender (Backend)🚀 Key Features
Minimalist Elegant UI: High-contrast Black & White dashboard focused on speed.
Mobile-as-a-Sensor: No expensive hardware; any NFC-enabled smartphone acts as the remote sensor and verification terminal.
Zero Gas for Users: The institution handles transaction costs for a seamless experience.
🔧 Smart Contract Logic
Rust

#![no_std]use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, String,
};#[contracttype]#[derive(Clone, Debug)]pub struct CitizenData {
    pub name: String,
    pub aid_balance: i128,
}#[contracttype]pub enum DataKey {
    Admin,
    TokenAddr,
    Citizen(Address),
}#[contract]pub struct AyudaContract;#[contractimpl]impl AyudaContract {
    pub fn init(env: Env, admin: Address, token_addr: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("Already init");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage()
            .instance()
            .set(&DataKey::TokenAddr, &token_addr);
    }

    pub fn register_citizen(env: Env, admin: Address, citizen_addr: Address, name: String) {
        admin.require_auth();
        let stored_admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap();
        assert!(admin == stored_admin, "Not admin");

        let data = CitizenData {
            name,
            aid_balance: 0,
        };
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);
    }

    pub fn fund_aid(env: Env, admin: Address, citizen_addr: Address, amount: i128) {
        admin.require_auth();
        let mut data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr.clone()))
            .expect("Citizen not registered");

        data.aid_balance += amount;
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr), &data);
    }

    pub fn claim_aid(env: Env, citizen_addr: Address) {
        let mut data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr.clone()))
            .expect("No record");

        let amount = data.aid_balance;
        assert!(amount > 0, "No aid available");

        let token_addr: Address = env.storage().instance().get(&DataKey::TokenAddr).unwrap();
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&env.current_contract_address(), &citizen_addr, &amount);

        data.aid_balance = 0;
        env.storage()
            .persistent()
            .set(&DataKey::Citizen(citizen_addr.clone()), &data);

        env.events()
            .publish((symbol_short!("paid"), citizen_addr), amount);
    }

    pub fn get_balance(env: Env, citizen_addr: Address) -> i128 {
        let data: CitizenData = env
            .storage()
            .persistent()
            .get(&DataKey::Citizen(citizen_addr))
            .unwrap_or(CitizenData {
                name: String::from_str(&env, "Unknown"),
                aid_balance: 0,
            });
        data.aid_balance
    }
}
🔧 Installation & Setup
Clone the Repository
Bash

git clone https://github.com/rylsherdamz-rgb/stellar.git
Smart Contract Build
Bash

soroban contract build
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ayuda.wasm \
  --source deployer \
  --network testnet
Frontend & Backend
Bash

# In backend folder
cargo run# In frontend folder
npm install
npm run dev this is my contract and i just want you the picture in this new readme i have from this readme just the path 
