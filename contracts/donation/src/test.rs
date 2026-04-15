#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as TestAddress, Address, Env, String};

#[test]
fn test_initialize() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);

    let contract = DonationContractClient::new(&env, &env.register_contract(None, DonationContract));

    let result = contract.initialize(&admin, &token);
    assert_eq!(result, String::from_str(&env, "Kontrak berhasil diinisialisasi"));
}

#[test]
fn test_create_campaign() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let recipient = Address::generate(&env);

    let contract = DonationContractClient::new(&env, &env.register_contract(None, DonationContract));

    contract.initialize(&admin, &token);

    let campaign_id = contract.create_campaign(
        &String::from_str(&env, "Bencana Banjir Jakarta"),
        &String::from_str(&env, "Bantuan untuk korban banjir di Jakarta"),
        &1_000_000_000, // 1 miliar
        &(env.ledger().timestamp() + 7 * 24 * 3600), // 7 hari
        &recipient,
    );

    assert_eq!(campaign_id, 1);

    let campaign = contract.get_campaign(&1);
    assert_eq!(campaign.id, 1);
    assert_eq!(campaign.target_amount, 1_000_000_000);
    assert_eq!(campaign.collected_amount, 0);
}

#[test]
fn test_donate() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let recipient = Address::generate(&env);
    let donor = Address::generate(&env);

    let contract = DonationContractClient::new(&env, &env.register_contract(None, DonationContract));

    contract.initialize(&admin, &token);

    let campaign_id = contract.create_campaign(
        &String::from_str(&env, "Bencana Gempa Sumatera"),
        &String::from_str(&env, "Bantuan untuk korban gempa di Sumatera"),
        &500_000_000,
        &(env.ledger().timestamp() + 7 * 24 * 3600),
        &recipient,
    );

    // Donor 1 donates
    let result = contract.donate(&donor, &campaign_id, &100_000_000);
    assert_eq!(result, String::from_str(&env, "Donasi berhasil dicatat"));

    // Check campaign progress
    let (collected, target, _) = contract.get_campaign_progress(&campaign_id);
    assert_eq!(collected, 100_000_000);
    assert_eq!(target, 500_000_000);

    // Check donor stats
    let stats = contract.get_donor_stats(&donor);
    assert_eq!(stats.total_donated, 100_000_000);
    assert_eq!(stats.donation_count, 1);
}

#[test]
fn test_multiple_donors() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let recipient = Address::generate(&env);
    let donor1 = Address::generate(&env);
    let donor2 = Address::generate(&env);
    let donor3 = Address::generate(&env);

    let contract = DonationContractClient::new(&env, &env.register_contract(None, DonationContract));

    contract.initialize(&admin, &token);

    let campaign_id = contract.create_campaign(
        &String::from_str(&env, "Bencana Tsunami Aceh"),
        &String::from_str(&env, "Bantuan untuk korban tsunami"),
        &300_000_000,
        &(env.ledger().timestamp() + 7 * 24 * 3600),
        &recipient,
    );

    // Multiple donors
    contract.donate(&donor1, &campaign_id, &100_000_000);
    contract.donate(&donor2, &campaign_id, &80_000_000);
    contract.donate(&donor3, &campaign_id, &50_000_000);

    let donations = contract.get_donations();
    assert_eq!(donations.len(), 3);

    let (collected, target, status) = contract.get_campaign_progress(&campaign_id);
    assert_eq!(collected, 230_000_000);
    assert_eq!(target, 300_000_000);
    assert_eq!(status, symbol_short!("ACTIVE")); // Still active
}

#[test]
fn test_campaign_auto_close() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let recipient = Address::generate(&env);
    let donor = Address::generate(&env);

    let contract = DonationContractClient::new(&env, &env.register_contract(None, DonationContract));

    contract.initialize(&admin, &token);

    let campaign_id = contract.create_campaign(
        &String::from_str(&env, "Bencana Gunung Merapi"),
        &String::from_str(&env, "Bantuan untuk korban letusan Merapi"),
        &200_000_000,
        &(env.ledger().timestamp() + 7 * 24 * 3600),
        &recipient,
    );

    // Donor reaches target
    contract.donate(&donor, &campaign_id, &200_000_000);

    let (_, _, status) = contract.get_campaign_progress(&campaign_id);
    assert_eq!(status, symbol_short!("CLOSED")); // Auto closed when target reached
}

#[test]
fn test_get_campaign_percentage() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let recipient = Address::generate(&env);
    let donor = Address::generate(&env);

    let contract = DonationContractClient::new(&env, &env.register_contract(None, DonationContract));

    contract.initialize(&admin, &token);

    let campaign_id = contract.create_campaign(
        &String::from_str(&env, "Bencana Tanah Longsor"),
        &String::from_str(&env, "Bantuan untuk korban tanah longsor"),
        &100_000_000,
        &(env.ledger().timestamp() + 7 * 24 * 3600),
        &recipient,
    );

    contract.donate(&donor, &campaign_id, &50_000_000);

    let percentage = contract.get_campaign_percentage(&campaign_id);
    assert_eq!(percentage, 50); // 50%
}

#[test]
fn test_statistics() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let token = Address::generate(&env);
    let recipient1 = Address::generate(&env);
    let recipient2 = Address::generate(&env);
    let donor1 = Address::generate(&env);
    let donor2 = Address::generate(&env);

    let contract = DonationContractClient::new(&env, &env.register_contract(None, DonationContract));

    contract.initialize(&admin, &token);

    // Create 2 campaigns
    let campaign1 = contract.create_campaign(
        &String::from_str(&env, "Campaign 1"),
        &String::from_str(&env, "Description 1"),
        &500_000_000,
        &(env.ledger().timestamp() + 7 * 24 * 3600),
        &recipient1,
    );

    let campaign2 = contract.create_campaign(
        &String::from_str(&env, "Campaign 2"),
        &String::from_str(&env, "Description 2"),
        &300_000_000,
        &(env.ledger().timestamp() + 7 * 24 * 3600),
        &recipient2,
    );

    // Multiple donations
    contract.donate(&donor1, &campaign1, &300_000_000);
    contract.donate(&donor2, &campaign1, &200_000_000);
    contract.donate(&donor1, &campaign2, &150_000_000);

    let (campaign_count, total_collected, donation_count) = contract.get_total_stats();
    assert_eq!(campaign_count, 2);
    assert_eq!(total_collected, 650_000_000);
    assert_eq!(donation_count, 3);
}
