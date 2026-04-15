#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Address, Env, String, Symbol, Vec, Map};

// ========== Data Structures ==========

#[contracttype]
#[derive(Clone, Debug)]
pub struct DonationCampaign {
    pub id: u64,
    pub disaster_name: String,
    pub description: String,
    pub target_amount: i128,
    pub collected_amount: i128,
    pub deadline: u64,
    pub recipient_address: Address,
    pub status: Symbol, // "active", "closed", "distributed"
    pub created_at: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Donation {
    pub id: u64,
    pub campaign_id: u64,
    pub donor: Address,
    pub amount: i128,
    pub timestamp: u64,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct DonorStats {
    pub total_donated: i128,
    pub donation_count: u64,
}

// ========== Storage Keys ==========

const CAMPAIGN_COUNT: Symbol = symbol_short!("CAM_CNT");
const CAMPAIGNS: Symbol = symbol_short!("CAMPAIGNS");
const DONATIONS: Symbol = symbol_short!("DONATIONS");
const DONOR_STATS: Symbol = symbol_short!("DONOR_STS");
const TOKEN: Symbol = symbol_short!("TOKEN");
const ADMIN: Symbol = symbol_short!("ADMIN");

// ========== Campaign Status Constants ==========

const STATUS_ACTIVE: Symbol = symbol_short!("ACTIVE");
const STATUS_CLOSED: Symbol = symbol_short!("CLOSED");
const STATUS_DIST: Symbol = symbol_short!("DIST");

// ========== Main Contract ==========

#[contract]
pub struct DonationContract;

#[contractimpl]
impl DonationContract {
    
    // ===== Initialize =====
    
    /// Inisialisasi kontrak dengan admin dan token address
    pub fn initialize(env: Env, admin: Address, token: Address) -> String {
        // Cek apakah sudah diinisial
        if let Some(_) = env.storage().instance().get::<_, Address>(&ADMIN) {
            panic_with_error(&env, "Contract sudah diinisialisasi");
        }

        // Set admin
        env.storage().instance().set(&ADMIN, &admin);
        
        // Set token (USDC atau token lain)
        env.storage().instance().set(&TOKEN, &token);
        
        // Set initial campaign count
        env.storage().instance().set(&CAMPAIGN_COUNT, &0u64);

        String::from_str(&env, "Kontrak berhasil diinisialisasi")
    }

    // ===== Campaign Management =====

    /// Membuat campaign donasi baru
    pub fn create_campaign(
        env: Env,
        disaster_name: String,
        description: String,
        target_amount: i128,
        deadline: u64,
        recipient_address: Address,
    ) -> u64 {
        // Get current campaign count
        let campaign_count: u64 = env.storage().instance()
            .get(&CAMPAIGN_COUNT)
            .unwrap_or(0);

        let new_id = campaign_count + 1;

        // Create new campaign
        let campaign = DonationCampaign {
            id: new_id,
            disaster_name,
            description,
            target_amount,
            collected_amount: 0,
            deadline,
            recipient_address,
            status: STATUS_ACTIVE,
            created_at: env.ledger().timestamp(),
        };

        // Get existing campaigns
        let mut campaigns: Vec<DonationCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));

        // Add new campaign
        campaigns.push_back(campaign);

        // Save to storage
        env.storage().instance().set(&CAMPAIGNS, &campaigns);
        env.storage().instance().set(&CAMPAIGN_COUNT, &new_id);

        new_id
    }

