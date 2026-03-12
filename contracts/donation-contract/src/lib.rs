#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, String, Symbol};

const DONATIONS: Symbol = symbol_short!("DONATE");

#[contract]
pub struct DonationContract;

#[contractimpl]
impl DonationContract {
    pub fn record_donation(env: Env, donation_id: String, campaign_id: String, donor: String, amount: i128) {
        let mut donations: Map<String, (String, String, i128)> =
            env.storage().instance().get(&DONATIONS).unwrap_or(Map::new(&env));

        donations.set(donation_id, (campaign_id, donor, amount));
        env.storage().instance().set(&DONATIONS, &donations);
    }
}
