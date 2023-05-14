// Import necessary dependencies and modules
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Deps, StdResult, StdError, Addr, Uint128};

// Import from the project
use crate::{
    state::{
        config_r, depositors_r, balances_r, Balances,
        Config, test_r,
    },
};


//              the place for queries
//


// Define a struct to hold the query data
#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct BalanceResponse {
    pub balance: Balances,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct DepositResponse {
    pub address: Addr,
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct TestResponse {
    pub test: String,
}


// function to handle Test query message
pub fn query_test(deps: Deps) -> StdResult<TestResponse> {
    let test = test_r(deps.storage).load()?; // load test data from storage
    Ok(TestResponse { test: test }) // return TestResponse containing loaded test data
}

// function to handle GetBalance query message
pub fn query_balance(deps: Deps) -> StdResult<BalanceResponse> {
    let total_deposit = balances_r(deps.storage).load()?; // load balance data from storage
    Ok(BalanceResponse { balance: total_deposit }) // return BalanceResponse containing loaded balance data
}

// function to handle GetDeposit query message
pub fn query_deposit(
    deps: Deps,
    address: Addr,
) -> StdResult<DepositResponse> {
    match depositors_r(deps.storage).may_load(address.as_bytes())? { // load depositor info from storage for given address
        Some(deposit_info) => { // if depositor info is found
            let balance = deposit_info.amount; // extract deposited amount from depositor info
            Ok(DepositResponse { 
                address: address, // set depositor address in response
                balance: balance, // set deposited amount in response
            })
        }
        None => { // if depositor info is not found
            return Err(StdError::generic_err(
                "address is not a depositor".to_string(),
            )); // return generic error with message "address is not a depositor"
        }
    }
}
