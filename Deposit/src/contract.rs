#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::functions;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONFIG, Config};


const CONTRACT_NAME: &str = "crates.io:deposit";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config { owner: info.sender.clone() };
    if config.owner != info.sender{
        return Err(ContractError::InvalidOwner {  });
    }
    CONFIG.save(deps.storage, &config).unwrap();

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Deposits { amount, denom } => functions::execute_deposit(deps, info, amount, denom),
        ExecuteMsg::Withdraw { amount, denom } => functions::execute_withdraw(deps, info, amount, denom),
        ExecuteMsg::Cw20Deposits { owner, amount } => functions::execute_cw20_deposit(deps, info, owner, amount),  
        ExecuteMsg::Cw20Withdraws { contract, amount }=> functions::execute_cw20_withdraw(deps, info, contract, amount),

    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    _env: Env, 
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {  }=> to_binary(&functions::get_config(deps)?),
        QueryMsg::Deposits { address } => to_binary(&functions::query_deposit(deps, address)?),
        QueryMsg::Cw20Deposits { address } => to_binary(&functions::query_cw20_deposits(deps, address)?),
    }
}

