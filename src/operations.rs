// Import necessary dependencies and modules
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, 
    Uint128, StdError, Addr, CosmosMsg, WasmMsg, SubMsg,
};
// Import from the project
use crate::state::{
    config_w, config_r, depositors_w, depositors_r, balances_r, balances_w,
    Config, Depositors, reward_w, Reward, Strategy, strategy_w, strategy_r
};

// Define the block size as a constant variable with value 256 for padding
pub const BLOCK_SIZE: usize = 256;


//            the place where the stuff happens



// The deposit function allows a user to deposit a specified amount of silk
pub fn deposit(
    deps: DepsMut,
    info: MessageInfo,
    from: Addr,
    amount: Uint128
) -> StdResult<Response> {

    // Load the config data from the storage.
    let config = config_r(deps.storage).load()?;

    // If the sender of the transaction is not the staking token, return an error.
    if config.token != info.sender {
        return Err(StdError::generic_err("Sender was not staking token".to_string()));
    }

    // Load the staking balance data from the storage and increment it by the `amount` argument.
    let mut balance = balances_r(deps.storage).load()?;
    balance.liabilities += amount;
    balance.holdings +=  amount;
    // Save the updated staking balance to the storage.
    balances_w(deps.storage).save(&balance)?;

    // Check if there is already a staking information for the `from` address.
    match depositors_r(deps.storage).may_load(from.as_bytes())? {
        // If there is already staking information for the address, update the `amount` field with the `amount` argument.
        Some(mut deposit_info) => {
            deposit_info.amount += amount;
            depositors_w(deps.storage).save(from.as_bytes(), &deposit_info)?;
        }
        // If there is no staking information for the address, save a new `Depositors` struct with the `amount` field set to the `amount` argument.
        None => {
            depositors_w(deps.storage).save(from.as_bytes(),
                &Depositors {
                    amount,
                },
            )?;
        }
    }

    

    // Return a default response if everything executed successfully.
    Ok(Response::default())
}


/// This function registers a new reward token in the contract storage.
/// It takes in the token details such as token name, hash, path to silk,
/// swap minimum, and liquidity ratio as inputs.
pub fn try_register_reward_token(
    deps: DepsMut,              // DepsMut is a struct that provides access to contract's dependencies in mutable form.
    info: MessageInfo,          // MessageInfo is a struct that contains information about the incoming message.
    token: String,              // The contract of the reward token to be registered.
    hash: String,               // The hash of the reward token contract.
    path_to_silk: String,       // The path to silk for the reward token.
    swap_minimum: Uint128,      // The minimum amount of tokens required for swapping.
    liquidity_ratio: i32,       // The liquidity ratio for the reward token.
) -> StdResult<Response> {

    // Create a new Reward struct with the given details and default value for unswapped_reward.
    let reward_token = Reward { 
        hash: hash,
        path_to_silk: path_to_silk,
        swap_minimum: swap_minimum,
        liquidity_ratio: liquidity_ratio,
        unswapped_reward: Uint128::zero(),
    };
    
    // Save the reward token to the contract storage using the reward_w object.
    // reward_w is a helper function that wraps the storage variable for rewards.
    reward_w(deps.storage).save(token.as_bytes(), &reward_token)?;

    // Return a default response indicating success.
    Ok(Response::default())
}

pub fn try_register_strategy(
    deps: DepsMut,              
    info: MessageInfo,
    strategy_name: String,
    contract: String,
    hash: String,
    invoke_deposit: String,
    invoke_withdraw: String,
    invoke_claim_rewards: String,
    invoke_query_balance: String,     
) -> StdResult<Response> {

    let strategy = Strategy { 
        contract: contract,
        hash: hash,
        invoke_deposit: invoke_deposit,
        invoke_withdraw: invoke_withdraw,
        invoke_claim_rewards: invoke_claim_rewards,
        invoke_query_balance: invoke_query_balance,  
        allocation: Uint128::zero(),
        allocation_ratio: Uint128::zero(),
    };
    
    strategy_w(deps.storage).save(strategy_name.as_bytes(), &strategy)?;

    Ok(Response::default())
}

pub fn try_update_strategy(
    deps: DepsMut,              
    info: MessageInfo,
    strategy_name: String,
    contract: String,
    hash: String,
    invoke_deposit: String,
    invoke_withdraw: String,
    invoke_claim_rewards: String,
    invoke_query_balance: String, 
    allocation: Uint128,
    allocation_ratio: Uint128,
) -> StdResult<Response> {

    match strategy_r(deps.storage).may_load(strategy_name.as_bytes())? {
        // If there is already staking information for the address, update the `amount` field with the `amount` argument.
        Some(mut strategy) => {

            strategy = Strategy { 
                contract: contract,
                hash: hash,
                invoke_deposit: invoke_deposit,
                invoke_withdraw: invoke_withdraw,
                invoke_claim_rewards: invoke_claim_rewards,
                invoke_query_balance: invoke_query_balance,  
                allocation: allocation,
                allocation_ratio: allocation_ratio,
            };
            
            strategy_w(deps.storage).save(strategy_name.as_bytes(), &strategy)?;
        
            Ok(Response::default())
        }
        // If there is no staking information for the address, save a new `Depositors` struct with the `amount` field set to the `amount` argument.
        None => {
            return Err(StdError::generic_err("There is no strategy with that name".to_string()));
        }
    }
}

pub fn try_claim_rewards(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    // Define the address of the recipient contract
    let recipient = String::from("secret1c9dfnjl6lnhjr5cqmtfnfqej87uspcltkg0d0l");
    // Define the hash of the recipient contract's code
    let code_hash = String::from("593c95b45c9d034148c4dcc02b858314a841e477cedbeb72d33b25caa0786af2");

    // Create the custom message
    let message = String::from("{\"deposit\":{}}");

    // Convert the message string to a binary format
    let message_binary = Binary::from(message.as_bytes());

    // Create and send the message to the recipient contract
    let wasm_msg = WasmMsg::Execute {
        contract_addr: recipient.to_string(),
        code_hash: code_hash.to_string(),
        msg: message_binary.clone(),
        funds: vec![],
    };

    // Return a new response containing the wasm message
    Ok(Response::new().add_message(wasm_msg))
}