    /// Mendapatkan semua campaign
    pub fn get_campaigns(env: Env) -> Vec<DonationCampaign> {
        env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env))
    }

    /// Mendapatkan campaign berdasarkan ID
    pub fn get_campaign(env: Env, campaign_id: u64) -> DonationCampaign {
        let campaigns: Vec<DonationCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));

        for i in 0..campaigns.len() {
            if let Some(campaign) = campaigns.get(i) {
                if campaign.id == campaign_id {
                    return campaign;
                }
            }
        }

        panic_with_error(&env, "Campaign tidak ditemukan");
    }

    /// Menutup campaign
    pub fn close_campaign(env: Env, campaign_id: u64) -> String {
        let mut campaigns: Vec<DonationCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));

        let mut found = false;
        for i in 0..campaigns.len() {
            if let Some(mut campaign) = campaigns.get(i) {
                if campaign.id == campaign_id {
                    campaign.status = STATUS_CLOSED;
                    campaigns.set(i, campaign);
                    found = true;
                    break;
                }
            }
        }

        if !found {
            panic_with_error(&env, "Campaign tidak ditemukan");
        }

        env.storage().instance().set(&CAMPAIGNS, &campaigns);
        String::from_str(&env, "Campaign berhasil ditutup")
    }

    // ===== Donation Management =====

    /// Melakukan donasi ke campaign
    pub fn donate(
        env: Env,
        donor: Address,
        campaign_id: u64,
        amount: i128,
    ) -> String {
        // Validasi amount
        if amount <= 0 {
            panic_with_error(&env, "Jumlah donasi harus lebih dari 0");
        }

        // Get campaign
        let mut campaigns: Vec<DonationCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));

        let mut campaign_found = false;
        let mut campaign_index = 0;

        for i in 0..campaigns.len() {
            if let Some(campaign) = campaigns.get(i) {
                if campaign.id == campaign_id {
                    campaign_found = true;
                    campaign_index = i;

                    // Validasi campaign status
                    if campaign.status != STATUS_ACTIVE {
                        panic_with_error(&env, "Campaign tidak aktif");
                    }

                    break;
                }
            }
        }

        if !campaign_found {
            panic_with_error(&env, "Campaign tidak ditemukan");
        }

        // Update campaign collected amount
        if let Some(mut campaign) = campaigns.get(campaign_index) {
            campaign.collected_amount += amount;
            
            // Auto-close campaign if target reached
            if campaign.collected_amount >= campaign.target_amount {
                campaign.status = STATUS_CLOSED;
            }

            campaigns.set(campaign_index, campaign);
        }

        // Create donation record
        let donations: Vec<Donation> = env.storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Vec::new(&env));

        let donation_id = donations.len() as u64 + 1;

        let donation = Donation {
            id: donation_id,
            campaign_id,
            donor: donor.clone(),
            amount,
            timestamp: env.ledger().timestamp(),
        };

        let mut new_donations = donations;
        new_donations.push_back(donation);

        // Update donor stats
        let mut donor_stats_map: Map<Address, DonorStats> = env.storage()
            .instance()
            .get(&DONOR_STATS)
            .unwrap_or(Map::new(&env));

        let current_stats = donor_stats_map.get(donor.clone()).unwrap_or(DonorStats {
            total_donated: 0,
            donation_count: 0,
        });

        let updated_stats = DonorStats {
            total_donated: current_stats.total_donated + amount,
            donation_count: current_stats.donation_count + 1,
        };

        donor_stats_map.set(donor, updated_stats);

        // Save all updates
        env.storage().instance().set(&CAMPAIGNS, &campaigns);
        env.storage().instance().set(&DONATIONS, &new_donations);
        env.storage().instance().set(&DONOR_STATS, &donor_stats_map);

        String::from_str(&env, "Donasi berhasil dicatat")
    }

    /// Mendapatkan semua donasi
    pub fn get_donations(env: Env) -> Vec<Donation> {
        env.storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Vec::new(&env))
    }

    /// Mendapatkan donasi berdasarkan campaign ID
    pub fn get_campaign_donations(env: Env, campaign_id: u64) -> Vec<Donation> {
        let donations: Vec<Donation> = env.storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Vec::new(&env));

        let mut result: Vec<Donation> = Vec::new(&env);

        for i in 0..donations.len() {
            if let Some(donation) = donations.get(i) {
                if donation.campaign_id == campaign_id {
                    result.push_back(donation);
                }
            }
        }

        result
    }

    // ===== Donor Information =====

    /// Mendapatkan statistik donor
    pub fn get_donor_stats(env: Env, donor: Address) -> DonorStats {
        let donor_stats_map: Map<Address, DonorStats> = env.storage()
            .instance()
            .get(&DONOR_STATS)
            .unwrap_or(Map::new(&env));

        donor_stats_map.get(donor).unwrap_or(DonorStats {
            total_donated: 0,
            donation_count: 0,
        })
    }

    /// Mendapatkan donasi donor tertentu
    pub fn get_donor_donations(env: Env, donor: Address) -> Vec<Donation> {
        let donations: Vec<Donation> = env.storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Vec::new(&env));

        let mut result: Vec<Donation> = Vec::new(&env);

        for i in 0..donations.len() {
            if let Some(donation) = donations.get(i) {
                if donation.donor == donor {
                    result.push_back(donation);
                }
            }
        }

        result
    }

    // ===== Campaign Progress =====

    /// Mendapatkan progress campaign
    pub fn get_campaign_progress(env: Env, campaign_id: u64) -> (i128, i128, Symbol) {
        let campaign = Self::get_campaign(env, campaign_id);
        (campaign.collected_amount, campaign.target_amount, campaign.status)
    }

    /// Mendapatkan persentase campaign
    pub fn get_campaign_percentage(env: Env, campaign_id: u64) -> u64 {
        let campaign = Self::get_campaign(env, campaign_id);
        
        if campaign.target_amount == 0 {
            0
        } else {
            ((campaign.collected_amount * 100) / campaign.target_amount) as u64
        }
    }

    // ===== Statistics =====

    /// Mendapatkan statistik keseluruhan
    pub fn get_total_stats(env: Env) -> (u64, i128, u64) {
        let campaigns: Vec<DonationCampaign> = env.storage()
            .instance()
            .get(&CAMPAIGNS)
            .unwrap_or(Vec::new(&env));

        let donations: Vec<Donation> = env.storage()
            .instance()
            .get(&DONATIONS)
            .unwrap_or(Vec::new(&env));

        let mut total_collected: i128 = 0;

        for i in 0..campaigns.len() {
            if let Some(campaign) = campaigns.get(i) {
                total_collected += campaign.collected_amount;
            }
        }

        (
            campaigns.len() as u64,
            total_collected,
            donations.len() as u64,
        )
    }
}

// ========== Helper Functions ==========

fn panic_with_error(_env: &Env, msg: &str) -> ! {
    panic!("{}", msg);
}