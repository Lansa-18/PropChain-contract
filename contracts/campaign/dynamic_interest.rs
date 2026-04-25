#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Env};

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    TotalSupply,
    TotalBorrowed,
    InterestParams,
}

#[derive(Clone)]
#[contracttype]
pub struct InterestParams {
    pub base_rate: i128, // e.g. 2% = 200 (basis points)
    pub slope: i128,     // multiplier
    pub max_rate: i128,  // cap
}

#[contract]
pub struct LendingContract;

#[contractimpl]
impl LendingContract {

    // 🔹 Initialize parameters
    pub fn init(env: Env, base_rate: i128, slope: i128, max_rate: i128) {
        let params = InterestParams {
            base_rate,
            slope,
            max_rate,
        };

        env.storage().instance().set(&DataKey::InterestParams, &params);
        env.storage().instance().set(&DataKey::TotalSupply, &0i128);
        env.storage().instance().set(&DataKey::TotalBorrowed, &0i128);
    }

    // 🔹 Update supply (e.g. deposits)
    pub fn update_supply(env: Env, amount: i128) {
        let mut supply: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        supply += amount;

        env.storage().instance().set(&DataKey::TotalSupply, &supply);
    }

    // 🔹 Update borrowed (e.g. loans)
    pub fn update_borrowed(env: Env, amount: i128) {
        let mut borrowed: i128 = env.storage().instance()
            .get(&DataKey::TotalBorrowed)
            .unwrap_or(0);

        borrowed += amount;

        env.storage().instance().set(&DataKey::TotalBorrowed, &borrowed);
    }

    // 🔹 Compute utilization
    pub fn get_utilization(env: Env) -> i128 {
        let supply: i128 = env.storage().instance()
            .get(&DataKey::TotalSupply)
            .unwrap_or(0);

        let borrowed: i128 = env.storage().instance()
            .get(&DataKey::TotalBorrowed)
            .unwrap_or(0);

        if supply == 0 {
            return 0;
        }

        // scaled by 10,000 (basis points)
        (borrowed * 10_000) / supply
    }

    // 🔹 Compute dynamic interest rate
    pub fn get_interest_rate(env: Env) -> i128 {
        let params: InterestParams = env.storage().instance()
            .get(&DataKey::InterestParams)
            .expect("Params not set");

        let utilization = Self::get_utilization(env.clone());

        let mut rate =
            params.base_rate + ((utilization * params.slope) / 10_000);

        if rate > params.max_rate {
            rate = params.max_rate;
        }

        rate
    }
}
