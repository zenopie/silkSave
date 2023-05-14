// Import required dependencies.
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Import from the project
use cosmwasm_std::{Addr, Storage, Uint128};
use cosmwasm_storage::{
    singleton, singleton_read, ReadonlySingleton, Singleton, bucket_read, 
    bucket, ReadonlyBucket, Bucket
};


//              The storage for the contract
//

// Define static constants to use as keys in the storage.
pub static CONFIG_KEY: &[u8] = b"config";
pub static DEPOSITORS_KEY: &[u8] = b"depositors";
pub static TOTAL_DEPOSIT_KEY: &[u8] = b"total_deposit";
pub static TALLY_KEY: &[u8] = b"tally";
pub static REWARD_KEY: &[u8] = b"reward";
pub static TEST_KEY: &[u8] = b"test";
pub static STRATEGY_KEY: &[u8] = b"reward";

// Define a struct to hold the smart contract's configuration data.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,        // Address of the contract admin
    pub governance: Addr,   // Governance address
    pub token: String,      // Symbol of the staking token
    pub hash: String        // Hash of the token contract
}

// Define a struct to hold staking information for an address.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Depositors {
    pub amount: Uint128,    // Amount of tokens deposited
}

// Define a struct to hold information about a reward token.
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Reward {
    pub hash: String,               // Hash of the reward contract token
    pub path_to_silk: String,       // Path to the Silk in shade swap
    pub swap_minimum: Uint128,      // Minimum amount of tokens to swap for silk
    pub liquidity_ratio: i32,       // Minimum ratio of liquidity tokens to reward tokens
    pub unswapped_reward: Uint128,  // Amount of reward tokens that have not been swapped yet
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Strategy { 
    pub contract: String,
    pub hash: String,
    pub invoke_deposit: String,
    pub invoke_withdraw: String,
    pub invoke_claim_rewards: String,
    pub invoke_query_balance: String,  
    pub allocation: Uint128,
    pub allocation_ratio: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Balances {
    pub liabilities: Uint128,
    pub assets: Uint128,
    pub holdings: Uint128,
}


// This function returns a writable singleton instance of the `State` struct.
// It takes a mutable reference to a `Storage` trait object as an argument, which will be used to interact with the storage.
pub fn config_w(storage: &mut dyn Storage) -> Singleton<Config> {
    // This calls the `singleton` function from the `cosmwasm_std::storage` module with the provided storage and configuration key.
    // This creates or retrieves a singleton instance of the `State` struct from the storage.
    singleton(storage, CONFIG_KEY)
}

// This function returns a read-only singleton instance of the `State` struct.
// It takes an immutable reference to a `Storage` trait object as an argument, which will be used to interact with the storage.
pub fn config_r(storage: &dyn Storage) -> ReadonlySingleton<Config> {
    // This calls the `singleton_read` function from the `cosmwasm_std::storage` module with the provided storage and configuration key.
    // This retrieves a read-only singleton instance of the `State` struct from the storage.
    singleton_read(storage, CONFIG_KEY)
}

pub fn depositors_w(storage: &mut dyn Storage) -> Bucket<Depositors> {
    bucket(storage, DEPOSITORS_KEY)
}

pub fn depositors_r(storage: &dyn Storage) -> ReadonlyBucket<Depositors> {
   
    bucket_read(storage, DEPOSITORS_KEY)
}

pub fn balances_w(storage: &mut dyn Storage) -> Singleton<Balances> {
    singleton(storage, TOTAL_DEPOSIT_KEY)
}

pub fn balances_r(storage: &dyn Storage) -> ReadonlySingleton<Balances> {
    singleton_read(storage, TOTAL_DEPOSIT_KEY)
}

pub fn reward_w(storage: &mut dyn Storage) -> Bucket<Reward> {
    bucket(storage, REWARD_KEY)
}

pub fn reward_r(storage: &dyn Storage) -> ReadonlyBucket<Reward> {
    bucket_read(storage, REWARD_KEY)
}

pub fn strategy_w(storage: &mut dyn Storage) -> Bucket<Strategy> {
    bucket(storage, STRATEGY_KEY)
}

pub fn strategy_r(storage: &dyn Storage) -> ReadonlyBucket<Strategy> {
    bucket_read(storage, STRATEGY_KEY)
}







pub fn test_w(storage: &mut dyn Storage) -> Singleton<String> {
    singleton(storage, TEST_KEY)
}

pub fn test_r(storage: &dyn Storage) -> ReadonlySingleton<String> {
    singleton_read(storage, TEST_KEY)
}