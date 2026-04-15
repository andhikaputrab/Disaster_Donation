# Smart Contract Donasi Bantuan Korban Bencana Alam

Sistem smart contract untuk mengumpulkan dan mendistribusikan donasi kepada korban bencana alam secara transparan dan otomatis menggunakan blockchain Stellar Soroban.

## Fitur Utama

### 1. **Campaign Management (Manajemen Kampanye)**
- Membuat campaign donasi untuk setiap bencana
- Menetapkan target donasi dan deadline
- Auto-close campaign ketika target tercapai
- Tracking progress campaign

### 2. **Donation Tracking (Pelacakan Donasi)**
- Mencatat setiap donasi dengan detail donor, jumlah, dan waktu
- Transparansi penuh - semua donasi tercatat di blockchain
- Tidak bisa ada manipulasi data

### 3. **Donor Management (Manajemen Donor)**
- Statistik donasi per donor (total donasi, jumlah donasi)
- Riwayat donasi donor
- Laporan donor

### 4. **Statistics (Statistik)**
- Total campaign yang berjalan
- Total dana terkumpul
- Total donasi yang masuk
- Progress per campaign

## Struktur Data

### DonationCampaign
```rust
pub struct DonationCampaign {
    pub id: u64,                          // ID unik campaign
    pub disaster_name: String,            // Nama bencana
    pub description: String,              // Deskripsi campaign
    pub target_amount: i128,              // Target donasi
    pub collected_amount: i128,           // Dana terkumpul
    pub deadline: u64,                    // Batas waktu
    pub recipient_address: Address,       // Alamat penerima
    pub status: Symbol,                   // active/closed/distributed
    pub created_at: u64,                  // Waktu dibuat
}
```

### Donation
```rust
pub struct Donation {
    pub id: u64,                          // ID donasi
    pub campaign_id: u64,                 // Campaign ID
    pub donor: Address,                   // Alamat donor
    pub amount: i128,                     // Jumlah donasi
    pub timestamp: u64,                   // Waktu donasi
}
```

### DonorStats
```rust
pub struct DonorStats {
    pub total_donated: i128,              // Total donasi
    pub donation_count: u64,              // Jumlah donasi
}
```

## Fungsi-Fungsi Contract

### Initialization
```rust
initialize(admin: Address, token: Address) -> String
```
Inisialisasi contract dengan admin dan token address

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

## Flow Penggunaan

### 1. Inisialisasi
```
1. Deploy contract
2. Call initialize() dengan admin dan token address
```

### 2. Membuat Campaign
```
1. Call create_campaign() dengan detail bencana
2. Dapatkan campaign_id
3. Campaign siap menerima donasi
```

### 3. Melakukan Donasi
```
1. Donor memanggil donate() dengan campaign_id dan amount
2. Dana dicatat di blockchain
3. DonorStats terupdate otomatis
4. Jika target tercapai, campaign auto-close
```

### 4. Tracking & Reporting
```
1. Get campaign progress dengan get_campaign_progress()
2. Get donor stats dengan get_donor_stats()
3. Get total statistics dengan get_total_stats()
```

## Keamanan & Keuntungan

✅ **Immutable Records** - Semua donasi tercatat dan tidak bisa diubah
✅ **Transparent** - Siapa saja bisa melihat aliran dana
✅ **Automated** - Tidak perlu intermediary yang bisa korup
✅ **Fast Settlement** - Donasi langsung dicatat
✅ **Trustless** - Smart contract yang enforce aturan
✅ **Audit Trail** - Setiap transaksi tercatat lengkap

## Testing

Semua fungsi sudah dilengkapi dengan unit tests:
- Test initialize
- Test create campaign
- Test donate
- Test multiple donors
- Test campaign auto-close
- Test percentage calculation
- Test statistics

Jalankan dengan:
```bash
cargo test --lib
```

## Deploy ke Testnet

1. Build contract:
```bash
cargo build --target wasm32-unknown-unknown --release
```

2. Deploy ke Stellar testnet menggunakan Soroban CLI

## Contoh Use Case

### Campaign 1: Bencana Banjir Jakarta
- Target: 1 Miliar Rupiah (dalam token)
- Deadline: 7 hari
- Recipient: Alamat organisasi kemanusiaan

### Campaign 2: Gempa Sumatera
- Target: 500 Juta Rupiah
- Deadline: 14 hari
- Recipient: Alamat komunitas lokal

Donor dapat memberikan donasi ke campaign mana saja, dan semua tercatat transparan di blockchain!
