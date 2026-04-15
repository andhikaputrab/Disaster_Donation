# Disaster Relief Donation Smart Contract

A transparent blockchain-based smart contract system for collecting and distributing donations to disaster victims using Stellar Soroban. Built with trust and transparency at its core.

### ID Smartcontract
CBK36NYDT6RTLWFVR7P5O33EUGSLS2D2VULER5HE7RAAFKWUNRQPGZDX

### Testnet Screenshot
<img width="1911" height="730" alt="image" src="https://github.com/user-attachments/assets/9c666d10-6b86-454d-ada7-da41a0d4dd80" />

## Key Features

### 1. **Campaign Management**
- Create donation campaigns for each disaster event
- Set target donation amounts and deadlines
- Auto-close campaigns when targets are reached
- Real-time campaign progress tracking

### 2. **Donation Tracking**
- Record every donation with donor details, amount, and timestamp
- Complete transparency - all donations recorded on blockchain
- Tamper-proof records - immutable once recorded

### 3. **Donor Management**
- Per-donor donation statistics (total donated, donation count)
- Complete donor history
- Donor reports and profiles

### 4. **Statistics & Analytics**
- Total active campaigns
- Total funds collected
- Total number of donations
- Per-campaign progress metrics

## Data Structures

### DonationCampaign
```rust
pub struct DonationCampaign {
    pub id: u64,                          // Unique campaign identifier
    pub disaster_name: String,            // Name of the disaster
    pub description: String,              // Campaign description
    pub target_amount: i128,              // Donation target in stroops
    pub collected_amount: i128,           // Amount collected so far
    pub deadline: u64,                    // Campaign deadline (timestamp)
    pub recipient_address: Address,       // Address receiving funds
    pub status: Symbol,                   // active/closed/distributed
    pub created_at: u64,                  // Campaign creation timestamp
}
```

### Donation
```rust
pub struct Donation {
    pub id: u64,                          // Unique donation identifier
    pub campaign_id: u64,                 // Associated campaign ID
    pub donor: Address,                   // Donor blockchain address
    pub amount: i128,                     // Donation amount in stroops
    pub timestamp: u64,                   // Donation timestamp
}
```

### DonorStats
```rust
pub struct DonorStats {
    pub total_donated: i128,              // Total amount donated
    pub donation_count: u64,              // Total number of donations
}
```

## Contract Functions

### Initialization
```rust
initialize(admin: Address, token: Address) -> String
```
Initialize the contract with admin and token addresses

### Campaign Functions
```rust
create_campaign(
    disaster_name: String,
    description: String,
    target_amount: i128,
    deadline: u64,
    recipient_address: Address,
) -> u64

get_campaigns() -> Vec<DonationCampaign>
get_campaign(campaign_id: u64) -> DonationCampaign
close_campaign(campaign_id: u64) -> String
```

### Donation Functions
```rust
donate(
    donor: Address,
    campaign_id: u64,
    amount: i128,
) -> String

get_donations() -> Vec<Donation>
get_campaign_donations(campaign_id: u64) -> Vec<Donation>
```

### Donor Functions
```rust
get_donor_stats(donor: Address) -> DonorStats
get_donor_donations(donor: Address) -> Vec<Donation>
```

### Progress & Statistics
```rust
get_campaign_progress(campaign_id: u64) -> (i128, i128, Symbol)
get_campaign_percentage(campaign_id: u64) -> u64
get_total_stats() -> (u64, i128, u64)
```

## Usage Flow

### 1. Initialization
```
1. Deploy contract to Stellar testnet
2. Call initialize() with admin and token address
```

### 2. Create Campaign
```
1. Call create_campaign() with disaster details
2. Receive campaign_id
3. Campaign is now ready to receive donations
```

### 3. Make Donation
```
1. Donor calls donate() with campaign_id and amount
2. Funds are recorded on blockchain
3. DonorStats automatically updated
4. Campaign auto-closes when target is reached
```

### 4. Tracking & Reporting
```
1. Get campaign progress with get_campaign_progress()
2. Get donor stats with get_donor_stats()
3. Get total statistics with get_total_stats()
```

## Security & Benefits

✅ **Immutable Records** - All donations are recorded permanently and cannot be altered
✅ **Transparent** - Anyone can view the complete flow of funds
✅ **Automated** - No intermediaries that could be corrupted
✅ **Fast Settlement** - Donations are immediately recorded
✅ **Trustless** - Smart contracts enforce all rules programmatically
✅ **Complete Audit Trail** - Every transaction is fully traceable

## Testing

All contract functions include comprehensive unit tests:
- Initialize function test
- Create campaign functionality
- Donation transaction processing
- Multiple donors scenario
- Campaign auto-close on target reached
- Campaign percentage calculation
- Statistics aggregation

Run tests with:
```bash
cargo test --lib
```

## Deploy to Testnet

1. Build the contract:
```bash
cargo build --target wasm32-unknown-unknown --release
```

2. Deploy to Stellar testnet using Soroban CLI (see DEPLOYMENT.md for details)

## Use Case Examples

### Campaign 1: Jakarta Flood Disaster
- Target: 1 Billion tokens
- Duration: 7 days
- Recipient: Humanitarian organization address

### Campaign 2: Sumatera Earthquake Relief
- Target: 500 Million tokens
- Duration: 14 days
- Recipient: Local community address

Donors can contribute to any campaign, and all donations are recorded transparently on the blockchain for complete accountability!
