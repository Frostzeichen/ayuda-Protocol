# AYUDA Protocol 🛡️

Decentralized identity and transparent fund distribution system for institutions, built on Stellar.

---

## 🏗 System Evolution & Demo

### 1. Smart Contract Deployment

The foundation of the AYUDA Protocol is deployed on the Stellar Testnet. This confirms that the Soroban smart contract is live, immutable, and publicly verifiable.

![Smart Contract Deployment](docs/deployed.png)

---

### 2. Local Protocol Testing

Before deployment, core functions such as `register_citizen`, `fund_aid`, and `claim_aid` were tested locally to ensure correct state handling, prevent duplicate entries, and validate secure aid logic.

![Local Protocol Testing](docs/test.png)

---

### 3. On-Chain Verification (Explorer)

Every transaction is fully transparent and can be verified on the Stellar blockchain. This removes the “black box” problem in traditional aid systems.

Contract ID:

```txt
CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S
```

Explorer:

```txt
https://stellar.expert/explorer/testnet/contract/CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S
```

![On-Chain Verification](docs/explorer.png)

---

### 4. Admin Management Dashboard

The admin dashboard connects NFC-based identity verification with blockchain records, allowing real-time monitoring of student registration and aid distribution.

![Admin Dashboard](docs/dashboard.png)

---

## 📌 Project Overview

AYUDA Protocol is a decentralized identity and resource distribution system designed for institutional environments.

It solves transparency and accountability issues in aid distribution using NFC-based Proof-of-Presence and blockchain verification.

---

## ❗ The Problem

* Manual aid distribution is prone to errors, ghost recipients, and lack of auditing.
* There is no transparency on unclaimed or unverified aid.
* Students struggle with wallets, blockchain complexity, and gas fees.
* Institutions cannot clearly track where funds go after allocation.

---

## ✅ The Solution

* NFC cards act as secure physical identity verification.
* Every transaction is recorded on the Stellar blockchain.
* Proof-of-Presence ensures only verified students can claim aid.
* Institutions handle gas fees for a seamless experience.
* Every aid record is fully traceable on-chain.
* If aid remains unclaimed, funds are returned to the institution’s aid pool for future redistribution.

---

## 🛠 Tech Stack

| Layer          | Technology                            |
| -------------- | ------------------------------------- |
| Smart Contract | Rust (Soroban SDK), Stellar Network   |
| Backend        | Rust (Axum), Stellar CLI, Docker      |
| Frontend       | Next.js 14, Tailwind CSS, Web NFC API |
| Infrastructure | Render                                |

---

## 🚀 Key Features

* NFC-based identity verification
* Fully transparent blockchain audit trail
* Real-time admin dashboard
* Instant settlement via Soroban
* Zero gas fees for students
* Institutional aid pool recovery system

---

## 📂 Project Structure

```txt
ayuda-protocol/
├── contracts/
│   ├── src/
│   │   ├── lib.rs
│   │   └── test.rs
│   ├── Cargo.toml
├── frontend/
│   ├── src/
│   ├── components/
│   ├── lib/
│   ├── styles/
│   └── package.json
└── README.md
```

---

## 🔧 Smart Contract

```txt
CACB6NY66CGT2YGKBYJGOLSLPHZNYMZ5WBEGSNGYNGFFE7P6QSZYWV6S
```

---

## 🔧 Installation & Setup

```bash
soroban contract build

soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/ayuda.wasm \
  --source deployer \
  --network testnet
```

```bash
cd frontend
npm install
npm run dev
```

---

## 📷 Image Paths

```txt
docs/deployed.png
docs/test.png
docs/explorer.png
docs/dashboard.png
```

---

## 🌍 Why Stellar

Stellar enables fast, low-cost, and transparent transactions ideal for institutional aid systems. Soroban smart contracts ensure automation while maintaining full auditability.

---

## 🔮 Future Improvements

* QR fallback for NFC
* SMS notifications
* Multi-campus support
* Advanced analytics dashboard
* Offline NFC syncing

---

## 📜 License

MIT License

