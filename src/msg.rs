// Import necessary dependencies and modules
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Import from the project
use cosmwasm_std::{Binary, Uint128, Addr};

//             the place where we recieve the messages
//


#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub governance: Addr,
    pub token: String,
    pub hash: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Receive {
        amount: Uint128,
        from: Addr,
        #[serde(default)]
        msg: Option<Binary>,
    },
    RegisterRewardToken {
        token: String,
        hash: String,
        path_to_silk: String,
        swap_minimum: Uint128,
        liquidity_ratio: i32,
    },
    RegisterStrategy {
        strategy_name: String,
        contract: String,
        hash: String,
        invoke_deposit: String,
        invoke_withdraw: String,
        invoke_claim_rewards: String,
        invoke_query_balance: String,
    },
    UpdateStrategy {
        strategy_name: String,
        contract: String,
        hash: String,
        invoke_deposit: String,
        invoke_withdraw: String,
        invoke_claim_rewards: String,
        invoke_query_balance: String,
        allocation: Uint128,
        allocation_ratio: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum InvokeMsg {
    Deposit {},
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetBalance {},
    GetDeposit {address: Addr},
    Test {},
}

