// Import required dependencies.
use cosmwasm_std::{
    entry_point, to_binary, from_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, 
    StdResult, Uint128, StdError, Addr
};

// Import from the project
use crate::{
    msg::{InstantiateMsg, QueryMsg, ExecuteMsg, InvokeMsg,},
    state::{
        config_w, config_r, depositors_w, depositors_r, balances_w, balances_r,
        Config, test_r, test_w, Balances
    },
    operations::{
        deposit, try_register_reward_token, try_register_strategy, try_update_strategy
    },
    query::{query_balance, query_deposit, query_test, },
};


//                 the place where messages are processed and directed
//



// Define the entry point for contract instantiation
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    // Create a new state struct with the owner's address, token symbol, and hash
    let state = Config { 
        admin: info.sender.clone(),
        governance: msg.governance.clone(),
        token: msg.token.clone(), 
        hash: msg.hash.clone(),
    };
    // Set the initial balance to zero
    let total_deposit = Balances {
        liabilities: Uint128::zero(),
        assets: Uint128::zero(),
        holdings: Uint128::zero(),
    };
    
    // Save the state and balance to storage using singleton storage types
    config_w(deps.storage).save(&state)?;
    balances_w(deps.storage).save(&total_deposit)?;

    // Return an empty response indicating successful contract instantiation
    Ok(Response::default())
}

// Define the entry point for contract execution
#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Response> {
    match msg {
        // If we receive tokens, call the `try_receive` function
        ExecuteMsg::Receive { amount, from, msg } => try_receive(deps, info, amount, from, msg),
        // If we're registering a reward token, call the `try_register_reward_token` function
        ExecuteMsg::RegisterRewardToken{
            token, hash, path_to_silk, swap_minimum, liquidity_ratio,
        } => try_register_reward_token(
            deps, info, token, hash, path_to_silk, swap_minimum, liquidity_ratio,
        ),
        ExecuteMsg::RegisterStrategy{
            strategy_name, contract, hash, invoke_deposit, invoke_withdraw,
            invoke_claim_rewards, invoke_query_balance,
        } => try_register_strategy(
            deps, info, strategy_name, contract, hash, invoke_deposit, invoke_withdraw,
            invoke_claim_rewards, invoke_query_balance,
        ),
        ExecuteMsg::UpdateStrategy{
            strategy_name, contract, hash, invoke_deposit, invoke_withdraw,
            invoke_claim_rewards, invoke_query_balance, allocation, allocation_ratio,
        } => try_update_strategy(
            deps, info, strategy_name, contract, hash, invoke_deposit, invoke_withdraw,
            invoke_claim_rewards, invoke_query_balance, allocation, allocation_ratio,
        ),
    }
}


pub fn try_receive(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
    from: Addr,
    msg: Option<Binary>,
) -> StdResult<Response> {
    
    // Unwrap the `msg` into an optional binary, defaulting to an empty binary if it's not present
    let msg = msg.unwrap_or_default();
    
    // Convert the binary message to a string and save it to state
    let string_msg = String::from_utf8(msg.to_vec()).unwrap();
    test_w(deps.storage).save(&string_msg)?;

    // Deserialize the binary message into an `InvokeMsg` enum
    match from_binary(&msg)? {
        // If the message is `InvokeMsg::Deposit`, call the `deposit` function
        InvokeMsg::Deposit {} => deposit(deps, info, from, amount),
    }
}

// Define the entry point for queries
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetBalance {} => to_binary(&query_balance(deps)?), // if GetBalance query message is received, call query_balance function and encode its response in binary format
        QueryMsg::GetDeposit {address} => to_binary(&query_deposit(deps, address)?), // if GetDeposit query message is received, call query_deposit function with given address and encode its response in binary format
        QueryMsg::Test {} => to_binary(&query_test(deps)?), // if Test query message is received, call query_test function and encode its response in binary format
    }
}

