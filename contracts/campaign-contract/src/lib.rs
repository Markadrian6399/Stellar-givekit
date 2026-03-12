#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, String, Symbol};

const CAMPAIGNS: Symbol = symbol_short!("CAMPS");

#[contract]
pub struct CampaignContract;

#[contractimpl]
impl CampaignContract {
    pub fn create_campaign(env: Env, campaign_id: String, owner: String, target_amount: i128) {
        let mut campaigns: Map<String, (String, i128, bool)> =
            env.storage().instance().get(&CAMPAIGNS).unwrap_or(Map::new(&env));

        campaigns.set(campaign_id, (owner, target_amount, true));
        env.storage().instance().set(&CAMPAIGNS, &campaigns);
    }
}
