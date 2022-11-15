#[cfg(not(feature = "library"))]
//below are the annotations for the contract
use cosmwasm_std::entry_point; //this is the entry_point annotation imported from the comswasm_std library
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult}; //importing the required libraries from the cosmwasm_std library which
//will be using throughout the contract
use cw2::set_contract_version; 

use crate::error::ContractError;
use crate::msg::{CountResponse, ExecuteMsg, InstantiateMsg, QueryMsg}; //Importing various types created in msg.rs file and using the structs, enums here in the contract file
use crate::state::{State, STATE}; //Importing the state to store the information in the contract

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter-wba";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate( //instantiate method to initialize the contract with the initial state
    deps: DepsMut, //DepsMut is a mutable reference to the contract's storage and contains the important dependencies like contract, storage and other apis to query
    _env: Env, //Env is the environment of the contract which contains the information about the blockchain and the contract like address, block time and other information
    info: MessageInfo, //MessageInfo contains the information about the message sent by the user or token valye sent
    msg: InstantiateMsg, //this is the information contains in the msg.rs file which we need to initialize the data in the contract 
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count, 
        owner: info.sender.clone(), //getting from the caller info as specified above
        poll_count: 0,

    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?; //setting the contract version
    STATE.save(deps.storage, &state)?; //saving the state in the contract

    Ok(Response::new() //creating and retruning the response with the details like what method is being called, the caller and the count
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(////once after instantiating we can execute the code, this is the place where most of the business logic is written
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {//here we are executing match statement. Based on the type of ExecuteMsg, 
        //the following arm will be executed, for example if the ExecuteMessage contains the Increment, the first arm will be executed in this case 
        //which is making call to the try_increment method
        ExecuteMsg::Increment {} => try_increment(deps),
        ExecuteMsg::Decrement {} => try_decrement(deps),
        ExecuteMsg::Reset { count } => try_reset(deps, info, count),
    }
}

pub fn try_increment(deps: DepsMut) -> Result<Response, ContractError> {//this is the method to increment the count and update the state of the count in the contact 
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count += 1;//incrementing the count
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment")) //returning the response with the method name
}

pub fn try_decrement(deps: DepsMut) -> Result<Response, ContractError> {//this is the method to decrement the count and update the state of the count in the contact 
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.count -= 1;
        Ok(state)
    })?;

    Ok(Response::new().add_attribute("method", "try_increment"))
}


pub fn try_reset(deps: DepsMut, info: MessageInfo, count: i32) -> Result<Response, ContractError> { //this is the method to reset the count with the passed count value and update the state of the count in the contact
    STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
        if info.sender != state.owner {
            return Err(ContractError::Unauthorized {});
        }
        state.count = count;//resetting the count and updating the state
        Ok(state)
    })?;
    Ok(Response::new().add_attribute("method", "reset"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {//this is the method to query the count value from the contract
    match msg {
        QueryMsg::GetCount {} => to_binary(&query_count(deps)?),
    }
}

fn query_count(deps: Deps) -> StdResult<CountResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(CountResponse { count: state.count })
}
