#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Map, String, Symbol};

const REWARDS: Symbol = symbol_short!("REWARD");

#[contract]
pub struct RewardContract;

#[contractimpl]
impl RewardContract {
    pub fn grant_reward(env: Env, wallet: String, reward_code: String) {
        let mut rewards: Map<String, String> =
            env.storage().instance().get(&REWARDS).unwrap_or(Map::new(&env));

        rewards.set(wallet, reward_code);
        env.storage().instance().set(&REWARDS, &rewards);
    }
}
