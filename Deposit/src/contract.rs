#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, BankMsg};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::functions;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{CONFIG, Config, DEPOSITS, Deposits};


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
        ExecuteMsg::Withdraw { amount, denom } => unimplemented!(),
        ExecuteMsg::Cw20Deposits { owner, amount } => unimplemented!(),    }
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
