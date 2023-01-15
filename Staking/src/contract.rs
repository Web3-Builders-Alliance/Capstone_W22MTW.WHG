#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Validator};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::functions::token_info;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{TokenInfo, InvestmentInfo, INVESTMENT};


const CONTRACT_NAME: &str = "crates.io:staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    //check the validator
    let validator= deps.querier.query_validator(msg.validator.clone())?;
    if validator.is_none(){
        return Err(ContractError::NotInValidatorSet { validator: msg.validator });
    }

    let token = TokenInfo {
        name_token: msg.name_token,
        symbol_token: msg.symbol_token, 
        decimals: msg.decimals
    };

    token_info(deps.storage).save(&token)?;

    let denom = deps.querier.query_bonded_denom()?;
    let invest = InvestmentInfo{
        owner: info.sender,
        unbonding_period: msg.unbonding_period,
        bond_denom: denom,
        validator:msg.validator,
        emergancy_fee: msg.emergancy_fee,
    };

    INVESTMENT.save(deps.storage, &invest)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    unimplemented!()
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
