# Smart Contract Deployment & Execution Guide

## ✅ Prerequisites

Make sure you have installed:
1. **Rust** - https://rustup.rs/
2. **Soroban CLI** - Install with:
   ```bash
   cargo install --locked soroban-cli
   ```
3. **Node.js & npm** - (optional, for frontend)
4. **Stellar Account** (testnet) - https://laboratory.stellar.org/

Verify installations:
```bash
rustc --version
cargo --version
soroban --version
```

---

## 📦 Step 1: Build Contract

### 1.1 Build for Testing
```bash
cd contracts/donation
cargo build --target wasm32-unknown-unknown --release
```

After building, the WASM file will be located at:
```
target/wasm32-unknown-unknown/release/donation_contract.wasm
```

### 1.2 Expected Output:
```
   Compiling donation_contract v0.1.0
    Finished release [optimized] target(s) in 2.50s
```

---

## 🧪 Step 2: Run Local Tests

### 2.1 Run all tests:
```bash
cargo test --lib
```

### 2.2 Run specific tests:
```bash
cargo test --lib test_donate
cargo test --lib test_create_campaign
```

### 2.3 Run with verbose output:
```bash
cargo test --lib -- --nocapture
```

**Expected Output:**
```
running 7 tests
test test_initialize ... ok
test test_create_campaign ... ok
test test_donate ... ok
test test_multiple_donors ... ok
test test_campaign_auto_close ... ok
test test_get_campaign_percentage ... ok
test test_statistics ... ok

test result: ok. 7 passed; 0 failed; 0 ignored
```

---

## 🚀 Step 3: Setup Stellar Network

### 3.1 Configure Soroban for Testnet
```bash
soroban network add \
  --name testnet \
  --rpc-url https://soroban-testnet.stellar.org:443 \
  --network-passphrase "Test SDF Network ; September 2015"
```

### 3.2 Use testnet as default:
```bash
soroban network use testnet
```

### 3.3 Create or Import Stellar Account (Testnet)

**Option A: Create new account**
```bash
soroban keys generate --name donor1
soroban keys generate --name admin1
```

**Option B: Import existing account**
```bash
soroban keys add --name my_account --secret-key S... (your private key)
```

### 3.4 Fund Account (via Friendbot)
1. Open: https://laboratory.stellar.org/
2. Go to "Friendbot" tab
3. Paste your account's public key
4. Click "Get Started with Test Network"

---

## 🎯 Step 4: Deploy Contract to Testnet

### 4.1 Deploy command:
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/donation_contract.wasm \
  --network testnet \
  --source admin1
```

### 4.2 Output (IMPORTANT - SAVE THIS):
```
Contract deployed successfully.
Contract ID: CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q
```

**💾 SAVE this Contract ID!** You'll need it for all contract interactions.

---

## 🔧 Step 5: Interact with Contract

### 5.1 Initialize Contract
```bash
soroban contract invoke \
  --id CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q \
  --source admin1 \
  --network testnet \
  -- initialize \
  --admin admin1 \
  --token GBUQWP3BOUZX34ULNQG23RQ6F4YUSXHTQSXUSMIQSTBE2TSLKNQF6XK
```

### 5.2 Create Campaign (Flood Disaster)
```bash
soroban contract invoke \
  --id CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q \
  --source admin1 \
  --network testnet \
  -- create_campaign \
  --disaster_name "Jakarta Flood 2024" \
  --description "Relief for flood victims in Jakarta" \
  --target_amount 1000000000 \
  --deadline 1750000000 \
  --recipient_address admin1
```

### 5.3 Get Campaign
```bash
soroban contract invoke \
  --id CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q \
  --source admin1 \
  --network testnet \
  -- get_campaign \
  --campaign_id 1
```

### 5.4 Donate (from donor)
```bash
soroban contract invoke \
  --id CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q \
  --source donor1 \
  --network testnet \
  -- donate \
  --donor donor1 \
  --campaign_id 1 \
  --amount 100000000
```

### 5.5 Get Campaign Progress
```bash
soroban contract invoke \
  --id CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q \
  --source donor1 \
  --network testnet \
  -- get_campaign_progress \
  --campaign_id 1
```

### 5.6 Get Donor Stats
```bash
soroban contract invoke \
  --id CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q \
  --source donor1 \
  --network testnet \
  -- get_donor_stats \
  --donor donor1
```

### 5.7 Get Total Statistics
```bash
soroban contract invoke \
  --id CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q \
  --source admin1 \
  --network testnet \
  -- get_total_stats
```

---

## 🔗 Using Contract ID via Environment Variable

It's easier to save the Contract ID:
```bash
export CONTRACT_ID="CAQQ33EWLX5Z7JCQP6QLJQDL7HHTJNV5D3H6CEPV7KZ5Q5H3K7Z5Q"

# Then use it directly:
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin1 \
  --network testnet \
  -- get_total_stats
```

---

## 📋 Quick Command Cheat Sheet

```bash
# Build contract
cargo build --target wasm32-unknown-unknown --release

# Run tests
cargo test --lib

# Setup testnet
soroban network add --name testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015"

# Generate keys
soroban keys generate --name admin1

# Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/donation_contract.wasm \
  --network testnet \
  --source admin1

# Get public key
soroban keys address admin1
```

---

## ✔️ Verify Deployment

### 1. Check contract status on Stellar Explorer
- Testnet: https://stellar.expert/explorer/test/
- Search with your Contract ID

### 2. View transactions
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin1 \
  --network testnet \
  -- get_campaigns
```

---

## 🐛 Troubleshooting

### Error: "wasm32-unknown-unknown target not found"
```bash
rustup target add wasm32-unknown-unknown
```

### Error: "soroban-cli not found"
```bash
cargo install --locked soroban-cli
```

### Error: "Account not funded"
1. Open https://laboratory.stellar.org/
2. Go to "Friendbot" tab
3. Paste your public key
4. Click "Get Started"

### Error: "Invalid network"
```bash
soroban network use testnet
soroban network list
```

---

## 📝 Complete Workflow Example

```bash
# 1. Build
cargo build --target wasm32-unknown-unknown --release

# 2. Setup testnet
soroban network add --name testnet --rpc-url https://soroban-testnet.stellar.org:443 --network-passphrase "Test SDF Network ; September 2015"

# 3. Generate wallets
soroban keys generate --name admin1
soroban keys generate --name donor1

# 4. Fund from Friendbot (at https://laboratory.stellar.org/)

# 5. Deploy
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/donation_contract.wasm \
  --network testnet \
  --source admin1

# 6. Set CONTRACT_ID
export CONTRACT_ID="(paste contract id from step 5)"

# 7. Initialize
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin1 \
  --network testnet \
  -- initialize \
  --admin $(soroban keys address admin1) \
  --token GBUQWP3BOUZX34ULNQG23RQ6F4YUSXHTQSXUSMIQSTBE2TSLKNQF6XK

# 8. Create campaign
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin1 \
  --network testnet \
  -- create_campaign \
  --disaster_name "Jakarta Flood" \
  --description "Relief for flood victims" \
  --target_amount 1000000000 \
  --deadline 1750000000 \
  --recipient_address $(soroban keys address admin1)

# 9. Check campaigns
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin1 \
  --network testnet \
  -- get_campaigns
```

---

## 🎓 Official Documentation

- Soroban Docs: https://soroban.stellar.org
- Soroban CLI: https://github.com/stellar/rs-soroban-sdk
- Stellar Network: https://stellar.org
