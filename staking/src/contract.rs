#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,to_binary, Uint128};
use cw2::set_contract_version;

use crate::error::ContractError;

use crate::functions::{self, token_info, invest_info, total_supply};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{TokenInfo, InvestmentInfo, CLAIMS, Supply};

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
        owner: info.sender.clone(),
        unbonding_period: msg.unbonding_period,
        bond_denom: denom.clone(),
        validator:msg.validator.clone(),
        emergancy_fee: msg.emergancy_fee,
    };
    invest_info(deps.storage).save(&invest)?;
    
    let supply = Supply { bonded: Uint128::new(0), claims: Uint128::new(0) };
    
    total_supply(deps.storage).save(&supply)?;
    
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("validator", msg.validator)
        .add_attribute("owner", info.sender)
        .add_attribute("bonded_token", denom)
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { amount, denom } => functions::transfer(deps, info, amount, denom),
        ExecuteMsg::Bond {  } => functions::bond(deps, env, info),
        ExecuteMsg::BondAllTokens {  } => functions::bond_all_tokens(deps, env, info),
        ExecuteMsg::Redelegate {  } => functions::redelegate(deps, env, info),
        ExecuteMsg::Claim {  } => functions::claim(deps, env, info),
        ExecuteMsg::Unbond { amount } => functions::unbond(deps, env, info, amount),
    }
}




#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    _env: Env,
    msg: QueryMsg
) -> StdResult<Binary> {
    match msg{
        QueryMsg::Balance { address } => to_binary(&functions::balance(deps, address)?),
        QueryMsg::Claims { address } => to_binary(&CLAIMS.query_claims(deps, &deps.api.addr_validate(&address)?)?),
        QueryMsg::Investment {  } => to_binary(&functions::query_investment(deps)?),
        QueryMsg::TokenInfo {  } => to_binary(&functions::query_token_info(deps)?),
    } 
    
}