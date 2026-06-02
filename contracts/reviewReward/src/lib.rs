#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, contracterror,
    Address, Env, String, symbol_short, panic_with_error,
};

const DAY_IN_LEDGERS: u32 = 17280;
const INSTANCE_TTL: u32 = 7 * DAY_IN_LEDGERS;
const INSTANCE_THRESHOLD: u32 = 6 * DAY_IN_LEDGERS;
const PERSISTENT_TTL: u32 = 30 * DAY_IN_LEDGERS;
const PERSISTENT_THRESHOLD: u32 = 29 * DAY_IN_LEDGERS;

#[contracttype]
pub enum DataKey {
    Admin,
    Count,
    Review(u64),
    Balance(Address),
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Review {
    pub id: u64,
    pub submission_id: u64,
    pub reviewer: Address,
    pub score: u32,
    pub comment: String,
    pub timestamp: u64,
    pub rewarded: bool,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ReviewError {
    NotAuthorized = 1,
    NotFound = 2,
    InvalidInput = 3,
}

#[contract]
pub struct ReviewReward;

#[contractimpl]
impl ReviewReward {
    pub fn initialize(env: Env, admin: Address) {
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Count, &0_u64);
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
    }

    /// Submit a peer review. The caller must be the reviewer (require_auth).
    pub fn submit_review(env: Env, submission_id: u64, reviewer: Address, score: u32, comment: String) -> u64 {
        reviewer.require_auth();
        if score > 100 { panic_with_error!(&env, ReviewError::InvalidInput); }

        let id: u64 = env.storage().instance().get(&DataKey::Count).unwrap_or(0) + 1;
        let rec = Review {
            id,
            submission_id,
            reviewer: reviewer.clone(),
            score,
            comment: comment.clone(),
            timestamp: env.ledger().timestamp(),
            rewarded: false,
        };

        env.storage().persistent().set(&DataKey::Review(id), &rec);
        env.storage().persistent().extend_ttl(&DataKey::Review(id), PERSISTENT_THRESHOLD, PERSISTENT_TTL);
        env.storage().instance().set(&DataKey::Count, &id);
        env.storage().instance().extend_ttl(INSTANCE_THRESHOLD, INSTANCE_TTL);
        env.events().publish((symbol_short!("review"), reviewer, submission_id), id);
        id
    }

    /// Read a review by id
    pub fn get_review(env: Env, id: u64) -> Review {
        env.storage().persistent().get(&DataKey::Review(id)).unwrap_or_else(|| panic_with_error!(&env, ReviewError::NotFound))
    }

    /// Admin mints reward points to a reviewer
    pub fn reward_reviewer(env: Env, reviewer: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap_or_else(|| panic_with_error!(&env, ReviewError::NotAuthorized));
        admin.require_auth();
        if amount <= 0 { panic_with_error!(&env, ReviewError::InvalidInput); }

        let key = DataKey::Balance(reviewer.clone());
        let bal: i128 = env.storage().persistent().get(&key).unwrap_or(0_i128);
        env.storage().persistent().set(&key, &(bal + amount));
        env.storage().persistent().extend_ttl(&key, PERSISTENT_THRESHOLD, PERSISTENT_TTL);
        env.events().publish((symbol_short!("reward"), reviewer.clone()), amount);
    }

    /// Check balance for an address
    pub fn balance_of(env: Env, who: Address) -> i128 {
        env.storage().persistent().get(&DataKey::Balance(who)).unwrap_or(0_i128)
    }

    /// Mark a review as rewarded (admin only)
    pub fn mark_review_rewarded(env: Env, id: u64) {
        let admin: Address = env.storage().instance().get(&DataKey::Admin).unwrap_or_else(|| panic_with_error!(&env, ReviewError::NotAuthorized));
        admin.require_auth();
        let mut r: Review = env.storage().persistent().get(&DataKey::Review(id)).unwrap_or_else(|| panic_with_error!(&env, ReviewError::NotFound));
        r.rewarded = true;
        env.storage().persistent().set(&DataKey::Review(id), &r);
        env.storage().persistent().extend_ttl(&DataKey::Review(id), PERSISTENT_THRESHOLD, PERSISTENT_TTL);
        env.events().publish((symbol_short!("mark_rwd"),), id);
    }
}

// ============================================================
// TESTS
// ============================================================

#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{testutils::Address as _, Env, String};

    #[test]
    fn test_full_flow() {
        let env = Env::default();
        env.mock_all_auths();

        let admin = Address::generate(&env);
        let reviewer = Address::generate(&env);

        let contract_id = env.register(ReviewReward, ());
        let client = ReviewRewardClient::new(&env, &contract_id);

        client.initialize(&admin);

        let comment = String::from_str(&env, "Good job");
        let id = client.submit_review(&42, &reviewer, &85, &comment);
        let r = client.get_review(&id);
        assert_eq!(r.score, 85);

        client.reward_reviewer(&reviewer, &10_i128);
        assert_eq!(client.balance_of(&reviewer), 10_i128);

        client.mark_review_rewarded(&id);
        let r2 = client.get_review(&id);
        assert!(r2.rewarded);
    }
}
